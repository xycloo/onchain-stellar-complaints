#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Address, Bytes, BytesN, Env};
use storage::{write_admin, DataKey};

mod storage;
mod events;


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 0,
    NotInitialized = 1,
    NoAdminAuth = 2,
    MessageAlreadyExists = 3
}

#[contract]
pub struct Feedback;

#[contractimpl]
impl Feedback {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if storage::has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }

        write_admin(&env, &admin);
        storage::bump_instance(&env);

        Ok(())
    }

    pub fn send(env: Env, from: Address, message: Bytes) -> Result<(), Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized)
        }

        from.require_auth();
        let hash = get_hash(&env, &message);

        if env.storage().temporary().has(&DataKey::Feedback(hash.clone())) {
            return Err(Error::MessageAlreadyExists)
        }

        storage::write_votes(&env, &hash, 1);
        events::new_feedback(&env, &from, hash, message);

        Ok(())
    }

    pub fn upvote(env: &Env, from: Address, hash: BytesN<32>) -> Result<(), Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized)
        }

        from.require_auth();
        storage::write_votes(&env, &hash, storage::read_votes(&env, &hash) + 1);
        events::upvote(&env, &from, hash);
        Ok(())
    }

    pub fn downvote(env: Env, from: Address, hash: BytesN<32>) -> Result<(), Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized)
        }
        
        from.require_auth();
        storage::write_votes(&env, &hash, storage::read_votes(&env, &hash) - 1);
        events::downvote(&env, &from, hash);

        Ok(())
    }

    pub fn update_binary(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized)
        }
        storage::read_admin(&env).require_auth();
        env.deployer().update_current_contract_wasm(hash);
        
        Ok(())
    }

    pub fn get_feedback(env: Env, hash: BytesN<32>) -> Result<i32, Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized)
        }
        
        Ok(storage::read_votes(&env, &hash))
    }
}

fn get_hash(env: &Env, text: &Bytes) -> BytesN<32> {
    env.crypto().sha256(text)
}

mod test;

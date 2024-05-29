use soroban_sdk::{contracttype, Address, BytesN, Env};


const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_LEDGER_LIFE: u32 = 30 * DAY_IN_LEDGERS; // ~30 days.
const INSTANCE_LEDGER_TTL_THRESHOLD: u32 = INSTANCE_LEDGER_LIFE - DAY_IN_LEDGERS;
const TEMPORARY_LEDGER_LIFE: u32 = 30 * DAY_IN_LEDGERS; // ~30 days.
const TEMPORARY_LEDGER_TTL_THRESHOLD: u32 = TEMPORARY_LEDGER_LIFE - DAY_IN_LEDGERS;

#[contracttype]
pub enum DataKey {
    Admin,
    Feedback(BytesN<32>)
}

fn bump_temporary(env: &Env, key: &DataKey) {
    env.storage().temporary().extend_ttl(key, TEMPORARY_LEDGER_TTL_THRESHOLD, TEMPORARY_LEDGER_LIFE)
}

pub(crate) fn bump_instance(env: &Env) {
    env.storage().instance().extend_ttl(INSTANCE_LEDGER_TTL_THRESHOLD, INSTANCE_LEDGER_LIFE)
}

pub(crate) fn read_admin(env: &Env) -> Address {
    // Note: should only be called once the exeuction
    // sorted out that the admin actually exists.
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub(crate) fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub(crate) fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub(crate) fn write_votes(env: &Env, hash: &BytesN<32>, votes: i32) {
    env.storage().temporary().set(&DataKey::Feedback(hash.clone()), &votes);
    bump_temporary(env, &DataKey::Feedback(hash.clone()));
}

pub(crate) fn read_votes(env: &Env, hash: &BytesN<32>) -> i32 {
    env.storage().temporary().get(&DataKey::Feedback(hash.clone())).unwrap()
}

#![cfg(test)]

extern crate std;
use super::*;
use soroban_sdk::{testutils::{Address as _, Events}, Env};

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn not_initialize() {
    let env = Env::default();
    env.mock_all_auths(); // note: this contract doens't currently rely on non-root auths.
    
    let contract_id = env.register_contract(None, Feedback);
    let client = FeedbackClient::new(&env, &contract_id);
    
    let message = Bytes::from_slice(&env, "I would like to start seeing mainnet soroban apps interconnected!".as_bytes());
    let user1 = Address::generate(&env);
    client.send(&user1, &message);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn feedback_double() {
    let env = Env::default();
    env.mock_all_auths(); // note: this contract doens't currently rely on non-root auths.
    
    let contract_id = env.register_contract(None, Feedback);
    let client = FeedbackClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let message = Bytes::from_slice(&env, "I would like to start seeing mainnet soroban apps interconnected!".as_bytes());
    let hash = env.crypto().sha256(&message);
    
    client.initialize(&admin);
    
    let user1 = Address::generate(&env);
    client.send(&user1, &message);

    let user2 = Address::generate(&env);
    client.send(&user2, &message);
    
    assert_eq!(client.get_feedback(&hash), 1);

    let all_events = env.events().all();
    assert_eq!(all_events.len(), 2);
}

#[test]
fn feedback_upvote() {
    let env = Env::default();
    env.mock_all_auths(); // note: this contract doens't currently rely on non-root auths.
    
    let contract_id = env.register_contract(None, Feedback);
    let client = FeedbackClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let message = Bytes::from_slice(&env, "I would like to start seeing mainnet soroban apps interconnected!".as_bytes());
    let hash = env.crypto().sha256(&message);
    
    client.initialize(&admin);
    
    let user1 = Address::generate(&env);
    client.send(&user1, &message);

    let user2 = Address::generate(&env);
    client.upvote(&user2, &hash);
    client.upvote(&user2, &hash);

    assert_eq!(client.get_feedback(&hash), 3);

    let all_events = env.events().all();
    assert_eq!(all_events.len(), 3);
}

#[test]
fn feedback_downvote() {
    let env = Env::default();
    env.mock_all_auths(); // note: this contract doens't currently rely on non-root auths.
    
    let contract_id = env.register_contract(None, Feedback);
    let client = FeedbackClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let message = Bytes::from_slice(&env, "I would like to start seeing mainnet soroban apps interconnected!".as_bytes());
    let hash = env.crypto().sha256(&message);
    
    client.initialize(&admin);
    
    let user1 = Address::generate(&env);
    client.send(&user1, &message);

    let user2 = Address::generate(&env);
    client.upvote(&user2, &hash);
    client.upvote(&user2, &hash);

    assert_eq!(client.get_feedback(&hash), 3);

    client.downvote(&user2, &hash);
    client.downvote(&user2, &hash);
    client.downvote(&user1, &hash);
    
    assert_eq!(client.get_feedback(&hash), 0);

    client.downvote(&user1, &hash);
    client.downvote(&user1, &hash);
    
    assert_eq!(client.get_feedback(&hash), -2);

    let all_events = env.events().all();
    assert_eq!(all_events.len(), 8);
}


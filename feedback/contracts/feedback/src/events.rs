use soroban_sdk::{Address, Bytes, Env, BytesN};


pub(crate) fn new_feedback(env: &Env, from: &Address, hash: BytesN<32>, text: Bytes) {
    env.events().publish(("feedback", from, hash), text)
}

pub(crate) fn upvote(env: &Env, from: &Address, hash: BytesN<32>) {
    env.events().publish(("upvote", from, hash), ())
}

pub(crate) fn downvote(env: &Env, from: &Address, hash: BytesN<32>) {
    env.events().publish(("downvote", from, hash), ())
}

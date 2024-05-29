use serde::{Deserialize, Serialize};
use zephyr_sdk::{
    prelude::*,
    soroban_sdk::xdr::ScVal,
    DatabaseDerive, EnvClient,
};

#[derive(DatabaseDerive, Clone)]
#[with_name("feedback")]
pub struct Feedback {
    pub source: ScVal,
    pub hash: ScVal,
    pub text: ScVal,
    pub votes: ScVal,
}

#[derive(DatabaseDerive, Clone)]
#[with_name("score")]
pub struct Vote {
    pub source: ScVal,
    pub upvote: bool,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct FeedbackHttp {
    pub from: String,
    pub hash: String,
    pub text: String,
    pub votes: i32,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct SimulateSend {
    pub from: String,
    pub sequence: i64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SimulateVote {
    pub from: String,
    pub sequence: i64,
    pub hash: String,
    pub upvote: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SimulationRequest {
    Send(SimulateSend),
    Vote(SimulateVote)
}


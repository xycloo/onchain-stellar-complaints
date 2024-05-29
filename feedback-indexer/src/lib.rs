use types::*;
use zephyr_sdk::{
    prelude::*,
    soroban_sdk::{vec, xdr::ScVal, Address, Bytes, BytesN, IntoVal, String as SString, Symbol},
    EnvClient,
};

mod types;

fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!())
}

const CONTRACT_ADDRESS: [u8; 32] = [170, 178, 66, 162, 68, 36, 204, 199, 241, 161, 43, 240, 181, 33, 7, 193, 248, 103, 27, 207, 224, 172, 100, 77, 95, 248, 77, 100, 104, 251, 134, 197];

#[no_mangle]
pub extern "C" fn on_close() {
    let env = EnvClient::new();

    let events = env.reader().pretty().soroban_events();
    env.log().debug(format!("got events {:?}", events.len()), None);
    for event in events {
        if event.contract == CONTRACT_ADDRESS {
            let topics = event.topics;

            if SString::from_str(&env.soroban(), "feedback") == env.from_scval(&topics[0]) {
                env.log().debug("feedback", None);
                env.put(&Feedback {
                    source: topics[1].clone(),
                    hash: topics[2].clone(),
                    text: event.data.clone(),
                    votes: env.to_scval(1_i32),
                });
            } else if SString::from_str(&env.soroban(), "upvote") == env.from_scval(&topics[0]) {
                env.put(&Vote {
                    source: topics[1].clone(),
                    upvote: true,
                });
                update_feedback(&env, topics[2].clone(), true)
            } else if SString::from_str(&env.soroban(), "downvote") == env.from_scval(&topics[0]) {
                update_feedback(&env, topics[2].clone(), false);
                env.put(&Vote {
                    source: topics[1].clone(),
                    upvote: true,
                });
            }
        }
    }
}

// Note: in the upcoming releases this think of flow will be improved:
// - ability to read with filters (much more efficient and no need to filter all the feedbacks).
// - directly accept soroban types into tables (no need for all these conversions).
fn update_feedback(env: &EnvClient, hash: ScVal, upvote: bool) {
    let entries = Feedback::read_to_rows(env);
    let feedback = entries.iter().find(|entry| entry.hash == hash);
    if let Some(feedback) = feedback {
        let mut feedback = feedback.clone();
        let mut votes: i32 = env.from_scval(&feedback.votes);
        if upvote {
            votes += 1
        } else {
            votes -= 1
        }

        feedback.votes = env.to_scval(votes);

        env.update()
            .column_equal_to_xdr("hash", &hash)
            .execute(&feedback);
    }
}

// note: this function does a potentially large iteration,
// thus we rely on raw XDR rather than the host env for efficiency.
#[no_mangle]
pub extern "C" fn feedbacks() {
    let env = EnvClient::empty();
    let feedbacks: Vec<FeedbackHttp> = Feedback::read_to_rows(&env)
        .iter()
        .map(|entry| {
            let ScVal::Address(address) = &entry.source else { panic!()};
            let ScVal::Bytes(hash) = &entry.hash else { panic!()};
            let ScVal::Bytes(text) = &entry.text else { panic!()};
            let ScVal::I32(votes) = entry.votes else { panic!()};

            FeedbackHttp {
                from: address.to_string(),
                hash: hex::encode(hash.0.as_slice()),
                text: String::from_utf8(text.to_vec()).unwrap_or("Invalid text".into()),
                votes,
            }
        })
        .collect();

    env.conclude(feedbacks)
}

#[no_mangle]
pub extern "C" fn simulate() {
    let env = EnvClient::empty();
    let request: SimulationRequest = env.read_request_body();

    let response = match request {
        SimulationRequest::Send(SimulateSend { from, sequence, message }) => {
            let address = Address::from_string(&SString::from_str(&env.soroban(), &from));
            let message = Bytes::from_slice(&env.soroban(), message.as_bytes());
            env.simulate_contract_call_to_tx(
                from,
                sequence,
                CONTRACT_ADDRESS,
                Symbol::new(&env.soroban(), "send"),
                vec![
                    &env.soroban(),
                    address.into_val(env.soroban()),
                    message.into_val(env.soroban()),
                ],
            )
            
        },
        SimulationRequest::Vote(SimulateVote { from, sequence, hash, upvote }) => {
            let address = Address::from_string(&SString::from_str(&env.soroban(), &from));
            let hash = BytesN::<32>::from_array(&env.soroban(), &to_array::<u8, 32>(hex::decode(hash).unwrap()));
            let action = if upvote {
                "upvote"
            } else {
                "downvote"
            };

            env.simulate_contract_call_to_tx(
                from,
                sequence,
                CONTRACT_ADDRESS,
                Symbol::new(&env.soroban(), action),
                vec![
                    &env.soroban(),
                    address.into_val(env.soroban()),
                    hash.into_val(env.soroban())
                ],
            )
        }
    }
    .unwrap();

    env.conclude(response)
}

// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Session;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Session = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "accessTokenExpirationTimestampMs")]
    access_token_expiration_timestamp_ms: i64,
    #[serde(rename = "clientId")]
    client_id: String,
}

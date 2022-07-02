use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    op: i64,
    d: Option<serde_json::Value>,
    s: Option<i64>,
    t: Option<i64>,
}

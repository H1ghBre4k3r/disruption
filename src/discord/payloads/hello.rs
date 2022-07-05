use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloPayloadData {
    pub heartbeat_interval: u128,
}

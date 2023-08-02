use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HelloPayloadData {
    pub heartbeat_interval: u128,
}

use serde::{Deserialize, Serialize};

use super::MessageApiType;

/// <https://discord.com/developers/docs/resources/message#message-snapshot-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageSnapshotApiType {
    /// minimal subset of fields in the forwarded message
    pub message: MessageApiType,
}

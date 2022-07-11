use serde_repr::{Deserialize_repr, Serialize_repr};

/// ? https://discord.com/developers/docs/interactions/message-components#component-object
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum MessageComponentApiType {
    ActionRow = 1,
    Button = 2,
    SelectMenu = 3,
    TextInput = 4,
}

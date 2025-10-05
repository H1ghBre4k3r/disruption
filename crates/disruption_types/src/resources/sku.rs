use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/sku#sku-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SKUApiType {
    /// ID of SKU
    pub id: String,
    /// Type of SKU
    #[serde(rename = "type")]
    pub type_: SKUType,
    /// ID of the parent application
    pub application_id: String,
    /// Customer-facing name of your premium offering
    pub name: String,
    /// System-generated URL slug based on the SKU's name
    pub slug: String,
    /// SKU flags combined as a bitfield
    pub flags: u32,
}

/// <https://discord.com/developers/docs/resources/sku#sku-object-sku-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SKUType {
    /// Durable one-time purchase
    DURABLE = 2,
    /// Consumable one-time purchase
    CONSUMABLE = 3,
    /// Represents a recurring subscription
    SUBSCRIPTION = 5,
    /// System-generated group for each SUBSCRIPTION SKU created
    SUBSCRIPTION_GROUP = 6,
}

/// <https://discord.com/developers/docs/resources/sku#sku-object-sku-flags>
#[allow(non_camel_case_types)]
pub enum SKUFlags {
    /// SKU is available for purchase
    AVAILABLE = 1 << 2,
    /// Recurring SKU that can be purchased by a user and applied to a single server
    GUILD_SUBSCRIPTION = 1 << 7,
    /// Recurring SKU purchased by a user for themselves
    USER_SUBSCRIPTION = 1 << 8,
}

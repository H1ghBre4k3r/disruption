use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/entitlement#entitlement-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntitlementApiType {
    /// ID of the entitlement
    pub id: String,
    /// ID of the SKU
    pub sku_id: String,
    /// ID of the parent application
    pub application_id: String,
    /// ID of the user that is granted access to the entitlement's sku
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Type of entitlement
    #[serde(rename = "type")]
    pub type_: EntitlementType,
    /// Entitlement was deleted
    pub deleted: bool,
    /// Start date at which the entitlement is valid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Date at which the entitlement is no longer valid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// ID of the guild that is granted access to the entitlement's sku
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// For consumable items, whether or not the entitlement has been consumed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<bool>,
}

/// <https://discord.com/developers/docs/resources/entitlement#entitlement-object-entitlement-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum EntitlementType {
    /// Entitlement was purchased by user
    PURCHASE = 1,
    /// Entitlement for Discord Nitro subscription
    PREMIUM_SUBSCRIPTION = 2,
    /// Entitlement was gifted by developer
    DEVELOPER_GIFT = 3,
    /// Entitlement was purchased by a dev in application test mode
    TEST_MODE_PURCHASE = 4,
    /// Entitlement was granted when the SKU was free
    FREE_PURCHASE = 5,
    /// Entitlement was gifted by another user
    USER_GIFT = 6,
    /// Entitlement was claimed by user for free as a Nitro Subscriber
    PREMIUM_PURCHASE = 7,
    /// Entitlement was purchased as an app subscription
    APPLICATION_SUBSCRIPTION = 8,
}

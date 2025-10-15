use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/subscription#subscription-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionApiType {
    /// ID of the subscription
    pub id: String,
    /// ID of the user who is subscribed
    pub user_id: String,
    /// List of SKUs subscribed to
    pub sku_ids: Vec<String>,
    /// List of entitlements granted for this subscription
    pub entitlement_ids: Vec<String>,
    /// List of SKUs that this user will be subscribed to at renewal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_sku_ids: Option<Vec<String>>,
    /// Start of the current subscription period
    pub current_period_start: String,
    /// End of the current subscription period
    pub current_period_end: String,
    /// Current status of the subscription
    pub status: SubscriptionStatus,
    /// When the subscription was canceled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    /// ISO3166-1 alpha-2 country code of the payment source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// <https://discord.com/developers/docs/resources/subscription#subscription-object-subscription-statuses>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SubscriptionStatus {
    /// Subscription is active and scheduled to renew
    ACTIVE = 0,
    /// Subscription is active but will not renew
    ENDING = 1,
    /// Subscription is inactive and not being charged
    INACTIVE = 2,
}

/// Tests for User-related types serialization and deserialization
mod common;

use common::*;
use disruption_types::entities::UserApiType;

#[test]
fn test_basic_user_deserialization() {
    let json = load_fixture("users", "basic_user.json");
    let user: UserApiType = serde_json::from_str(&json).expect("Failed to deserialize basic user");

    assert_eq!(user.id, "80351110224678912");
    assert_eq!(user.username, "Nelly");
    assert_eq!(user.discriminator, "1337");
    assert_eq!(
        user.avatar,
        Some("8342729096ea3675442027381ff50dfe".to_string())
    );
    assert_eq!(user.bot, Some(false));
}

#[test]
fn test_full_user_deserialization() {
    let json = load_fixture("users", "full_user.json");
    let user: UserApiType = serde_json::from_str(&json).expect("Failed to deserialize full user");

    assert_eq!(user.id, "80351110224678912");
    assert_eq!(user.username, "Nelly");
    assert_eq!(user.global_name, Some("Nelly".to_string()));
    assert_eq!(user.mfa_enabled, Some(true));
    assert_eq!(user.verified, Some(true));
    assert_eq!(user.email, Some("nelly@example.com".to_string()));
    assert_eq!(user.premium_type, Some(2));
}

#[test]
fn test_bot_user_deserialization() {
    let json = load_fixture("users", "bot_user.json");
    let user: UserApiType = serde_json::from_str(&json).expect("Failed to deserialize bot user");

    assert_eq!(user.id, "123456789012345678");
    assert_eq!(user.username, "TestBot");
    assert_eq!(user.bot, Some(true));
    assert_eq!(user.global_name, Some("Test Bot".to_string()));
}

#[test]
fn test_user_serialization_roundtrip() {
    let user = UserApiType {
        id: "123456789".to_string(),
        username: "test_user".to_string(),
        discriminator: "1234".to_string(),
        global_name: Some("Test User".to_string()),
        avatar: Some("avatar_hash".to_string()),
        bot: Some(false),
        system: Some(false),
        mfa_enabled: Some(false),
        banner: None,
        accent_color: None,
        locale: Some("en-US".to_string()),
        verified: Some(true),
        email: None,
        flags: Some(0),
        premium_type: Some(0),
        public_flags: Some(0),
        avatar_decoration_data: None,
        collectibles: None,
        primary_guild: None,
    };

    // Serialize and deserialize to ensure roundtrip works
    let json = serde_json::to_string(&user).expect("Failed to serialize");
    let deserialized: UserApiType = serde_json::from_str(&json).expect("Failed to deserialize");

    // Verify key fields match
    assert_eq!(user.id, deserialized.id);
    assert_eq!(user.username, deserialized.username);
    assert_eq!(user.global_name, deserialized.global_name);
}

#[test]
fn test_user_with_minimal_fields() {
    let json = r#"{
        "id": "123",
        "username": "test",
        "discriminator": "0001",
        "avatar": null
    }"#;

    let user: UserApiType = serde_json::from_str(json).expect("Failed to deserialize minimal user");

    assert_eq!(user.id, "123");
    assert_eq!(user.username, "test");
    assert_eq!(user.discriminator, "0001");
    assert_eq!(user.avatar, None);
}

#[test]
fn test_user_default() {
    let user = UserApiType::default();
    assert_eq!(user.id, "");
    assert_eq!(user.username, "");
    assert_eq!(user.discriminator, "");
}

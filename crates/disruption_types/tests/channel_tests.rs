/// Tests for Channel-related types serialization and deserialization
mod common;

use common::*;
use disruption_types::channel::{ChannelApiType, ChannelTypeApiType};

#[test]
fn test_text_channel_deserialization() {
    let json = load_fixture("channels", "text_channel.json");
    let channel: ChannelApiType =
        serde_json::from_str(&json).expect("Failed to deserialize text channel");

    assert_eq!(channel.id, "41771983423143937");
    assert_eq!(channel.name, Some("general".to_string()));
    assert_eq!(channel.type_, ChannelTypeApiType::GUILD_TEXT);
    assert_eq!(channel.nsfw, Some(false));
    assert_eq!(
        channel.topic,
        Some("24/7 chat about how to gank Mike #2".to_string())
    );
}

#[test]
fn test_voice_channel_deserialization() {
    let json = load_fixture("channels", "voice_channel.json");
    let channel: ChannelApiType =
        serde_json::from_str(&json).expect("Failed to deserialize voice channel");

    assert_eq!(channel.id, "155101607195836416");
    assert_eq!(channel.name, Some("ROCKET CHEESE".to_string()));
    assert_eq!(channel.type_, ChannelTypeApiType::GUILD_VOICE);
    assert_eq!(channel.bitrate, Some(64000));
    assert_eq!(channel.user_limit, Some(0));
}

#[test]
fn test_dm_channel_deserialization() {
    let json = load_fixture("channels", "dm_channel.json");
    let channel: ChannelApiType =
        serde_json::from_str(&json).expect("Failed to deserialize DM channel");

    assert_eq!(channel.id, "319674150115610528");
    assert_eq!(channel.type_, ChannelTypeApiType::DM);
    assert!(channel.recipients.is_some());

    let recipients = channel.recipients.unwrap();
    assert_eq!(recipients.len(), 1);
    assert_eq!(recipients[0].id, "82198898841029460");
}

#[test]
fn test_channel_serialization_roundtrip() {
    let json = load_fixture("channels", "text_channel.json");
    let channel: ChannelApiType = serde_json::from_str(&json).expect("Failed to deserialize");
    let reserialized = serde_json::to_string(&channel).expect("Failed to serialize");
    let reparsed: ChannelApiType =
        serde_json::from_str(&reserialized).expect("Failed to re-deserialize");

    // Verify key fields are preserved
    assert_eq!(channel.id, reparsed.id);
    assert_eq!(channel.type_, reparsed.type_);
    assert_eq!(channel.name, reparsed.name);
}

#[test]
fn test_voice_channel_serialization_roundtrip() {
    let json = load_fixture("channels", "voice_channel.json");
    let channel: ChannelApiType = serde_json::from_str(&json).expect("Failed to deserialize");
    let reserialized = serde_json::to_string(&channel).expect("Failed to serialize");
    let reparsed: ChannelApiType =
        serde_json::from_str(&reserialized).expect("Failed to re-deserialize");

    // Verify key fields are preserved
    assert_eq!(channel.id, reparsed.id);
    assert_eq!(channel.type_, reparsed.type_);
    assert_eq!(channel.bitrate, reparsed.bitrate);
}

#[test]
fn test_channel_types() {
    // Test various channel types can be deserialized
    let types_to_test = vec![
        (0, ChannelTypeApiType::GUILD_TEXT, "TEXT"),
        (1, ChannelTypeApiType::DM, "DM"),
        (2, ChannelTypeApiType::GUILD_VOICE, "VOICE"),
        (4, ChannelTypeApiType::GUILD_CATEGORY, "CATEGORY"),
        (5, ChannelTypeApiType::GUILD_NEWS, "NEWS"),
        (13, ChannelTypeApiType::GUILD_STAGE_VOICE, "STAGE_VOICE"),
        (15, ChannelTypeApiType::GUILD_FORUM, "FORUM"),
        (16, ChannelTypeApiType::GUILD_MEDIA, "MEDIA"),
    ];

    for (type_num, expected_type, type_name) in types_to_test {
        let json = format!(r#"{{"id": "123", "type": {}}}"#, type_num);
        let channel: ChannelApiType = serde_json::from_str(&json)
            .expect(&format!("Failed to deserialize {} channel", type_name));
        assert_eq!(channel.type_, expected_type);
    }
}

/// Tests for Message-related types serialization and deserialization
mod common;

use common::*;
use disruption_types::channel::{MessageApiType, MessageTypeApiType};

#[test]
fn test_basic_message_deserialization() {
    let json = load_fixture("messages", "basic_message.json");
    let message: MessageApiType =
        serde_json::from_str(&json).expect("Failed to deserialize basic message");

    assert_eq!(message.id, "162701077035089920");
    assert_eq!(message.channel_id, "131391742183342080");
    assert_eq!(message.content, "Hello, world!");
    assert_eq!(message.author.id, "140564834364416000");
    assert_eq!(message.author.username, "test");
    assert!(!message.tts);
    assert!(!message.mention_everyone);
    assert!(message.mentions.is_empty());
    assert!(message.embeds.is_empty());
}

#[test]
fn test_message_with_embed_deserialization() {
    let json = load_fixture("messages", "message_with_embed.json");
    let message: MessageApiType =
        serde_json::from_str(&json).expect("Failed to deserialize message with embed");

    assert_eq!(message.id, "222222222222222222");
    assert_eq!(message.content, "Check this out!");
    assert_eq!(message.embeds.len(), 1);

    let embed = &message.embeds[0];
    assert_eq!(embed.title, Some("Embed Title".to_string()));
    assert_eq!(
        embed.description,
        Some("This is an embed description".to_string())
    );
    assert_eq!(embed.color, Some(3066993));

    let fields = embed.fields.as_ref().unwrap();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "Field 1");
    assert_eq!(fields[0].value, "Value 1");
    assert_eq!(fields[0].inline, Some(false));
}

#[test]
fn test_message_serialization_roundtrip() {
    let json = load_fixture("messages", "basic_message.json");
    let message: MessageApiType = serde_json::from_str(&json).expect("Failed to deserialize");
    let reserialized = serde_json::to_string(&message).expect("Failed to serialize");
    let reparsed: MessageApiType =
        serde_json::from_str(&reserialized).expect("Failed to re-deserialize");

    // Verify key fields are preserved
    assert_eq!(message.id, reparsed.id);
    assert_eq!(message.content, reparsed.content);
    assert_eq!(message.author.id, reparsed.author.id);
}

#[test]
fn test_message_with_embed_roundtrip() {
    let json = load_fixture("messages", "message_with_embed.json");
    let message: MessageApiType = serde_json::from_str(&json).expect("Failed to deserialize");
    let reserialized = serde_json::to_string(&message).expect("Failed to serialize");
    let reparsed: MessageApiType =
        serde_json::from_str(&reserialized).expect("Failed to re-deserialize");

    // Verify key fields and embeds are preserved
    assert_eq!(message.id, reparsed.id);
    assert_eq!(message.embeds.len(), reparsed.embeds.len());
    assert_eq!(message.embeds[0].title, reparsed.embeds[0].title);
}

#[test]
fn test_message_types() {
    // Test various message types
    let message_types = vec![
        (0, MessageTypeApiType::DEFAULT, "DEFAULT"),
        (7, MessageTypeApiType::USER_JOIN, "USER_JOIN"),
        (19, MessageTypeApiType::REPLY, "REPLY"),
        (
            20,
            MessageTypeApiType::CHAT_INPUT_COMMAND,
            "CHAT_INPUT_COMMAND",
        ),
    ];

    for (type_num, expected_type, type_name) in message_types {
        let json = format!(
            r#"{{
                "id": "123",
                "channel_id": "456",
                "author": {{
                    "id": "789",
                    "username": "test",
                    "discriminator": "0001",
                    "avatar": null
                }},
                "content": "test",
                "timestamp": "2020-01-01T00:00:00.000000+00:00",
                "edited_timestamp": null,
                "tts": false,
                "mention_everyone": false,
                "mentions": [],
                "mention_roles": [],
                "attachments": [],
                "embeds": [],
                "pinned": false,
                "type": {}
            }}"#,
            type_num
        );

        let message: MessageApiType = serde_json::from_str(&json)
            .expect(&format!("Failed to deserialize {} message", type_name));
        assert_eq!(message.type_, expected_type);
    }
}

#[test]
fn test_message_mentions() {
    let json = r#"{
        "id": "123",
        "channel_id": "456",
        "author": {
            "id": "789",
            "username": "test",
            "discriminator": "0001",
            "avatar": null
        },
        "content": "<@140564834364416000> hello!",
        "timestamp": "2020-01-01T00:00:00.000000+00:00",
        "edited_timestamp": null,
        "tts": false,
        "mention_everyone": false,
        "mentions": [
            {
                "id": "140564834364416000",
                "username": "mentioned_user",
                "discriminator": "1234",
                "avatar": "abc123"
            }
        ],
        "mention_roles": ["987654321"],
        "attachments": [],
        "embeds": [],
        "pinned": false,
        "type": 0
    }"#;

    let message: MessageApiType =
        serde_json::from_str(json).expect("Failed to deserialize message with mentions");

    assert_eq!(message.mentions.len(), 1);
    assert_eq!(message.mentions[0].id, "140564834364416000");
    assert_eq!(message.mention_roles.len(), 1);
    assert_eq!(message.mention_roles[0], "987654321");
}

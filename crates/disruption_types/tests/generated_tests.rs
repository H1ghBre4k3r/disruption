use disruption_types::generated::gateway::{
    ButtonComponent, ButtonStyle, Component, GatewayEvent, GatewayIdentify, GatewayOpcode,
    GatewayPayload, GatewaySectionAccessory, Interaction, SectionComponent,
};
use disruption_types::generated::rest::{ApplicationCommandPatchRequestPartial, UserResponse};

#[test]
fn generated_gateway_payload_accepts_current_opcode_and_event_values() {
    let payload: GatewayPayload =
        serde_json::from_str(r#"{"op":43,"d":{"guild_id":"1"},"s":null,"t":"CHANNEL_INFO"}"#)
            .expect("valid gateway payload");

    assert_eq!(payload.op, GatewayOpcode::RequestChannelInfo);
    assert_eq!(payload.t, Some(GatewayEvent::ChannelInfo));
}

#[test]
fn generated_gateway_opcode_values_match_discord() {
    assert_eq!(
        serde_json::to_value(GatewayOpcode::RequestSoundboardSounds)
            .expect("soundboard opcode serializes"),
        31
    );
    assert_eq!(
        serde_json::to_value(GatewayOpcode::RequestChannelInfo)
            .expect("channel info opcode serializes"),
        43
    );
}

#[test]
fn generated_numeric_enums_preserve_unknown_values() {
    let opcode: GatewayOpcode = serde_json::from_str("999").expect("unknown opcode is accepted");

    assert_eq!(
        serde_json::to_value(opcode).expect("opcode serializes"),
        999
    );
}

#[test]
fn generated_string_enums_have_an_unknown_fallback() {
    let event: GatewayEvent =
        serde_json::from_str(r#""FUTURE_DISCORD_EVENT""#).expect("unknown events are accepted");

    assert_eq!(event, GatewayEvent::Unknown);
}

#[test]
fn generated_components_include_new_layout_shapes() {
    let section: SectionComponent = serde_json::from_str(
        r#"{
          "type": 9,
          "components": [{"type": 10, "content": "hello"}],
          "accessory": {"type": 2, "style": 1}
        }"#,
    )
    .expect("valid section component");

    assert_eq!(section.components.len(), 1);
    match section.accessory {
        GatewaySectionAccessory::ButtonComponent(button) => {
            assert_eq!(button.style, ButtonStyle::Primary)
        }
        _ => panic!("section accessory was parsed as the wrong component"),
    }
}

#[test]
fn generated_identify_uses_self_contained_gateway_models() {
    let identify: GatewayIdentify = serde_json::from_str(
        r#"{
          "token": "token",
          "properties": {
            "os": "linux",
            "browser": "disco",
            "device": "disco"
          },
          "presence": {
            "since": null,
            "activities": [{"name": "Cards", "type": 0}],
            "status": "online",
            "afk": false
          },
          "intents": 5
        }"#,
    )
    .expect("valid identify payload");

    assert_eq!(identify.properties.os, "linux");
    assert_eq!(identify.intents, 5);
    assert_eq!(
        identify.presence.expect("presence is present").activities[0].name,
        "Cards"
    );
}

#[test]
fn generated_component_dispatches_by_type() {
    let component: Component =
        serde_json::from_str(r#"{"type":2,"style":1}"#).expect("valid button component");

    match component {
        Component::ButtonComponent(button) => assert_eq!(button.style, ButtonStyle::Primary),
        Component::ActionRowComponent(_) => panic!("button was parsed as an action row"),
        _ => panic!("button was parsed as the wrong component variant"),
    }

    let _: ButtonComponent = serde_json::from_str(r#"{"type":2,"style":1}"#)
        .expect("button component remains directly deserializable");
}

#[test]
fn generated_auto_populated_selects_do_not_require_options() {
    let user: Component = serde_json::from_str(r#"{"type":5,"custom_id":"user_select"}"#)
        .expect("user select without options is valid");
    let role: Component = serde_json::from_str(r#"{"type":6,"custom_id":"role_select"}"#)
        .expect("role select without options is valid");
    let mentionable: Component =
        serde_json::from_str(r#"{"type":7,"custom_id":"mentionable_select"}"#)
            .expect("mentionable select without options is valid");
    let channel: Component = serde_json::from_str(r#"{"type":8,"custom_id":"channel_select"}"#)
        .expect("channel select without options is valid");

    assert!(matches!(user, Component::UserSelectComponent(_)));
    assert!(matches!(role, Component::RoleSelectComponent(_)));
    assert!(matches!(
        mentionable,
        Component::MentionableSelectComponent(_)
    ));
    assert!(matches!(channel, Component::ChannelSelectComponent(_)));
}

#[test]
fn generated_interaction_contains_current_wire_fields() {
    let interaction: Interaction = serde_json::from_str(
        r#"{
          "id": "1",
          "application_id": "2",
          "type": 2,
          "token": "token",
          "version": 1,
          "app_permissions": "8",
          "attachment_size_limit": 104857600
        }"#,
    )
    .expect("valid interaction");

    assert_eq!(interaction.attachment_size_limit, Some(104857600));
    assert_eq!(interaction.app_permissions, Some("8".to_owned()));
}

#[test]
fn generated_rest_models_deserialize_official_shape() {
    let user: UserResponse = serde_json::from_str(
        r#"{
          "id": "123",
          "username": "discord",
          "discriminator": "0001",
          "global_name": "Discord",
          "avatar": null,
          "public_flags": 0,
          "flags": 0,
          "primary_guild": null
        }"#,
    )
    .expect("valid Discord user response");

    assert_eq!(user.username, "discord");
    assert_eq!(user.id, "123");
}

#[test]
fn generated_patch_requests_omit_absent_fields() {
    let patch = ApplicationCommandPatchRequestPartial {
        contexts: None,
        default_member_permissions: None,
        description: None,
        description_localizations: None,
        dm_permission: None,
        handler: None,
        integration_types: None,
        name: None,
        name_localizations: None,
        options: None,
    };

    let serialized = serde_json::to_value(patch)
        .expect("patch serializes")
        .as_object()
        .cloned()
        .expect("patch is an object");

    assert!(!serialized.contains_key("name"));
    assert_eq!(
        serialized.get("description"),
        Some(&serde_json::Value::Null)
    );
    assert_eq!(serialized.get("handler"), Some(&serde_json::Value::Null));
}

use disruption_types::generated::gateway::{
    GatewayEvent, GatewayOpcode, GatewayPayload, Interaction, SectionComponent,
};
use disruption_types::generated::rest::UserResponse;

#[test]
fn generated_gateway_payload_accepts_current_opcode_and_event_values() {
    let payload: GatewayPayload =
        serde_json::from_str(r#"{"op":31,"d":{"guild_id":"1"},"s":null,"t":"CHANNEL_INFO"}"#)
            .expect("valid gateway payload");

    assert_eq!(payload.op, GatewayOpcode::Value31);
    assert_eq!(payload.t, Some(GatewayEvent::ChannelInfo));
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
    assert_eq!(section.accessory["type"], 2);
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

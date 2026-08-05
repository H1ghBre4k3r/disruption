use disruption_types::generated::gateway::{
    GatewayEvent, GatewayOpcode, GatewayPayload, Interaction,
};

#[test]
fn generated_gateway_payload_accepts_current_opcode_and_event_values() {
    let payload: GatewayPayload =
        serde_json::from_str(r#"{"op":31,"d":{"guild_id":"1"},"s":null,"t":"CHANNEL_INFO"}"#)
            .expect("valid gateway payload");

    assert_eq!(payload.op, GatewayOpcode::Value31);
    assert_eq!(payload.t, Some(GatewayEvent::ChannelInfo));
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

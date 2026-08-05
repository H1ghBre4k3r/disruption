# discord-codegen

This tool converts an explicitly supplied Discord OpenAPI document into
serde-compatible Rust types.

    cargo run -p discord-codegen -- generate \
      --input schema/discord-api-spec/openapi.json \
      --output crates/disruption_types/src/generated/rest.rs

The input is deliberately explicit: normal builds do not access the network.
Update schema/discord-api-spec.lock.json when changing the upstream spec,
then regenerate and review the generated diff.

The Discord docs contain Gateway, interaction, component, and webhook-event
surfaces that are not represented by the REST OpenAPI document. Those inputs
live in schema/discord-extensions.json and are generated separately:

    cargo run -p discord-codegen -- generate \
      --input schema/discord-extensions.json \
      --output crates/disruption_types/src/generated/gateway.rs

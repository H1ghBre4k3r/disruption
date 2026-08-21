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

The extension schema is a self-contained Gateway model surface. It does not
import types from the handwritten payload or entity modules. Concrete nested
objects are named schemas and referenced with local \`$ref\`s. The following
schema extensions are supported:

- \`x-rust-type\` selects a local Rust primitive such as \`u8\` or \`u64\`.
- \`x-enum-names\` supplies Rust variant names for numeric enum values.
- \`x-dynamic: true\` explicitly keeps an object-shaped field as
  \`serde_json::Value\` when its wire shape is polymorphic or dictionary-like.

Gateway fields are checked in against Discord's official
[Gateway event documentation](https://docs.discord.com/developers/events/gateway-events)
and [component reference](https://docs.discord.com/developers/components/reference).
The schema is local and generation never accesses the network.

# discord-codegen

This tool converts an explicitly supplied Discord OpenAPI document into
serde-compatible Rust types.

    cargo run -p discord-codegen -- generate \
      --input /path/to/openapi.json \
      --output crates/disruption_types/src/generated/rest.rs

The input is deliberately explicit: normal builds do not access the network.
Update `schema/discord-api-spec.lock.json` when changing the upstream spec,
then regenerate and review the generated diff.

use axum::{extract::State, response::IntoResponse};
use serde_json::json;
use xenia_kit::{health::Health, manifest::Result, Schemas};

const SOURCE_SHA_TAG: &str = concat!("<cartridge-source-sha:", env!("CARTRIDGE_SOURCE_SHA"), ">");

pub fn source_sha() -> &'static str {
    &SOURCE_SHA_TAG["<cartridge-source-sha:".len()..SOURCE_SHA_TAG.len() - 1]
}

pub fn state(schemas: &Schemas, service: &str) -> Result<Health> {
    Health::from_value(
        schemas,
        json!({
            "schema": schemas.health.id(),
            "service": service,
            "source_sha": source_sha(),
            "ok": true
        }),
    )
}

pub async fn get(State(health): State<Health>) -> impl IntoResponse {
    health.response()
}

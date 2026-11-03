mod events;
mod fragment;
mod health;

use axum::{
    http::{header, HeaderValue},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde_json::Value;
use xenia_kit::transport::Transport;
use xenia_kit::{manifest::Result, theme, Manifest, Schemas};

const MANIFEST_JSON: &str = include_str!("../cartridge.json");
const PACK_CSS: &str = include_str!("../static/pack.css");

async fn pack_css() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/css; charset=utf-8"),
        )],
        PACK_CSS,
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    let schemas = Schemas::load().await?;
    let raw: Value = serde_json::from_str(MANIFEST_JSON)?;
    let manifest = Manifest::from_value(raw, &schemas)?;
    let id = manifest
        .id()
        .ok_or("embedded manifest has no service identity")?;

    theme::lint(PACK_CSS).map_err(|error| format!("pack.css: {error}"))?;
    let health = health::state(&schemas, id)?;
    let events = events::router(&schemas, id)?;
    let transport = Transport::bind(id, manifest.declared_endpoint()).await?;

    let app = Router::new()
        .route("/fragment", get(fragment::get))
        .route("/health", get(health::get))
        .route("/static/pack.css", get(pack_css))
        .with_state(health)
        .merge(events);

    axum::serve(transport.listener, app).await?;
    Ok(())
}

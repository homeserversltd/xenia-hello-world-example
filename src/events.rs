use axum::Router;
use xenia_kit::{events::Events, manifest::Result, Schemas};

pub fn router(schemas: &Schemas, id: &str) -> Result<Router> {
    Events::new(schemas, format!("/api/tabs/{id}/events/renew")).map(|events| events.router())
}

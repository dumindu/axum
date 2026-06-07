use axum::{extract::Path, response::IntoResponse};
use tracing::info;
use uuid::Uuid;

use crate::errors::Error;

pub async fn list() -> impl IntoResponse {
    Error::DbFetch
}

pub async fn create() -> impl IntoResponse {
    Error::DbInsert
}

pub async fn read(Path(id): Path<Uuid>) -> impl IntoResponse {
    tracing::info!(id = %id);
    Error::DbFetch
}

pub async fn update(Path(id): Path<Uuid>) -> impl IntoResponse {
    info!(id = %id);
    Error::DbUpdate
}

pub async fn delete(Path(id): Path<Uuid>) -> impl IntoResponse {
    info!(id = %id);
    Error::DbDelete
}

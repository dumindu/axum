mod handlers;
mod payload;

use axum::{Router, routing::get};
use utoipa::OpenApi;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list).post(handlers::create))
        .route("/{id}", get(handlers::read).put(handlers::update).delete(handlers::delete))
}

#[derive(OpenApi)]
#[openapi(
    paths(handlers::list, handlers::create, handlers::read, handlers::update, handlers::delete),
    components(schemas(payload::BookRequest))
)]
pub struct BookApi;

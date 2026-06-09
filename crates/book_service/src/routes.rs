use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

use crate::{app, state::AppState};

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/livez", get(livez))
        .nest("/v1/books", app::book::router())
        .with_state(state)
}

async fn livez() -> impl IntoResponse {
    StatusCode::OK
}

use std::str::FromStr;

use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{app, state::AppState};

pub fn init(state: AppState) -> Router {
    let cors_layer = cors_layer(state.clone());
    Router::new()
        .route("/livez", get(livez))
        .nest("/v1/books", app::book::router())
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn livez() -> impl IntoResponse {
    StatusCode::OK
}

fn cors_layer(state: AppState) -> CorsLayer {
    let origins: Vec<HeaderValue> = state
        .server_conf
        .allowed_origins
        .split(',')
        .map(|s| HeaderValue::try_from(s.trim()).expect("Invalid origin header"))
        .collect();

    let methods: Vec<Method> = state
        .server_conf
        .allowed_methods
        .split(',')
        .map(|s| Method::from_str(s.trim()).expect("Invalid HTTP method"))
        .collect();

    let headers: Vec<HeaderName> = state
        .server_conf
        .allowed_headers
        .split(',')
        .map(|s| HeaderName::from_str(s.trim()).expect("Invalid HTTP header name"))
        .collect();

    CorsLayer::new().allow_origin(origins).allow_methods(methods).allow_headers(headers)
}

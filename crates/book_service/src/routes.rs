use std::{str::FromStr, time::Duration};

use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method, Request, Response, StatusCode},
    response::IntoResponse,
    routing::get,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::{Span, info, info_span};

use crate::{app, state::AppState};

const REQUEST_ID_HEADER: &str = "x-request-id";

pub fn init(state: AppState) -> Router {
    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(x_request_id.clone(), MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let request_id = request
                        .headers()
                        .get(REQUEST_ID_HEADER)
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("unknown");

                    let uri_path = request.uri().path();

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        uri_path,
                        request_id,
                        status_code = tracing::field::Empty,
                        latency = tracing::field::Empty
                    )
                })
                .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
                    span.record("status_code", tracing::field::display(response.status()));
                    span.record("latency", latency.as_millis());

                    span.in_scope(|| {
                        info!("request completed");
                    });
                }),
        )
        .layer(PropagateRequestIdLayer::new(x_request_id))
        .layer(cors_layer(state.clone()));

    Router::new()
        .route("/livez", get(livez))
        .nest("/v1/books", app::book::router())
        .layer(middleware)
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

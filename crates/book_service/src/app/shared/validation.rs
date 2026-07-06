use std::collections::HashMap;

use axum::{
    Json,
    extract::{FromRef, FromRequest, Request},
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use garde::Validate;
use serde::{Serialize, de::DeserializeOwned};
use utoipa::ToSchema;

use crate::AppState;

#[derive(Serialize, ToSchema)]
#[schema(examples(
    r#"{ "errors": { "image_url": "not a valid url", "title": "length is greater than 255" } }"#
))]
pub struct ValidationErrorResponse {
    pub errors: HashMap<String, String>,
}

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate<Context = ()>,
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        validate_content_type(req.headers())?;

        let app_state = AppState::from_ref(state);
        let bytes = read_request_body(req, app_state.server_conf.default_body_limit).await?;

        let value: T = deserialize_payload(&bytes)?;

        validate_business_rules(&value)?;

        Ok(ValidatedJson(value))
    }
}

/// Step 1: Content-Type Validation
fn validate_content_type(headers: &axum::http::HeaderMap) -> Result<(), Response> {
    let has_json_header = headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|ct| ct.starts_with("application/json"));

    if !has_json_header {
        let mut errors = HashMap::with_capacity(1);
        errors.insert(
            "body".to_string(),
            "Missing required Content-Type: application/json header".to_string(),
        );
        return Err(
            (StatusCode::BAD_REQUEST, Json(ValidationErrorResponse { errors })).into_response()
        );
    }
    Ok(())
}

/// Step 2: Read Body
async fn read_request_body(
    req: Request,
    body_bytes_limit: usize,
) -> Result<axum::body::Bytes, Response> {
    axum::body::to_bytes(req.into_body(), body_bytes_limit).await.map_err(|err| {
        let mut errors = HashMap::with_capacity(1);
        errors.insert("body".to_string(), format!("Failed to read request body: {}", err));
        (StatusCode::BAD_REQUEST, Json(ValidationErrorResponse { errors })).into_response()
    })
}

/// Step 3: Deserialize Payload (Supports Native Jiff Types out of the box)
fn deserialize_payload<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Response> {
    let deserializer = &mut serde_json::Deserializer::from_slice(bytes);

    serde_path_to_error::deserialize(deserializer).map_err(|serde_err| {
        let mut errors = HashMap::with_capacity(1);

        let field_path = serde_err.path().to_string();
        let inner_err = serde_err.into_inner();

        let clean_msg = inner_err.to_string();
        let isolated_msg = clean_msg.split(" at line ").next().unwrap_or(&clean_msg);
        let field_key = if field_path.is_empty() { "body".to_string() } else { field_path };

        errors.insert(field_key, isolated_msg.trim().to_string());
        (StatusCode::BAD_REQUEST, Json(ValidationErrorResponse { errors })).into_response()
    })
}

/// Step 4: Validate Domain & Business Invariants
fn validate_business_rules<T>(value: &T) -> Result<(), Response>
where
    T: Validate<Context = ()>,
{
    if let Err(report) = value.validate_with(&()) {
        let mut errors = HashMap::with_capacity(report.iter().count());

        for (path, error) in report.iter() {
            let message =
                if !error.message().is_empty() { error.message() } else { "Invalid field value" };

            errors.insert(path.to_string(), message.to_string());
        }

        return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(ValidationErrorResponse { errors }))
            .into_response());
    }
    Ok(())
}

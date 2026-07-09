use std::{borrow::Cow, collections::HashMap, fmt::Display};

use axum::{
    Json,
    extract::{FromRef, FromRequest, Request},
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use garde::{
    Validate,
    i18n::{
        I18n, InvalidCreditCard, InvalidEmail, InvalidPhoneNumber, InvalidUrl, IpKind, with_i18n,
    },
};
use serde::{Serialize, de::DeserializeOwned};
use utoipa::ToSchema;

use crate::AppState;

#[derive(Serialize, ToSchema)]
#[schema(examples(
    r#"{ "errors": { "image_url": "Must be a valid URL", "title": "Must be at least 1 character long" } }"#
))]
pub struct ValidationErrorResponse {
    pub errors: HashMap<Cow<'static, str>, Cow<'static, str>>,
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

        validate_rules(&value)?;

        Ok(ValidatedJson(value))
    }
}

/// Step 1: Content-Type Validation
fn validate_content_type(headers: &axum::http::HeaderMap) -> Result<(), Response> {
    let valid = headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|ct| ct.starts_with("application/json"));
    if !valid {
        return Err(
            (StatusCode::BAD_REQUEST, "{\"error\":\"Must be application/json\"}").into_response()
        );
    }
    Ok(())
}

/// Step 2: Read Body
async fn read_request_body(
    req: Request,
    body_bytes_limit: usize,
) -> Result<axum::body::Bytes, Response> {
    axum::body::to_bytes(req.into_body(), body_bytes_limit)
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, "{\"error\":\"Malformed request\"}").into_response())
}

/// Step 3: Deserialize Payload
fn deserialize_payload<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Response> {
    let deserializer = &mut serde_json::Deserializer::from_slice(bytes);

    serde_path_to_error::deserialize(deserializer).map_err(|err| {
        (StatusCode::BAD_REQUEST, format!("{{\"error\":\"{}\"}}", err)).into_response()
    })
}

/// Step 4: Validate Rules
fn validate_rules<T>(value: &T) -> Result<(), Response>
where
    T: Validate<Context = ()>,
{
    if let Err(report) = with_i18n(English, || value.validate()) {
        let mut errors = HashMap::with_capacity(report.iter().count());

        for (path, error) in report.iter() {
            let message = if !error.message().is_empty() {
                Cow::Owned(error.message().to_string())
            } else {
                Cow::Borrowed("Invalid field value")
            };

            errors.insert(Cow::Owned(path.to_string()), message);
        }

        return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(ValidationErrorResponse { errors }))
            .into_response());
    }
    Ok(())
}

struct English;

impl I18n for English {
    fn length_lower_than(&self, min: usize) -> Cow<'static, str> {
        match min {
            1 => Cow::Borrowed("Must be at least 1 character long"),
            _ => format!("Must be at least {min} characters long").into(),
        }
    }

    fn length_greater_than(&self, max: usize) -> Cow<'static, str> {
        match max {
            1 => Cow::Borrowed("Must not exceed 1 character"),
            _ => format!("Must not exceed {max} characters").into(),
        }
    }

    fn range_lower_than(&self, min: &dyn Display) -> Cow<'static, str> {
        format!("Must be greater than or equal to {min}").into()
    }

    fn range_greater_than(&self, max: &dyn Display) -> Cow<'static, str> {
        format!("Must be less than or equal to {max}").into()
    }

    fn credit_card_invalid(&self, _reason: InvalidCreditCard) -> Cow<'static, str> {
        Cow::Borrowed("Must be a valid credit card number")
    }

    fn pattern_no_match(&self, _pattern: &dyn Display) -> Cow<'static, str> {
        Cow::Borrowed("Must match the required format")
    }

    fn contains_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("Must contain \"{pattern}\"").into()
    }

    fn url_invalid(&self, _reason: InvalidUrl) -> Cow<'static, str> {
        Cow::Borrowed("Must be a valid URL")
    }

    fn prefix_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("Must start with \"{pattern}\"").into()
    }

    fn suffix_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("Must end with \"{pattern}\"").into()
    }

    fn phone_number_invalid(&self, _reason: InvalidPhoneNumber) -> Cow<'static, str> {
        Cow::Borrowed("Must be a valid phone number")
    }

    fn ip_invalid(&self, kind: IpKind) -> Cow<'static, str> {
        format!("Must be a valid {kind} address").into()
    }

    fn matches_field_mismatch(&self, field: &dyn Display) -> Cow<'static, str> {
        format!("Must match the {field} field").into()
    }

    fn email_invalid(&self, _reason: InvalidEmail) -> Cow<'static, str> {
        Cow::Borrowed("Must be a valid email address")
    }

    fn ascii_invalid(&self) -> Cow<'static, str> {
        Cow::Borrowed("Must contain only ASCII characters")
    }

    fn alphanumeric_invalid(&self) -> Cow<'static, str> {
        Cow::Borrowed("Must contain only letters and numbers")
    }

    fn required_not_set(&self) -> Cow<'static, str> {
        Cow::Borrowed("This field is required")
    }
}

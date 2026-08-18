use std::{borrow::Cow, collections::HashMap, fmt::Display};

use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use garde::{
    Report, Validate,
    i18n::{
        I18n, InvalidCreditCard, InvalidEmail, InvalidPhoneNumber, InvalidUrl, IpKind, with_i18n,
    },
};
use serde::{Serialize, de::DeserializeOwned};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate<Context = ()>,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        with_i18n(English, || value.validate())?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Serialize, ToSchema)]
#[schema(examples(
    r#"{ "errors": { "image_url": "Must be a valid URL", "title": "Must be at least 1 character long" } }"#
))]
pub struct ValidationErrorResponse {
    pub errors: HashMap<String, String>,
}

#[derive(Debug)]
pub enum ServerError {
    ValidationError(Report),
    AxumJsonRejection(JsonRejection),
}

impl From<Report> for ServerError {
    fn from(err: Report) -> Self {
        Self::ValidationError(err)
    }
}

impl From<JsonRejection> for ServerError {
    fn from(err: JsonRejection) -> Self {
        Self::AxumJsonRejection(err)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            Self::ValidationError(report) => {
                let errors = report
                    .iter()
                    .map(|(path, error)| (path.to_string(), error.message().to_string()))
                    .collect::<HashMap<_, _>>();

                (StatusCode::UNPROCESSABLE_ENTITY, Json(ValidationErrorResponse { errors }))
                    .into_response()
            }
            Self::AxumJsonRejection(rejection) => {
                (StatusCode::BAD_REQUEST, rejection).into_response()
            }
        }
    }
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

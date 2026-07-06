use axum::{
    body::Body,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use utoipa::ToSchema;

pub enum Error {
    DbInsert,
    DbFetch,
    DbUpdate,
    DbDelete,
    Serialization,
    Deserialization,
    InvalidResourceId,
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, bytes): (StatusCode, &'static [u8]) = match self {
            Error::DbInsert => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"DB_INSERT_FAILED\"}")
            }
            Error::DbFetch => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"DB_FETCH_FAILED\"}")
            }
            Error::DbUpdate => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"DB_UPDATE_FAILED\"}")
            }
            Error::DbDelete => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"DB_DELETE_FAILED\"}")
            }
            Error::Serialization => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"SERIALIZATION_FAILED\"}")
            }

            Error::Deserialization => {
                (StatusCode::BAD_REQUEST, b"{\"error\": \"DESERIALIZATION_FAILED\"}")
            }
            Error::InvalidResourceId => {
                (StatusCode::BAD_REQUEST, b"{\"error\": \"INVALID_RESOURCE_ID\"}")
            }
            Error::NotFound => (StatusCode::NOT_FOUND, b""),
        };

        let mut response = Response::new(Body::from(bytes));
        *response.status_mut() = status;
        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

        response
    }
}

#[derive(ToSchema)]
#[schema(examples(r#"{"error": "DB_INSERT_FAILED"}"#))]
pub struct ErrorResponse {
    pub error: ErrorCode,
}

#[derive(ToSchema)]
pub enum ErrorCode {
    #[schema(rename = "DB_INSERT_FAILED")]
    DbInsertFailed,
    #[schema(rename = "DB_FETCH_FAILED")]
    DbFetchFailed,
    #[schema(rename = "DB_UPDATE_FAILED")]
    DbUpdateFailed,
    #[schema(rename = "DB_DELETE_FAILED")]
    DbDeleteFailed,
    #[schema(rename = "SERIALIZATION_FAILED")]
    SerializationFailed,
    #[schema(rename = "DESERIALIZATION_FAILED")]
    DeserializationFailed,
    #[schema(rename = "INVALID_RESOURCE_ID")]
    InvalidResourceId,
}

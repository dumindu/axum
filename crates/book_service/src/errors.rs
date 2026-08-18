use axum::{
    body::Body,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use utoipa::ToSchema;

#[derive(ToSchema)]
#[schema(examples(r#"{"error": "DB_FETCH_FAILED"}"#))]
pub struct ErrorResponse {
    #[expect(dead_code, reason = "OpenAPI YAML")]
    pub error: Error,
}

#[derive(ToSchema)]
pub enum Error {
    #[schema(rename = "DB_INSERT_FAILED")]
    DbInsert,
    #[schema(rename = "DB_FETCH_FAILED")]
    DbFetch,
    #[schema(rename = "DB_UPDATE_FAILED")]
    DbUpdate,
    #[schema(rename = "DB_DELETE_FAILED")]
    DbDelete,
    #[serde(skip)]
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

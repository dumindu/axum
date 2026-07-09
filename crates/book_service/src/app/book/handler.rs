use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::{error, info};
use uuid::Uuid;

use super::payload::BookRequest;
use crate::{
    app::shared::{Pagination, ValidatedJson, ValidationErrorResponse},
    errors::{Error, ErrorResponse},
    models::Book,
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/v1/books",
    tag = "Books",
    params(
        Pagination
    ),
    responses(
        (status = 200, description = "A successful list read", body = [Book]),
        (status = 500, description = "An internal failure", body = ErrorResponse)
    )
)]
pub async fn list(
    State(mut state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, Error> {
    let (limit, offset) = pagination.limit_offset();
    let books =
        Book::all().limit(limit).offset(offset).exec(&mut state.db).await.map_err(|err| {
            error!(target: "database", "failed to fetch: {err:?}");
            Error::DbFetch
        })?;
    Ok((StatusCode::OK, Json(books)))
}

#[utoipa::path(
    post,
    path = "/v1/books",
    tag = "Books",
    request_body = BookRequest,
    responses(
        (status = 201, description = "A successful create", body = Book),
        (status = 400, description = "An invalid payload", body = ErrorResponse),
        (status = 422, description = "An unprocessable payload", body = ValidationErrorResponse),
        (status = 500, description = "An internal failure", body = ErrorResponse)
    )
)]
pub async fn create(
    State(mut state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<BookRequest>,
) -> Result<impl IntoResponse, Error> {
    let saved = toasty::create!(Book {
        title: payload.title,
        published_date: payload.published_date,
        image_url: payload.image_url,
        description: payload.description,
    })
    .exec(&mut state.db)
    .await
    .map_err(|err| {
        error!(target: "database", "failed to insert: {err:?}");
        Error::DbInsert
    })?;

    info!(id = %saved.id, title = %saved.title, "new book created");

    Ok((StatusCode::CREATED, Json(saved)))
}

#[utoipa::path(
    get,
    path = "/v1/books/{id}",
    tag = "Books",
    params(
        ("id" = Uuid, Path, description = "The UUIDv7 identifier of the book")
    ),
    responses(
        (status = 200, description = "A successful read", body = Book),
        (status = 404, description = "A record could not be found"),
        (status = 500, description = "An internal failure", body = ErrorResponse)
    )
)]
pub async fn read(
    State(mut state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let book = Book::get_by_id(&mut state.db, &id).await.map_err(|err| {
        if err.is_record_not_found() {
            Error::NotFound
        } else {
            error!(target: "database", "failed to fetch: {err:?}");
            Error::DbFetch
        }
    })?;

    Ok((StatusCode::OK, Json(book)))
}

#[utoipa::path(
    put,
    path = "/v1/books/{id}",
    tag = "Books",
    request_body = BookRequest,
    params(
        ("id" = Uuid, Path, description = "The UUIDv7 identifier of the book")
    ),
    responses(
        (status = 200, description = "A successful update", body = Book),
        (status = 400, description = "An invalid payload", body = ErrorResponse),
        (status = 404, description = "A record could not be found"),
        (status = 422, description = "An unprocessable payload", body = ValidationErrorResponse),
        (status = 500, description = "An internal failure", body = ErrorResponse)
    )
)]
pub async fn update(
    State(mut state): State<AppState>,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<BookRequest>,
) -> Result<impl IntoResponse, Error> {
    let mut book = Book::get_by_id(&mut state.db, &id).await.map_err(|err| {
        if err.is_record_not_found() {
            Error::NotFound
        } else {
            error!(target: "database", "failed to fetch: {err:?}");
            Error::DbFetch
        }
    })?;

    toasty::update!(book {
        title: payload.title,
        published_date: payload.published_date,
        image_url: payload.image_url,
        description: payload.description,
    })
    .exec(&mut state.db)
    .await
    .map_err(|err| {
        error!(target: "database", "failed to update: {err:?}");
        Error::DbUpdate
    })?;

    info!(id = %id, "book updated");

    Ok((StatusCode::OK, Json(book)))
}

#[utoipa::path(
    delete,
    path = "/v1/books/{id}",
    tag = "Books",
    params(
        ("id" = Uuid, Path, description = "The UUIDv7 identifier of the book")
    ),
    responses(
        (status = 204, description = "A successful delete"),
        (status = 500, description = "An internal failure", body = ErrorResponse)
    )
)]
pub async fn delete(
    State(mut state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    Book::delete_by_id(&mut state.db, id).await.map_err(|err| {
        error!(target: "database", "failed to delete: {err:?}");
        Error::DbDelete
    })?;

    info!(id = %id, "book deleted");

    Ok(StatusCode::NO_CONTENT)
}

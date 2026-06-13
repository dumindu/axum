use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::{error, info};
use uuid::Uuid;

use super::payload::{BookRequest, BookResponse};
use crate::{app::shared::Pagination, errors::Error, models::Book, state::AppState};

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

    let response: Vec<BookResponse> = books.into_iter().map(BookResponse::from).collect();
    Ok((StatusCode::OK, Json(response)))
}

pub async fn create(
    State(mut state): State<AppState>,
    Json(payload): Json<BookRequest>,
) -> Result<impl IntoResponse, Error> {
    let saved = toasty::create!(Book {
        title: payload.title.clone(),
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

    info!(id = %saved.id, title = %payload.title, "new book created");

    Ok((StatusCode::CREATED, Json(BookResponse::from(saved))))
}

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

    Ok((StatusCode::OK, Json(BookResponse::from(book))))
}

pub async fn update(
    State(mut state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<BookRequest>,
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

    Ok((StatusCode::OK, Json(BookResponse::from(book))))
}

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

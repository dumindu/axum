use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Book;

#[derive(Debug, Deserialize)]
pub struct BookRequest {
    pub title: String,
    pub published_date: Date,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BookResponse {
    pub id: Uuid,
    pub title: String,
    pub published_date: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        Self {
            id: book.id,
            title: book.title,
            published_date: book.published_date.to_string(),
            image_url: book.image_url,
            description: book.description,
            created_at: book.created_at.to_string(),
            updated_at: book.updated_at.to_string(),
        }
    }
}

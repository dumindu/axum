use garde::Validate;
use jiff::civil::Date;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::models::BookStatus;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BookRequest {
    #[garde(length(min = 1, max = 255))]
    #[schema(
        min_length = 1,
        max_length = 255,
        examples("Harry Potter and the Philosopher's Stone")
    )]
    pub title: String,

    #[garde(skip)]
    #[schema(value_type = String, format = Date, examples("2026-01-31"))]
    pub published_date: Date,

    #[garde(url)]
    #[schema(nullable, examples("https://example.com"))]
    pub image_url: Option<String>,

    #[garde(skip)]
    #[schema(nullable, examples("An orphaned boy discovers he is a wizard"))]
    pub description: Option<String>,

    #[garde(skip)]
    pub status: BookStatus,
}

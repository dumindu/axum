use jiff::{Timestamp, civil::Date};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, toasty::Model, Serialize, ToSchema)]
pub struct Book {
    #[key]
    #[auto]
    #[schema(value_type = String, format = Uuid, examples("019f36b5-660f-7714-a9a3-e8555ba44cfc"))]
    pub id: Uuid,

    #[schema(examples("The title of the book"))]
    pub title: String,

    #[schema(value_type = String, format = Date, examples("2026-01-31"))]
    pub published_date: Date,

    #[schema(nullable, examples("https://example.com"))]
    pub image_url: Option<String>,

    #[schema(nullable, examples("A description for the book."))]
    pub description: Option<String>,

    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2026-07-06T13:38:00Z"))]
    pub created_at: Timestamp,

    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2026-07-06T13:38:00Z"))]
    pub updated_at: Timestamp,
}

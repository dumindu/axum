use jiff::{Timestamp, civil::Date};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, toasty::Model, Serialize)]
pub struct Book {
    #[key]
    #[auto]
    pub id: Uuid,

    pub title: String,
    pub published_date: Date,
    pub image_url: Option<String>,
    pub description: Option<String>,

    #[auto]
    pub created_at: Timestamp,

    #[auto]
    pub updated_at: Timestamp,
}

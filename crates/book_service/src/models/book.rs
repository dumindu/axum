use jiff::{Timestamp, civil::Date};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, toasty::Model, Serialize, ToSchema)]
pub struct Book {
    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2027-01-01T00:00:00.123456Z"))]
    pub created_at: Timestamp,

    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2027-01-01T00:00:00.123456Z"))]
    pub updated_at: Timestamp,

    #[key]
    #[auto]
    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub id: Uuid,

    #[schema(value_type = String, format = Date, examples("2007-07-21"))]
    pub published_date: Date,

    pub status: BookStatus,

    #[schema(examples("Harry Potter and the Deathly Hallows"))]
    pub title: String,

    #[schema(nullable, examples("It is the seventh and final novel in the Harry Potter series"))]
    pub description: Option<String>,

    #[schema(nullable, examples("https://upload.wikimedia.org/wikipedia/en/a/a9/Harry_Potter_and_the_Deathly_Hallows.jpg"))]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, toasty::Embed, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[column(type = u8)]
pub enum BookStatus {
    #[column(variant = 0)]
    Pending,
    #[column(variant = 1)]
    Verified,
}

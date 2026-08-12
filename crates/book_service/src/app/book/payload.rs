use garde::Validate;
use jiff::civil::Date;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::models::BookStatus;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BookRequest {
    #[garde(length(min = 1, max = 255))]
    #[schema(min_length = 1, max_length = 255, examples("Harry Potter and the Deathly Hallows"))]
    pub title: String,

    #[garde(skip)]
    #[schema(nullable, examples("It is the seventh and final novel in the Harry Potter series"))]
    pub description: Option<String>,

    #[garde(url)]
    #[schema(
        nullable,
        examples(
            "https://upload.wikimedia.org/wikipedia/en/a/a9/Harry_Potter_and_the_Deathly_Hallows.jpg"
        )
    )]
    pub image_url: Option<String>,

    #[garde(skip)]
    #[schema(value_type = String, format = Date, examples("2007-07-21"))]
    pub published_date: Date,

    #[garde(skip)]
    pub status: BookStatus,
}

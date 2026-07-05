use garde::Validate;
use jiff::civil::Date;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct BookRequest {
    #[garde(length(min = 1, max = 25))]
    pub title: String,
    #[garde(skip)]
    pub published_date: Date,
    #[garde(url)]
    pub image_url: Option<String>,
    #[garde(skip)]
    pub description: Option<String>,
}

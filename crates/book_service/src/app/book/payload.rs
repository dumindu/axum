use jiff::civil::Date;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BookRequest {
    pub title: String,
    pub published_date: Date,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

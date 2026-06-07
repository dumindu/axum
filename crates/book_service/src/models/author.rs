use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
pub struct Author {
    #[key]
    #[auto]
    pub id: Uuid,

    pub name: String,
    pub description: Option<String>,

    #[auto]
    pub created_at: Timestamp,

    #[auto]
    pub updated_at: Timestamp,
}

use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
pub struct Author {
    #[auto]
    pub created_at: Timestamp,

    #[auto]
    pub updated_at: Timestamp,

    #[key]
    #[auto]
    pub id: Uuid,

    pub name: String,
    pub description: Option<String>,
}

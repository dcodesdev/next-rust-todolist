use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TodoItem {
    #[ts(type = "string")]
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    #[ts(type = "string")]
    pub list_id: Uuid,
    #[ts(type = "string")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,
}

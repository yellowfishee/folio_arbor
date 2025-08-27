use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LiteratureNote {
    pub id: i64,
    pub content: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

impl LiteratureNote {
    pub fn new(content: String) -> Self {
        Self {
            id: 0,
            content,
            create_time: Utc::now(),
            update_time: Utc::now(),
        }
    }
}

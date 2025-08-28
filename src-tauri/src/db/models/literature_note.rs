use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use chrono::NaiveDateTime;
use sqlx::sqlite::SqliteRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct LiteratureNote {
    pub id: i64,
    pub content: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

// 手动实现FromRow而不是使用派生
impl FromRow<'_, SqliteRow> for LiteratureNote {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let content: String = row.try_get("content")?;
        let create_time_naive: NaiveDateTime = row.try_get("create_time")?;
        let update_time_naive: NaiveDateTime = row.try_get("update_time")?;

        Ok(Self {
            id,
            content,
            create_time: DateTime::from_naive_utc_and_offset(create_time_naive, Utc),
            update_time: DateTime::from_naive_utc_and_offset(update_time_naive, Utc),
        })
    }
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



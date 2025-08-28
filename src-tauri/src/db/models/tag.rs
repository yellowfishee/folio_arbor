use chrono::{DateTime, Utc};
use sqlx::{FromRow};

#[derive(Debug, serde::Serialize, serde::Deserialize, FromRow)]
pub struct Tag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_id: Option<i64>,
    #[serde(skip_deserializing)]  // 添加此属性跳过反序列化
    pub create_time: DateTime<Utc>,
    #[serde(skip_deserializing)]  // 添加此属性跳过反序列化
    pub update_time: DateTime<Utc>,
}




impl Tag {
    pub fn new(full_name: String, p_id: Option<i64>) -> Self {
        Self {
            id: None,
            full_name,
            p_id,
            create_time: Utc::now(),
            update_time: Utc::now(),
        }
    }
}

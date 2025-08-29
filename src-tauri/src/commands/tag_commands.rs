use crate::db::connections::DbState;
use crate::db::models::Tag;
use crate::services::TagService;
use tauri::{command};

#[command]
pub async fn get_tags(db: tauri::State<'_, DbState>) -> Result<Vec<Tag>, String> {
    TagService::get_tag_list(&db.db)
        .await
        .map_err(|e| e.to_string())
}

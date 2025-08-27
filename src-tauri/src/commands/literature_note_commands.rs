use crate::db::connections::DbState;
use crate::services::LiteratureNoteService;
use tauri::{command, State};
use crate::db::models::LiteratureNote;

#[command]
pub async fn create_literature_note(
    content: String,
    da_state: State<'_, DbState>,
) -> Result<String, String> {
    LiteratureNoteService::create_literature_note(&da_state.db, content)
        .await
        .map(|note| format!("Note created with ID: {}", note.id))
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_all_literature_notes(db_state: State<'_, DbState>) -> Result<Vec<LiteratureNote>, String> {
    LiteratureNoteService::get_all_literature_notes(&db_state.db)
        .await
        .map(|notes| notes)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_literature_note(
    id: i64,
    db_state: State<'_, DbState>,
) -> Result<String, String> {
    LiteratureNoteService::delete_literature_note(&db_state.db, id)
        .await
        .map(|_| format!("Note deleted with ID: {}", id))
        .map_err(|e| e.to_string())
}

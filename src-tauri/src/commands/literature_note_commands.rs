use crate::db::connections::DbState;
use crate::services::LiteratureNoteService;
use tauri::{command, State};

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
pub async fn get_all_literature_notes(db_state: State<'_, DbState>) -> Result<Vec<String>, String> {
    LiteratureNoteService::get_all_literature_notes(&db_state.db)
        .await
        .map(|notes| notes.iter().map(|note| note.content.clone()).collect())
        .map_err(|e| e.to_string())
}

use sqlx::SqlitePool;

use crate::db::models::LiteratureNote;


pub struct LiteratureNoteService;

impl LiteratureNoteService {
    pub async fn create_literature_note(
        pool: &SqlitePool,
        content: String
    ) -> Result<LiteratureNote, sqlx::Error> {
        let note = LiteratureNote::new(content);
        let result = sqlx::query_as!(
            LiteratureNote,
            "INSERT INTO literature_notes (content, create_time, update_time) VALUES (?, ?, ?) RETURNING *",
            note.content,
            note.create_time,
            note.update_time
        )
        .fetch_one(pool)
        .await;

        result
    }

       // 获取所有笔记
    pub async fn get_all_literature_notes(pool: &SqlitePool) -> Result<Vec<LiteratureNote>, sqlx::Error> {
        sqlx::query_as!(LiteratureNote, "SELECT * FROM literature_notes ORDER BY create_time DESC")
            .fetch_all(pool)
            .await
    }
}

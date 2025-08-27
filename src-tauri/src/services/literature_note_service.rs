use crate::db::models::LiteratureNote;
use sqlx::SqlitePool;
pub struct LiteratureNoteService;

impl LiteratureNoteService {
    pub async fn create_literature_note(
        pool: &SqlitePool,
        content: String,
    ) -> Result<LiteratureNote, sqlx::Error> {
        let note = LiteratureNote::new(content);

        // 手动执行插入并获取ID
        let result = sqlx::query(
            "INSERT INTO literature_notes (content, create_time, update_time) VALUES (?, ?, ?)"
        )
            .bind(&note.content)
            .bind(&note.create_time)
            .bind(&note.update_time)
            .execute(pool)
            .await?;

        // 获取刚插入的记录的ID
        let id = result.last_insert_rowid();

        // 返回新创建的笔记
        Ok(LiteratureNote {
            id: id as i64,
            content: note.content.clone(),
            create_time: note.create_time.clone(),
            update_time: note.update_time.clone(),
        })
    }

    // 获取所有笔记
    pub async fn get_all_literature_notes(
        pool: &SqlitePool,
    ) -> Result<Vec<LiteratureNote>, sqlx::Error> {
        sqlx::query_as::<_, LiteratureNote>(
            "SELECT * FROM literature_notes ORDER BY create_time DESC"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn delete_literature_note(
        pool: &SqlitePool,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM literature_notes WHERE id = ?"
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

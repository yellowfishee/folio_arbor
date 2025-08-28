use log::info;
use crate::db::models::LiteratureNote;
use sqlx::SqlitePool;
use crate::db::models::Tag;
use crate::services::tag_service::TagService;
pub struct LiteratureNoteService;

impl LiteratureNoteService {
    pub async fn create_literature_note(
        pool: &SqlitePool,
        content: String,
        tags: Vec<Tag>,
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
        info!("插入笔记成功，ID: {}", id);

        // 处理标签
        let tag_service = TagService {};
        for tag in tags {
            let tag_new = Tag::new(tag.full_name.clone(), None);
            if tag.id.is_none() {
                let tag_id = tag_service.create_tag(pool, &tag_new).await?;
                info!("创建标签成功，ID: {}", tag_id);
            } else {
                let tag_id = tag_service.get_tag_id_by_full_name(pool, tag.full_name.clone()).await?;
                info!("获取标签ID成功，ID: {}", tag_id);
            }

            // TODO 待处理标签关联
        }

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
        let notes = sqlx::query_as::<_, LiteratureNote>(
            "SELECT * FROM literature_notes ORDER BY create_time DESC"
        )
        .fetch_all(pool)
        .await;
        info!("获取所有笔记成功");
        notes
    }

    pub async fn delete_literature_note(
        pool: &SqlitePool,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM literature_notes WHERE id = ?"
        )
        .bind(id)
        .execute(pool)
        .await?;
        info!("删除笔记成功，影响行数: {}", result.rows_affected());
        Ok(())
    }
}

use log::info;
use sqlx::{SqlitePool};
use crate::db::models::Tag;

pub struct TagService;

impl TagService {
    pub async fn create_tag(
        &self,
        pool: &SqlitePool,
        tag: &Tag,
    ) -> Result<i64, sqlx::Error> {
        let tag_father_id = self
            .get_tag_id_by_full_name(pool, tag.full_name.clone().as_str().parse().unwrap())
            .await?;
        info!("tag_father_id: {}", tag_father_id);
        match tag_father_id {
            0 => {
                // 顶层标签，p_id为0
                let tag = Tag::new(tag.full_name.clone(), None);
            }
            _ => {
                // 非顶层标签，p_id为tag_father_id
                let tag = Tag::new(tag.full_name.clone(), Some(tag_father_id));
            }
        }

        let result = sqlx::query(
            "INSERT INTO tags (full_name, p_id, create_time, update_time) VALUES (?, ?, ?, ?)"
        )
            .bind(tag.full_name.clone())
            .bind(tag.p_id.unwrap_or(0))
            .bind(tag.create_time)
            .bind(tag.update_time)
            .execute(pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_tag_id_by_full_name(
        &self,
        pool: &SqlitePool,
        full_name: String,
    ) -> Result<i64, sqlx::Error> {
        // full name严格遵循 #xxx/xxx/xxx，也就是说，我需要找到最后一个/的位置
        // 对于顶层 #xxx，来说，也就是没有找到/时，直接返回0作为p_id，标志其是顶级标签
        let last_slash_index = match full_name.rfind('/') {
            Some(index) => index,
            None => return Ok(0),
        };
        let tag = sqlx::query_as::<_, Tag>("SELECT id FROM tags WHERE full_name = (?)")
            .bind(&full_name[last_slash_index + 1..])
            .fetch_one(pool)
            .await?;
        Ok(tag.id.unwrap_or(0))
    }

    pub async fn get_tag_list(
        pool: &SqlitePool,
    ) -> Result<Vec<Tag>, sqlx::Error> {
        let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags")
            .fetch_all(pool)
            .await?;
        Ok(tags)
    }
}

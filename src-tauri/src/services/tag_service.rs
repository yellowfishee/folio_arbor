use crate::db::models::Tag;
use log::info;
use sqlx::SqlitePool;

pub struct TagService;

impl TagService {
    pub async fn create_tag(&self, pool: &SqlitePool, tag: &Tag) -> Result<i64, sqlx::Error> {
        let tag_father_id = self
            .get_tag_id_by_full_name(pool, tag.full_name.clone().as_str().parse().unwrap())
            .await?;
        info!("tag_father_id: {}", tag_father_id);
        let tag = match tag_father_id {
            0 => {
                // 顶层标签，p_id为0
                let tag = Tag::new(tag.full_name.clone(), None);
                tag
            }
            _ => {
                // 非顶层标签，p_id为tag_father_id
                let tag = Tag::new(tag.full_name.clone(), Some(tag_father_id));
                tag
            }
        };

        let result = sqlx::query(
            "INSERT INTO tags (full_name, p_id, create_time, update_time) VALUES (?, ?, ?, ?)",
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
        // 将全路径按最后一个'/'分割为父路径和子标签
        let (parent_path, child_tag) = match full_name.rsplit_once('/') {
            Some((parent, child)) => (parent, child),
            None => return Ok(0), // 没有父路径时返回0
        };

        // 添加调试日志
        info!("正在查询父标签路径: {}, 子标签: {}", parent_path, child_tag);

        // 使用更安全的查询方式
        let parent_tag: std::option::Option<Tag> = sqlx::query_as("SELECT * FROM tags WHERE full_name = $1")
            .bind(parent_path) // 查询子标签对应的父标签ID
            .fetch_optional(pool)
            .await?;

        // 添加调试日志
        info!("查询结果: {:?}", parent_tag);

        match parent_tag {
            Some(tag) => {
                info!("找到父标签ID: {}", tag.id.unwrap_or(0));
                Ok(tag.id.unwrap_or(0))
            }
            None => {
                info!("未找到匹配的父标签，使用默认值0");
                Ok(0)
            }
        }
    }

    pub async fn get_tag_list(pool: &SqlitePool) -> Result<Vec<Tag>, sqlx::Error> {
        let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags")
            .fetch_all(pool)
            .await?;
        Ok(tags)
    }
}

use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{App, Manager as _};
pub type Db = Pool<Sqlite>;

pub struct DbState {
    pub db: Db
}

pub async fn setup_db(app: &App) -> Result<Db, sqlx::Error> {
    let mut path = app.path().app_data_dir().unwrap();

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {},
        Err(e) => {
            println!("创建数据目录失败: {}", e);
        }
    };

    path.push("folio_arbor.db");
    Sqlite::create_database(format!("sqlite:{}", path.to_str().unwrap()).as_str()).await.unwrap_or(());
    let db = SqlitePoolOptions::new()
    .connect(path.to_str().unwrap())
    .await
    .unwrap();

    Ok(db)
}

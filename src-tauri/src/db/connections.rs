use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{App, Manager as _};
pub type Db = Pool<Sqlite>;

pub struct DbState {
    pub db: Db,
}

pub async fn setup_db(app: &App) -> Result<Db, sqlx::Error> {
    let mut path = app.path().app_data_dir().unwrap();

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("创建数据目录失败: {}", e);
            return Err(sqlx::Error::Io(e));
        }
    };

    path.push("folio_arbor.db");

    println!("数据库路径: {}", path.to_str().unwrap());

    Sqlite::create_database(format!("sqlite:{}", path.to_str().unwrap()).as_str())
        .await
        .unwrap_or(());
    let db = SqlitePoolOptions::new()
        .connect(format!("sqlite:///{}", path.to_str().unwrap()).as_str())
        .await?;

    // 添加连接测试
    match sqlx::query("SELECT 1").execute(&db).await {
        Ok(_) => println!("✅ 数据库连接成功"),
        Err(e) => {
            eprintln!("❌ 数据库连接失败: {}", e);
            return Err(e);
        }
    }

    sqlx::migrate!("./migrations")
        .run(&db)
        .await?;

    Ok(db)
}

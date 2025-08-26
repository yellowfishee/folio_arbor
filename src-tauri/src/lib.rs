use tauri::Manager;


mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = db::connections::setup_db(&app)
                    .await
                    .expect("数据库连接失败");
                app.manage(db::connections::DbState { db });
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("应用启动失败");
}

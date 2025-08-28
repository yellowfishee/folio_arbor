use log::info;
use tauri::Manager;
mod commands;
mod db;
mod services;
mod utils;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    utils::init_logger();

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
        .invoke_handler(tauri::generate_handler![
            commands::create_literature_note,
            commands::get_all_literature_notes,
            commands::delete_literature_note,
            commands::get_tags,
        ])
        .run(tauri::generate_context!())
        .expect("应用启动失败");
    info!("应用启动完成");
}

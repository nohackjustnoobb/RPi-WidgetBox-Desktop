mod methods;
mod utils;

use methods::media_activity::register_media_activity_event;
use tauri::generate_handler;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .invoke_handler(generate_handler![register_media_activity_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

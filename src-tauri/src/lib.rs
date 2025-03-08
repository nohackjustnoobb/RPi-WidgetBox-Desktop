mod methods;
mod utils;

use methods::media_activity::{
    next_track, pause_media, play_media, prev_track, register_media_activity_event,
    unregister_media_activity_event,
};
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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(generate_handler![
            register_media_activity_event,
            unregister_media_activity_event,
            play_media,
            pause_media,
            next_track,
            prev_track,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

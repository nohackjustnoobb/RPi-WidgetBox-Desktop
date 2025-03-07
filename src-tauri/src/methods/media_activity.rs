use serde::Serialize;
use tauri::AppHandle;

#[cfg(target_os = "macos")]
static FLAG: once_cell::sync::Lazy<(std::sync::Mutex<bool>, std::sync::Condvar)> =
    once_cell::sync::Lazy::new(|| (std::sync::Mutex::new(false), std::sync::Condvar::new()));

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaActivity {
    title: Option<String>,
    artist: Option<String>,
    app_name: Option<String>,
    app_icon: Option<String>,
    album: Option<String>,
    album_cover: Option<String>,
    duration: Option<f64>,
    elapsed: Option<f64>,
}

#[tauri::command]
#[cfg(target_os = "macos")]
pub fn register_media_activity_event(app: AppHandle) {
    let (lock, _) = &*FLAG;
    let mut flag = lock.lock().unwrap();
    if *flag {
        return;
    }
    *flag = true;

    use std::thread::spawn;

    use crate::utils::image_as_base64;
    use media_remote::NowPlaying;
    use tauri::Emitter;

    spawn(move || {
        let now_playing = NowPlaying::new();

        // forwarding the events
        now_playing.subscribe(move |guard| {
            let info = guard.as_ref();

            if let Some(info) = info {
                app.emit(
                    "media-activity",
                    MediaActivity {
                        title: info.title.clone(),
                        artist: info.artist.clone(),
                        app_name: info.bundle_name.clone(),
                        app_icon: info
                            .bundle_icon
                            .as_ref()
                            .and_then(|img| image_as_base64(img)),
                        album: info.album.clone(),
                        album_cover: info
                            .album_cover
                            .as_ref()
                            .and_then(|img| image_as_base64(img)),
                        duration: info.duration,
                        elapsed: info.elapsed_time,
                    },
                )
                .unwrap();
            }
        });

        // Blocks forever to keep `now_playing` alive
        let (lock, cvar) = &*FLAG;
        let mut flag = lock.lock().unwrap();
        while *flag {
            flag = cvar.wait(flag).unwrap();
        }
    });
}

#[tauri::command]
#[cfg(target_os = "macos")]
pub fn unregister_media_activity_event() {
    let (lock, cvar) = &*FLAG;
    let mut flag = lock.lock().unwrap();
    *flag = false;
    cvar.notify_one();
}

#[tauri::command]
#[cfg(not(target_os = "macos"))]
pub fn register_media_activity_event() {
    // TODO: implement for other platforms
}

#[tauri::command]
#[cfg(not(target_os = "macos"))]
pub fn unregister_media_activity_event() {
    // TODO: implement for other platforms
}

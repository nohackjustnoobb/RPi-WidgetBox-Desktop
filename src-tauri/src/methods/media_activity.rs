use std::sync::atomic::AtomicBool;

use serde::Serialize;
use tauri::AppHandle;

static mut REGISTERED: AtomicBool = AtomicBool::new(false);

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
#[cfg(not(target_os = "macos"))]
pub fn register_media_activity_event() {
    // TODO: implement for other platforms
}

#[tauri::command]
#[cfg(target_os = "macos")]
pub fn register_media_activity_event(app: AppHandle) {
    let registered = unsafe { REGISTERED.get_mut() };
    if *registered {
        return;
    }
    *registered = true;

    use std::{
        sync::{Arc, Condvar, Mutex},
        thread,
    };

    use crate::utils::image_as_base64;
    use media_remote::NowPlaying;
    use tauri::Emitter;

    thread::spawn(move || {
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
        let pair = Arc::new((Mutex::new(()), Condvar::new()));
        let (lock, cvar) = &*pair;

        let guard = lock.lock().unwrap();
        let _unused = cvar.wait(guard).unwrap();
    });
}

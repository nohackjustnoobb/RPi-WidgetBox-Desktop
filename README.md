# RPi WidgetBox Desktop

This repository serves as the desktop editor for [RPi-WidgetBox](https://github.com/nohackjustnoobb/RPi-WidgetBox). It provides system-level APIs that plugins can utilize.

## API Reference

**Note:** You should use the `@tauri-apps/api` to invoke these methods or listen to events.

<details>
<summary>System Info</summary>

### `get_sys_info() -> SystemInfo`

Retrieves the current system information.

```typescript
interface SystemInfo {
  bootTime?: number | null;
  cpuNum?: number | null;
  cpuSpeed?: number | null;
  diskTotal?: number | null;
  diskFree?: number | null;
  memoryTotal?: number | null;
  memoryFree?: number | null;
  hostname?: string | null;
  avgLoad?: number | null;
  osRelease?: string | null;
  osType?: string | null;
  procTotal?: number | null;
}
```

</details>

<details>
<summary>Media Activity (macOS Only)</summary>

### `register_media_activity_event()`

Starts emitting media activity events to the editor.

### `unregister_media_activity_event()`

Stops emitting media activity events.

### `play_media()`

Plays the current media.

### `pause_media()`

Pauses the current media.

### `next_track()`

Skips to the next track.

### `prev_track()`

Returns to the previous track.

### Media Activity Event

When listening to media activity, the event name is `media-activity`, and it emits data in the following format:

```typescript
interface MediaActivity {
  infoUpdateTime?: number | null;
  isPlaying?: boolean | null;
  title?: string | null;
  artist?: string | null;
  appName?: string | null;
  appIcon?: string | null;
  album?: string | null;
  albumCover?: string | null;
  duration?: number | null;
  elapsed?: number | null;
}
```

</details>

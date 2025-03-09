# RPi WidgetBox Desktop

This repository serves as the desktop editor for [RPi-WidgetBox](https://github.com/nohackjustnoobb/RPi-WidgetBox). It provides system-level APIs that plugins can utilize.

## API Reference

**Note:** You should use the `@tauri-apps/api` to invoke these methods or listen to events.

<details>
<summary>System Info</summary>

### `get_sys_info() -> SystemInfo`

Retrieves the current system information.

```typescript
interface MemoryInfo {
  total: number;
  used: number;
}

interface CPUInfo {
  usage: number;
  name: string;
  brand: string;
  vendorId: string;
  frequency: number;
}

interface SystemInfo {
  bootTime: number;
  cpus: CPUInfo[];
  memory: MemoryInfo;
  swap: MemoryInfo;
  hostName?: string | null;
  osVersion?: string | null;
  longOsVersion?: string | null;
  systemName?: string | null;
  kernelVersion?: string | null;
  totalProc: number;
}
```

### `get_disks_info() -> Vec<DiskInfo>`

Retrieves the current disks information.

```typescript
interface DiskInfo {
  kind: string;
  name?: string | null;
  fileSystem?: string | null;
  mountPoint?: string | null;
  totalSpace: number;
  availableSpace: number;
}
```

### `get_networks_info() -> Vec<NetworkInfo>`

Retrieves the current networks information.

```typescript
interface NetworkInfo {
  name: string;
  totalReceived: number;
  totalTransmitted: number;
  totalPacketsReceived: number;
  totalPacketsTransmitted: number;
  totalErrorsOnReceived: number;
  totalErrorsOnTransmitted: number;
  macAddress: string;
  ipNetworks: string[];
  mtu: number;
}
```

### `get_components_info() -> Vec<ComponentInfo>`

Retrieves the current component's temperature information.

```typescript
interface ComponentInfo {
  label: string;
  temperature?: number | null;
  max?: number | null;
  critical?: number | null;
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

`NonSendMarker`, a type used to force systems to run on the main thread, is now a system parameter. This means that it no longer needs to be wrapped in `Option<NonSend<_>>`. Furthermore, `NonSendMarker` has been moved from `bevy::core` to `bevy::ecs::system`, so please update your imports accordingly.

```rust
// 0.15
use bevy::core::NonSendMarker;

fn my_system(_: Option<NonSend<NonSendMarker>>) {
    // ...
}

// 0.16
use bevy::ecs::system::NonSendMarker;

fn my_system(_: NonSendMarker) {
    // ...
}
```

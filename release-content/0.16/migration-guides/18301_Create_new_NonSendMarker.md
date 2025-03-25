If `NonSendMarker` is being used from `bevy_app::prelude::*`, replace it with `bevy_ecs::system::NonSendMarker` or use it from `bevy_ecs::prelude::*`. In addition to that, `NonSendMarker` does not need to be wrapped like so:

```rust
fn my_system(_non_send_marker: Option<NonSend<NonSendMarker>>) {
    ...
}
```

Instead, it can be used without any wrappers:

```rust
fn my_system(_non_send_marker: NonSendMarker) {
    ...
}
```

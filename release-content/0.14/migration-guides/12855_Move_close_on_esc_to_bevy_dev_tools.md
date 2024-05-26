`bevy_window::close_on_esc` has been moved to `bevy_dev_tools::close_on_esc`. You will first need to enable `bevy_dev_tools` as a feature in your `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "0.14", features = ["bevy_dev_tools"] }
```

Finally, modify any imports to use `bevy_dev_tools` instead:

```rust
// Old:
// use bevy::window::close_on_esc;

// New:
use bevy::dev_tools::close_on_esc;

App::new()
    .add_systems(Update, close_on_esc)
    // ...
    .run();
```

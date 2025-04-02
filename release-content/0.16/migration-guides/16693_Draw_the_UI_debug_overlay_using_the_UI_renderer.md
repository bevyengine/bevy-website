The `bevy_dev_tools::ui_debug_overlay` module has been replaced with a new debug overlay implemented using `bevy_ui`'s renderer. The new debug UI overlay still requires the `bevy_ui_debug` feature flag, but this flag is now available through `bevy` and `bevy_ui` instead of `bevy_dev_tools`. `UiDebugOptions` has been moved to `bevy_ui` as well, and now has several new options.

```rust
// 0.15
App::new()
    .add_plugins((DefaultPlugins, DebugUiPlugin))
    .insert_resource(UiDebugOptions {
        enabled: true,
    })
    .run();

// 0.16
App::new()
    // You no longer need `DebugUiPlugin`; enabling the `bevy_ui_debug` feature handles this for
    // you.
    .add_plugins(DefaultPlugins)
    .insert_resource(UiDebugOptions {
        enabled: true,
        // `UiDebugOptions` has a few new options, but for now we'll leave the defaults.
        ..default()
    })
    .run();
```

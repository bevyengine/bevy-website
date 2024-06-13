Dynamic plugins are now deprecated. If possible, remove all usage them from your code:

```rust
// 0.13
// This would be compiled into a separate dynamic library.
#[derive(DynamicPlugin)]
pub struct MyPlugin;

impl Plugin for MyPlugin {
    // ...
}

// This would be compiled into the main binary.
App::new()
    .load_plugin("path/to/plugin")
    .run();

// 0.14
// This would now be compiled into the main binary as well.
pub struct MyPlugin;

impl Plugin for MyPlugin {
    // ...
}

App::new()
    .add_plugins(MyPlugin)
    .run();
```

If you are unable to do that, you may temporarily silence the deprecation warnings by annotating all usage with `#[allow(deprecated)]`. Please note that the current dynamic plugin system will be removed by the next major Bevy release, so you will have to migrate eventually. You may be interested in these safer, related links:

- [Bevy Assets - Scripting](https://bevyengine.org/assets/#scripting): Scripting and modding libraries for Bevy
- [Bevy Assets - Development tools](https://bevyengine.org/assets/#development-tools): Hot reloading and other development functionality
- [`stabby`](https://github.com/ZettaScaleLabs/stabby): Stable Rust ABI

If you truly cannot go without dynamic plugins, you may copy the code from Bevy and add it to your project locally.

If possible, remove all usage of dynamic plugins.

```rust
// Old
#[derive(DynamicPlugin)]
pub struct MyPlugin;

App::new()
    .load_plugin("path/to/plugin")
    .run();

// New
pub struct MyPlugin;

App::new()
    .add_plugins(MyPlugin)
    .run();
```

If you are unable to do that, you may temporarily silence the deprecation warnings.

```rust
#[allow(deprecated)]
```

Please note that the current dynamic plugin system will be removed by the next major Bevy release, so you will have to migrate eventually. You may be interested in these safer alternatives:

- [Bevy Assets - Scripting](https://bevyengine.org/assets/#scripting): Scripting and modding libraries for Bevy
- [Bevy Assets - Development tools](https://bevyengine.org/assets/#development-tools): Hot reloading and other development functionality
- [`stabby`](https://github.com/ZettaScaleLabs/stabby): Stable Rust ABI

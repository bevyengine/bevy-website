`AssetMetaCheck` is used to configure how the `AssetPlugin` reads `.meta` files. It was previously a resource, but now has been changed to a field in `AssetPlugin`. If you use `DefaultPlugins`, you can use `.set` to configure this field.

```rust
// 0.13
App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(AssetMetaCheck::Never)
    .run()

// 0.14
App::new()
    .add_plugins(DefaultPlugins.set(AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
    }))
    .run()
```

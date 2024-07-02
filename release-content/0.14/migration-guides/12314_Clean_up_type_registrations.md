Many external types are no longer registered into the type registry by Bevy's default plugin. Generally, only those types used by other Bevy types (due to the new recursive registration) will be registered by default. If you were using reflection features with types from `std` or `glam` you may need to manually register them.

```rust
App::new().register_type::<DMat3>();
```

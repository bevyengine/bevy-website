Removed second generic from `PluginGroupBuilder` methods: `add_before` and `add_after`.

```rust
// Before:
DefaultPlugins
    .build()
    .add_before::<WindowPlugin, _>(FooPlugin)
    .add_after::<WindowPlugin, _>(BarPlugin)

// After:
DefaultPlugins
    .build()
    .add_before::<WindowPlugin>(FooPlugin)
    .add_after::<WindowPlugin>(BarPlugin)
```

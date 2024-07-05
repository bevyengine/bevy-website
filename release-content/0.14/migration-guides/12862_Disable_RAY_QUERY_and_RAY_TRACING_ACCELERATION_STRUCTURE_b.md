The `RAY_QUERY` and `RAY_TRACING_ACCELERATION_STRUCTURE` `wgpu` features are now disabled by default, due to some users having their program crash while initializing. (The `wgpu` issue for this can be found [here](https://github.com/gfx-rs/wgpu/issues/5488).)

If you use these features, you will need to re-enable them through `WgpuSettings::features`:

```rust
let mut settings = WgpuSettings::default();

// Enable `RAY_QUERY` and `RAY_TRACING_ACCELERATION_STRUCTURE`, along with the defaults.
settings.features |= WgpuFeatures::RAY_QUERY | WgpuFeatures::RAY_TRACING_ACCELERATION_STRUCTURE;

App::new()
    .add_plugins(DefaultPlugins.set(RenderPlugin {
        render_creation: settings.into(),
        ..default()
    }))
    .run()
```

Note that `WgpuSettings::default()` automatically configures good default flags for Bevy, while `WgpuFeatures::default()` is the equivalent of `WgpuFeatures::empty()`.

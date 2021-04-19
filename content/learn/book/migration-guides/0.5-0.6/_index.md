+++
title = "0.5 to 0.6"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
[extra]
long_title = "Migration Guide: 0.5 to 0.6"
+++

### `Light` and `LightBundle` are now `PointLight` and `PointLightBundle`

```rust
// 0.5
commands.spawn_bundle(LightBundle {
    light: Light {
        color: Color::YELLOW,
        fov: f32::to_radians(60.0),
        depth: 0.1..50.0,
        intensity: 200.0,
        range: 20.0,
    },
    ..Default::default()
});

// 0.6
commands.spawn_bundle(PointLightBundle {
    point_light: PointLight {
        color: Color::YELLOW,
        intensity: 200.0,
        range: 20.0,
    },
    ..Default::default()
});
```

Cleanup on the {{rust_type(type="struct" crate="bevy_pbr" mod="light" version="0.5.0" name="PointLight" no_mod=true)}},
removing unused fields `fov` and `depth`.

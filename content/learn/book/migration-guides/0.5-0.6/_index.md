+++
title = "0.5 to 0.6"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
[extra]
long_title = "Migration Guide: 0.5 to 0.6"
+++

### Calling ".system()" on a system is now optional

```rust
// 0.5
fn main() {
    App::build()
        .add_system(first_system.system())
        .add_system(second_system.system())
        .run();
}

// 0.6
fn main() {
    App::build()
        .add_system(first_system)
        .add_system(second_system.system())
        .run();
}
```

When adding a system to Bevy it is no longer necessary to call `.system()` beforehand.
Functions like `.label()` or `.config()` can now also be directly called on a system.

```rust
// 0.5
fn main() {
    App::build()
        .add_system(first_system.system().label("FirstSystem"))
        .add_system(second_system.system().after("FirstSystem"))
        .run();
}

// 0.6
fn main() {
    App::build()
        .add_system(first_system.label("FirstSystem"))
        .add_system(second_system.after("FirstSystem"))
        .run();
}
```

### "Light" and "LightBundle" are now "PointLight" and "PointLightBundle"

```rust
// 0.5
commands.spawn_bundle(LightBundle {
    light: Light {
        color: Color::rgb(1.0, 1.0, 1.0),
        depth: 0.1..50.0,
        fov: f32::to_radians(60.0),
        intensity: 200.0,
        range: 20.0,
    }
    ..Default::default()
});

// 0.6
commands.spawn_bundle(PointLightBundle {
    light: PointLight {
        color: Color::rgb(1.0, 1.0, 1.0),
        intensity: 200.0,
        range: 20.0,
    }
    ..Default::default()
});
```

The {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.5.0" name="Light" no_mod=true)}} and {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.5.0" name="LightBundle" no_mod=true)}} were renamed to {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="PointLight" no_mod=true)}} and {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="PointLightBundle" no_mod=true)}} to more clearly communicate the behavior of the Light Source.
At the same time the `fov` and `depth` fields were removed from the {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="PointLight" no_mod=true)}} as they were unused.

<!-- TODO: Remove this comment if https://github.com/bevyengine/bevy/pull/2367 is merged.

In addition the {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="DirectionalLight" no_mod=true)}} and {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="DirectionalLightBundle" no_mod=true)}} were introduced with `0.6`.

-->

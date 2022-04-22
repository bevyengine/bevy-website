+++
title = "0.5 to 0.6"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.5 to 0.6"
+++

### Rust 2021 now required

Bevy has been updated to use Rust 2021. This means we can take advantage of the new Cargo feature resolver by default (which both Bevy and the new wgpu version require). Make sure you update your crates to Rust 2021 or you will need to manually enable the new feature resolver with `resolver = "2" in your Cargo.toml.

```toml
[package]
name = "your_app"
version = "0.1.0"
edition = "2021"
```

Note that "virtual Cargo workspaces" still need to manually define `resolver = "2"`, even in Rust 2021. [Refer to the Rust 2021 documentation](https://doc.rust-lang.org/edition-guide/rust-2021/default-cargo-resolver.html#details) for details.

```toml
[workspace]
resolver = "2" # Important! wgpu/Bevy needs this!
members = [ "my_crate1", "my_crate2" ]
```

### "AppBuilder" was merged into "App"

All functions of [`AppBuilder`] were merged into [`App`].

In practice this means that you start constructing an [`App`] by calling [`App::new`] instead of [`App::build`] and [`Plugin::build`] takes a [`App`] instead of a [`AppBuilder`].

```rs
// 0.5
fn main() {
    App::build()
        .add_plugin(SomePlugin)
        .run();
}

impl Plugin for SomePlugin {
    fn build(&self, app: &mut AppBuilder) {

    }
}

// 0.6
fn main() {
    App::new()
        .add_plugin(SomePlugin)
        .run();
}

impl Plugin for SomePlugin {
    fn build(&self, app: &mut App) {

    }
}
```

[`AppBuilder`]: https://docs.rs/bevy/0.5.0/bevy/app/struct.AppBuilder.html
[`App`]: https://docs.rs/bevy/0.6.0/bevy/app/struct.App.html
[`App::new`]: https://docs.rs/bevy/0.6.0/bevy/app/struct.App.html#method.new
[`App::build`]: https://docs.rs/bevy/0.5.0/bevy/app/struct.App.html#method.build
[`Plugin::build`]: https://docs.rs/bevy/0.6.0/bevy/app/trait.Plugin.html#tymethod.build

### The "Component" trait now needs to be derived

Bevy no longer has a blanket implementation for the [`Component`] trait.
Instead you need to derive (or manualy implement) the trait for every Type that needs it.

```rust
// 0.5
struct MyComponent;

// 0.6
#[derive(Component)]
struct MyComponent;
```

In order to use foreign types as components, wrap them using the newtype pattern.

```rust
#[derive(Component)]
struct Cooldown(std::time::Duration);
```

[`Component`]: https://docs.rs/bevy/0.6.0/bevy/ecs/component/trait.Component.html

### Setting the Component Storage is now done in "Component" Trait

The change to deriving [`Component`], enabled setting the Component Storage at compiletime instead of runtime.

```rust
// 0.5
appbuilder
    .world
    .register_component(ComponentDescriptor::new::<MyComponent>(
        StorageType::SparseSet,
    ))
    .unwrap();

// 0.6
#[derive(Component)]
#[component(storage = "SparseSet")]
struct MyComponent;
```

### Calling ".system()" on a system is now optional

When adding a system to Bevy it is no longer necessary to call `.system()` beforehand.

```rust
// 0.5
fn main() {
    App::new()
        .add_system(first_system.system())
        .add_system(second_system.system())
        .run();
}

// 0.6
fn main() {
    App::new()
        .add_system(first_system)
        .add_system(second_system.system())
        .run();
}
```

System configuration Functions like `.label()` or `.config()` can now also be directly called on a system.

```rust
// 0.5
fn main() {
    App::new()
        .add_system(first_system.system().label("FirstSystem"))
        .add_system(second_system.system().after("FirstSystem"))
        .run();
}

// 0.6
fn main() {
    App::new()
        .add_system(first_system.label("FirstSystem"))
        .add_system(second_system.after("FirstSystem"))
        .run();
}
```

### ".single()" and ".single_mut()" are now infallible

The functions [`Query::single()`] and [`Query::single_mut()`] no longer return a `Result` and instead panic unless exactly one entity was found.

If you need the old behavior you can use the fallible [`Query::get_single()`] and [`Query_get_single_mut()`] instead.

```rs
// 0.5
fn player_system(query: Query<&Transform, With<Player>>) {
    let player_position = query.single().unwrap();
    // do something with player_position
}

// 0.6
fn player_system_infallible(query: Query<&Transform, With<Player>>) {
    let player_position = query.single();
    // do something with player_position
}

fn player_system_fallible(query: Query<&Transform, With<Player>>) {
    let player_position = query.get_single().unwrap();
    // do something with player_position
}
```

[`Query::single()`]: https://docs.rs/bevy/0.6.0/bevy/ecs/system/struct.Query.html#method.single
[`Query::single_mut()`]: https://docs.rs/bevy/0.6.0/bevy/ecs/system/struct.Query.html#method.single_mut
[`Query::get_single`]: https://docs.rs/bevy/0.6.0/bevy/ecs/system/struct.Query.html#method.get_single
[`Query_get_single_mut`]: https://docs.rs/bevy/0.6.0/bevy/ecs/system/struct.Query.html#method.get_single_mut

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
    },
    ..Default::default()
});

// 0.6
commands.spawn_bundle(PointLightBundle {
    light: PointLight {
        color: Color::rgb(1.0, 1.0, 1.0),
        intensity: 200.0,
        range: 20.0,
    },
    ..Default::default()
});
```

The [`Light`] and [`LightBundle`] types were renamed to [`PointLight`] and [`PointLightBundle`] to more clearly communicate the behavior of the Light Source.
At the same time the `fov` and `depth` fields were removed from [`PointLight`] as they were unused.

[`Light`]: https://docs.rs/bevy/0.5.0/bevy/pbr/struct.Light.html
[`LightBundle`]: https://docs.rs/bevy/0.5.0/bevy/pbr/struct.LightBundle.html
[`PointLight`]: https://docs.rs/bevy/0.6.0/bevy/pbr/struct.PointLight.html
[`PointLightBundle`]: https://docs.rs/bevy/0.6.0/bevy/pbr/struct.PointLightBundle.html

### System Param Lifetime Split

The Lifetime of [`SystemParam`] was split in two separate Lifetimes.

```rust
// 0.5
type SystemParamAlias<'a> = (Res<'a, AssetServer>, Query<'a, &'static Transform>, Local<'a, i32>);

#[derive(SystemParam)]
struct SystemParamDerive<'a> {
    res: Res<'a, AssetServer>,
    query: Query<'a, &Transform>,
    local: Local<'a, i32>,
}

// 0.6
type SystemParamAlias<'w, 's> = (Res<'w, AssetServer>, Query<'w, 's, &'static Transform>, Local<'s, i32>);

#[derive(SystemParam)]
struct SystemParamDerive<'w, 's> {
    res: Res<'w, AssetServer>,
    query: Query<'w, 's, &'static Transform>,
    local: Local<'s, i32>,
}
```

[`SystemParam`]: https://docs.rs/bevy/0.6.0/bevy/ecs/system/trait.SystemParam.html

### QuerySet declare "QueryState" instead of "Query"

Due to the [System Param Lifetime Split](#system-param-lifetime-split), [`QuerySet`] system parameters now need to specify their Queries with [`QuerySet`] instead of [`Query`].

```rust
// 0.5
fn query_set(mut queries: QuerySet<(Query<&mut Transform>, Query<&Transform>)>) {

}

// 0.6
fn query_set(mut queries: QuerySet<(QueryState<&mut Transform>, QueryState<&Transform>)>) {

}
```

[`QuerySet`]: https://docs.rs/bevy/0.6.0/bevy/ecs/system/struct.QuerySet.html
[`QueryState`]: https://docs.rs/bevy/0.6.0/bevy/ecs/query/struct.QueryState.html
[`Query`]: https://docs.rs/bevy/0.6.0/bevy/ecs/system/struct.Query.html

### "Input\<T\>.update()" is renamed to "Input\<T\>.clear()"

The [`Input::update`] function was renamed to [`Input::clear`].

[`Input::update`]: https://docs.rs/bevy/0.5.0/bevy/input/struct.Input.html#method.update
[`Input::clear`]: https://docs.rs/bevy/0.6.0/bevy/input/struct.Input.html#method.clear

### "SystemState" is now "SystemMeta"

The struct formerly known as [`SystemState`](https://docs.rs/bevy/0.5.0/bevy/ecs/system/struct.SystemState.html), which stores the metadata of a System, was renamed to [`SystemMeta`].

This was done to accommodate the new [`SystemState`] which allows easier cached access to [`SystemParam`] outside of a regular System.

[`SystemState`]: https://docs.rs/bevy/0.6.0/bevy/ecs/system/struct.SystemState.html
[`SystemMeta`]: https://docs.rs/bevy/0.6.0/bevy/ecs/system/struct.SystemMeta.html

<!-- TODO: Link to entry for SystemState in the release blog post. -->

### Vector casting functions are now named to match return type

The casting functions for [`IVec2`], [`DVec2`], [`UVec2`], and [`Vec2`] have all been changed from being named after their inner elements' cast target to what the entire "Vec" is being casted into. This affects all the different dimensions of the math vectors (i.e., [`Vec2`], [`Vec3`] and [`Vec4`]).

```rust
// 0.5
let xyz: Vec3 = Vec3::new(0.0, 0.0, 0.0);
let xyz: IVec3 = xyz.as_i32();

// 0.6
let xyz: Vec3 = Vec3::new(0.0, 0.0, 0.0);
let xyz: IVec3 = xyz.as_ivec3();
```

 [`IVec2`]: https://docs.rs/bevy/0.6.0/bevy/math/struct.IVec2.html
 [`DVec2`]: https://docs.rs/bevy/0.6.0/bevy/math/struct.DVec2.html
 [`UVec2`]: https://docs.rs/bevy/0.6.0/bevy/math/struct.UVec2.html
 [`Vec2`]: https://docs.rs/bevy/0.6.0/bevy/math/struct.Vec2.html
 [`Vec3`]: https://docs.rs/bevy/0.6.0/bevy/math/struct.Vec3.html
 [`Vec4`]: https://docs.rs/bevy/0.6.0/bevy/math/struct.Vec4.html

### StandardMaterial's "roughness" is renamed to "perceptual_roughness"

The [`StandardMaterial`] field `roughness` was renamed to `perceptual_roughness`.

[`StandardMaterial`]: https://docs.rs/bevy/0.6.0/bevy/pbr/struct.StandardMaterial.html

### SpriteBundle and Sprite

The [`SpriteBundle`] bundle type now uses a `texture` handle rather than a `material`. The `color` field of the material is now directly available inside of the [`Sprite`] struct, which also had its `resize_mode` field replaced with a `custom_size`. The following example shows how to spawn a tinted sprite at a particular size. For simpler cases, check out the updated [sprite](https://github.com/bevyengine/bevy/blob/v0.6.0/examples/2d/sprite.rs) and [rect](https://github.com/bevyengine/bevy/blob/v0.6.0/examples/2d/rect.rs) examples.

```rust
// 0.5
SpriteBundle {
    sprite: Sprite {
        size: Vec2::new(256.0, 256.0),
        resize_mode: SpriteResizeMode::Manual,
        ..Default::default()
    },
    material: materials.add(ColorMaterial {
        color: Color::RED,
        texture: Some(asset_server.load("branding/icon.png")),
    }),
    ..Default::default()
}

// 0.6
SpriteBundle {
    sprite: Sprite {
        custom_size: Some(Vec2::new(256.0, 256.0)),
        color: Color::RED,
        ..Default::default()
    },
    texture: asset_server.load("branding/icon.png"),
    ..Default::default()
}
```

[`SpriteBundle`]: https://docs.rs/bevy/0.6.0/bevy/sprite/struct.SpriteBundle.html
[`Sprite`]: https://docs.rs/bevy/0.6.0/bevy/sprite/struct.Sprite.html

### Visible is now Visibility

The [`Visible`] struct, which is used in a number of components to set visibility, was renamed to [`Visibility`]. Additionally, the field `is_transparent` was removed from the struct. For 3D, transparency can be set using the `alpha_mode` field on a material. Transparency is now automatically enabled for all objects in 2D.

```rust
// 0.5
let material_handle = materials.add(StandardMaterial {
    base_color_texture: Some(texture.clone()),
    ..Default::default()
});

commands.spawn_bundle(PbrBundle {
    material: material_handle,
    visible: Visible {
        is_visible: true,
        is_transparent: true,
    },
    ..Default::default()
});

// 0.6
let material_handle = materials.add(StandardMaterial {
    base_color_texture: Some(texture.clone()),
    alpha_mode: AlphaMode::Blend,
    ..Default::default()
});

commands.spawn_bundle(PbrBundle {
    material: material_handle,
    visibility: Visibility {
        is_visible: true,
    },
    ..Default::default()
});
```

[`Visible`]: https://docs.rs/bevy/0.5.0/bevy/prelude/struct.Visible.html
[`Visibility`]: https://docs.rs/bevy/0.6.0/bevy/prelude/struct.Visibility.html#

+++
title = "0.5 to 0.6"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
[extra]
long_title = "Migration Guide: 0.5 to 0.6"
+++

### "AppBuilder" was merged into "App"

All functions of {{rust_type(type="struct" crate="bevy_app" mod="" version="0.5.0" name="AppBuilder" no_mod=true)}} were merged into {{rust_type(type="struct" crate="bevy_app" mod="" version="0.6.0" name="App" no_mod=true)}}.

In practice this means that you start constructing an {{rust_type(type="struct" crate="bevy_app" mod="" version="0.6.0" name="App" no_mod=true)}} by calling {{rust_type(type="struct" crate="bevy_app" mod="" version="0.6.0" name="App" no_mod=true method="new")}} instead of {{rust_type(type="struct" crate="bevy_app" mod="" version="0.5.0" name="App" no_mod=true method="build")}} and {{rust_type(type="trait" crate="bevy_app" mod="" version="0.5.0" name="Plugin" no_mod=true method="build")}} takes a {{rust_type(type="struct" crate="bevy_app" mod="" version="0.6.0" name="App" no_mod=true)}} instead of a {{rust_type(type="struct" crate="bevy_app" mod="" version="0.5.0" name="AppBuilder" no_mod=true)}}

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

### The "Component" trait now needs to be derived

Bevy no longer has a blanket implementation for the {{rust_type(type="trait" crate="bevy_ecs" mod="component" version="0.6.0" name="Component" no_mod=true)}} trait.
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

### Setting the Component Storage is now done in "Component" Trait

The change to deriving {{rust_type(type="trait" crate="bevy_ecs" mod="component" version="0.6.0" name="Component" no_mod=true)}}, enabled setting the Component Storage at compiletime instead of runtime.

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

The functions {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.6.0" name="Query" no_mod=true method="single")}} and {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.6.0" name="Query" no_mod=true method="single_mut")}} no longer return a {{rust_type(type="enum", crate="std" mod="result", name="Result", no_mod=true)}} and Panic instead, if not exactly one Entity was found.

If you need the old behavior you can use the fallible {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.6.0" name="Query" no_mod=true method="get_single")}} and {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.6.0" name="Query" no_mod=true method="get_single_mut")}} instead.

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

The {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.5.0" name="Light" no_mod=true)}} and {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.5.0" name="LightBundle" no_mod=true)}} were renamed to {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="PointLight" no_mod=true)}} and {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="PointLightBundle" no_mod=true)}} to more clearly communicate the behavior of the Light Source.
At the same time the `fov` and `depth` fields were removed from the {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="PointLight" no_mod=true)}} as they were unused.

<!-- TODO: Remove this comment if https://github.com/bevyengine/bevy/pull/2367 is merged.

In addition the {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="DirectionalLight" no_mod=true)}} and {{rust_type(type="struct" crate="bevy_pbr" mod="" version="0.6.0" name="DirectionalLightBundle" no_mod=true)}} were introduced with `0.6`.

-->

### System Param Lifetime Split

The Lifetime of {{rust_type(type="trait" crate="bevy_ecs" mod="system" version="0.5.0" name="SystemParam" no_mod=true)}} was split in two separate Lifetimes.

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

### QuerySet declare "QueryState" instead of "Query"
<!-- Adapt for ParamSet instead, if https://github.com/bevyengine/bevy/pull/2765 is merged -->

Due to the [System Param Lifetime Split](#system-param-lifetime-split), {{rust_type(type="struct" crate="bevy_ecs" mod="system" name="QuerySet" version="0.6.0" no_mod=true plural=true)}} now need to specify their Queries with {{rust_type(type="struct" crate="bevy_ecs" mod="query" version="0.6.0" name="QueryState" no_mod=true)}} instead of {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.6.0" name="Query" no_mod=true)}}.

```rust
// 0.5
fn query_set(mut queries: QuerySet<(Query<&mut Transform>, Query<&Transform>)>) {

}

// 0.6
fn query_set(mut queries: QuerySet<(QueryState<&mut Transform>, QueryState<&Transform>)>) {

}
```

### "Input\<T\>.update()" is renamed to "Input\<T\>.clear()"

The {{rust_type(type="struct" crate="bevy_input" mod="" version="0.5.0" name="Input" no_mod=true method="update")}} function was renamed to {{rust_type(type="struct" crate="bevy_input" mod="" version="0.6.0" name="Input" no_mod=true method="clear")}}.

### "SystemState" is now "SystemMeta"

The {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.5.0" name="SystemState" no_mod=true)}} struct, which stores the metadata of a System, was renamed to {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.6.0" name="SystemMeta" no_mod=true)}}.

This was done to accommodate the new {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.6.0" name="SystemState" no_mod=true)}} which allows easier cached access to {{rust_type(type="trait" crate="bevy_ecs" mod="system" version="0.6.0" name="SystemParam" no_mod=true plural=true)}} outside of a regular System.
<!-- TODO: Link to entry for SystemState in the release blog post. -->

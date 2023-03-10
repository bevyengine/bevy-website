+++
title = "0.4 to 0.5"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.4 to 0.5"
+++

<!-- TODO: link to release blog post here -->

### "commands: &mut Commands" SystemParam is now "mut commands: Commands"

```rust
// 0.4
fn foo(commands: &mut Commands) {
}

// 0.5
fn foo(mut commands: Commands) {
}
```

Systems using the old `commands: &mut Commands` syntax in 0.5 will fail to compile when calling `foo.system()`.

This change was made because {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.5.0" name="Commands" no_mod=true)}}
now holds an internal {{rust_type(type="struct" crate="bevy_ecs" mod="world" version="0.5.0" name="World" no_mod=true)}}
reference to enable safe entity allocations.

Note: The internal {{rust_type(type="struct" crate="bevy_ecs" mod="world" version="0.5.0" name="World" no_mod=true)}} reference requires two lifetime parameters to pass Commands into a non-system function: `commands: &'a mut Commands<'b>`

### Commands API

The {{rust_type(type="struct" crate="bevy_ecs" version="0.5.0" mod="system" name="Commands" no_mod=true)}} API has been completely reworked for consistency with the {{rust_type(type="struct" crate="bevy_ecs" mod="world" version="0.5.0" name="World" no_mod=true)}} API.

```rust
// 0.4
commands
    .spawn(SomeBundle)
    .with(SomeComponent)
    .spawn(SomeBundle); // this sort of chaining is no longer possible

let entity = commands.spawn(SomeBundle).current_entity().unwrap();

commands.despawn(entity);

// 0.5
commands
    .spawn()
    .insert_bundle(SomeBundle)
    .insert(Component);

let entity = commands.spawn().insert_bundle(SomeBundle).id();

commands.entity(entity).despawn();
```

`commands.spawn()` no longer accepts any parameters.  To spawn bundles, use `commands.spawn_bundle(bundle)`.

Similarly, rather than using `with(some_component)` to spawn an object with multiple components, you must now use `insert(some_component)`:

```rust
// 0.4
commands.spawn(some_bundle)
    .with(some_component);
    
// 0.5
commands.spawn_bundle(some_bundle)
    .insert(some_component);
    
// or...
commands.spawn()
    .insert_bundle(some_bundle)
    .insert(some_component);
```

Removing and adding components on entities has also been changed:

```rust
// 0.4
commands.insert_one(some_entity, SomeComponent);
commands.remove_one::<SomeComponent>(some_entity);

// 0.5
commands.entity(some_entity).insert(SomeComponent);
commands.entity(some_entity).remove::<SomeComponent>();
```

### Timer now uses Duration

```rust
// 0.4
if timer.tick(time.delta_seconds()).finished() { /* do stuff */ }
timer.elapsed() // returns an `f32`

// 0.5
if timer.tick(time.delta()).finished() { /* do stuff */ }
timer.elapsed() // returns a `Duration`
```

Most of the methods of {{rust_type(type="struct" crate="bevy_core" version="0.5.0" name="Timer" no_mod=true)}}
now use `Duration` instead of `f32`.

This change allows timers to have consistent, high precision. For convenience, there is also an
`elapsed_secs` method that returns `f32`.  Otherwise, when you need an `f32`, use the
`as_secs_f32()` method on `Duration` to make the conversion.

### Simplified Events

```rust
// 0.4
fn event_reader_system(
    mut my_event_reader: Local<EventReader<MyEvent>>,
    my_events: Res<Events<MyEvent>>,
) {
    for my_event in my_event_reader.iter(&my_events) {
        // do things with your event
    }
}

// 0.5
fn event_reader_system(mut my_event_reader: EventReader<MyEvent>) {
    for my_event in my_event_reader.iter() {
        // do things with your event
    }
}
```

You no longer need two system parameters to read your events. One `EventReader` is sufficient.

Following the above example of using an `EventReader` to read events, you can now use `EventWriter` to create new ones.

```rust
// 0.4
fn event_writer_system(
    mut my_events: ResMut<Events<MyEvent>>,
) {
    my_events.send(MyEvent);
}

// 0.5
fn event_writer_system(
    mut my_events: EventWriter<MyEvent>
) {
    my_events.send(MyEvent);
}
```

### AppBuilder::add_resource is now called AppBuilder::insert_resource

This is a small change to have function names on `AppBuilder` consistent with the `Commands` API.

### TextBundle

This bundle has been reworked to allow multiple differently-styled sections of text within a single bundle. `Text::with_section` was added to simplify the common case where you're only interested in one text section.

```rust
// 0.4
TextBundle {
    text: Text {
        value: "hello!".to_string(),
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        style: TextStyle {
            font_size: 60.0,
            color: Color::WHITE,
            ..Default::default()
        },
    },
    ..Default::default()
}

// 0.5
TextBundle {
    text: Text::with_section(
        "hello!",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 60.0,
            color: Color::WHITE,
        },
        TextAlignment::default()
    ),
    ..Default::default()
}
```

### Scene must now be specified when loading a GLTF scene

Previously, you were able to load a GLTF scene asset with only a path. Now, you must include a fragment specifying the scene you want to load. If you only have one scene in the file, it's `#Scene0`.

```rust
// 0.4
asset_server.load("models/foo.glb");

// 0.5
asset_server.load("models/foo.glb#Scene0");
```

### State

States are now registered with `AppBuilder::add_state`, which creates
the `State` resource and registers a "driver" system that takes the
place of `StateStage`.  States are registered using `SystemSet`.

**IMPORTANT**: if you stop registering the `StateStage` but don't
register the driver (using `add_state` or `State::get_driver`), Bevy
0.5 will enter an infinite loop, causing your application to "lock up".

```rust
// 0.4
app.insert_resource(State::new(MyState::InitState))
   .add_stage_after(
       bevy::app::stage::UPDATE,
       MY_STATE_STAGE_NAME,
       bevy::ecs::StateStage::<MyState>::default(),
   )
   .on_state_enter(
       MY_STATE_STAGE_NAME,
       MyState::InitState,
       enter_init_state.system())
   .on_state_update(
       MY_STATE_STAGE_NAME,
       MyState::InitState,
       update_init_state.system())
   .on_state_exit(
       MY_STATE_STAGE_NAME,
       MyState::InitState,
       exit_init_state.system());

// 0.5
app.add_state(MyState::InitState)
   .add_system_set(SystemSet::on_enter(MyState::InitState)
       .with_system(enter_init_state.system()))
   .add_system_set(SystemSet::on_update(MyState::InitState)
       .with_system(update_init_state.system()))
   .add_system_set(SystemSet::on_exit(MyState::InitState)
       .with_system(exit_init_state.system()));
```

It is still possible to register the driver manually using
`State::get_driver`, but this is not normally required.

## ChangedRes removed

This change was made to allow for more flexiblity and more consistent behavior with change detection for components.

```rust
// 0.4
fn some_system(
    res: ChangedRes<SomeResource>
) {
    // this system only runs if SomeResource has changed
}

// 0.5
fn some_system(
    res: Res<SomeResource> // or ResMut
) {
    // this system always runs

    if !res.is_changed() { // or .is_added()
        return;
    }
}
```

### Cameras

`Camera3dBundle` is now known as `PerspectiveCameraBundle`, and `Camera2dBundle` is now known as `OrthographicCameraBundle`.

`OrthographicCameraBundle` does not implement `Default`, so to change its transform at spawn while keeping everything else the same, consider something like the following:

```rust
let mut camera = OrthographicCameraBundle::new_2d();
camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
commands.spawn_bundle(camera);
```

### Render API changes

`RasterizationStateDescriptor` no longer exists.  Much of its functionality has been moved to other fields on `PipelineDescriptor`.  `cull_mode`, for example, is now found in the `primitive: PrimitiveState` field.

Buffers of type `Vec<Color>` can no longer be uploaded to the GPU directly due to limitations with `RenderResources` and the new `Byteable` requirement.  Consider using a `Vec<Vec4>` instead, and inserting colors with `as_rgba_f32()` and `.into()` instead:

```rust
#[derive(RenderResources, Default, TypeUuid)]
struct SomeShader {
    #[render_resources(buffer)]
    pub colors: Vec<Vec4>
}

fn add_some_color(shader: SomeShader, color: Color) {
    shader.colors.push(color.as_rgba_f32().into());
}
```

### Shaders should now use `CameraViewProj`

The `ViewProj` matrix is now set via the name `CameraViewProj` rather than `Camera`.  If you don't update this, bevy will fail silently and you won't be able to see anything!

```glsl
// 0.4
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
}

// 0.5
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
}
```

### Diagnostics

`PrintDiagnosticsPlugin` is now `LogDiagnosticsPlugin`.

### System Ordering

The parallel system executor has been redesigned. Systems that had implicit orderings might no longer run in the same order. For more detail on the new behavior, see [the release notes](/news/bevy-0-5#new-parallel-system-executor).

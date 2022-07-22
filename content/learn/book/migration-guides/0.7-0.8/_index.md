+++
title = "0.7 to 0.8"
weight = 4
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.7 to 0.8"
+++

<!-- Github filter used to find the relevant PRs "is:pr label:C-Breaking-Change closed:>2022-04-15 [Merged by Bors]" -->

### [Camera Driven Rendering](https://github.com/bevyengine/bevy/pull/4745)

<!-- TODO pls cart add a migration guide section :( -->

This is a very complicated change and it is recommended to read the linked PRs for more details

```rust
// old 3d perspective camera
commands.spawn_bundle(PerspectiveCameraBundle::default())

// new 3d perspective camera
commands.spawn_bundle(Camera3dBundle::default())
```

```rust
// old 2d orthographic camera
commands.spawn_bundle(OrthographicCameraBundle::new_2d())

// new 2d orthographic camera
commands.spawn_bundle(Camera2dBundle::default())
```

```rust
// old 3d orthographic camera
commands.spawn_bundle(OrthographicCameraBundle::new_3d())

// new 3d orthographic camera
commands.spawn_bundle(Camera3dBundle {
    projection: OrthographicProjection {
        scale: 3.0,
        scaling_mode: ScalingMode::FixedVertical,
        ..default()
    }.into(),
    ..default()
})
```

UI no longer requires a dedicated camera. `UiCameraBundle` has been removed. `Camera2dBundle` and `Camera3dBundle` now both default to rendering UI as part of their own render graphs. To disable UI rendering for a camera, disable it using the CameraUi component:

```rust
commands
    .spawn_bundle(Camera3dBundle::default())
    .insert(UiCameraConfig {
        show_ui: false,
        ..default()
    })
```

```rust
// 0.7
camera.world_to_screen(transform, world_position);
// 0.8
camera.world_to_viewport(transform, world_position);
```

### [SpatialBundle](https://github.com/bevyengine/bevy/pull/5344)

If you previously needed a `TransformBundle` and a `VisibilityBundle` you should now use a `SpatialBundle`

### [Make ScalingMode more flexible](https://github.com/bevyengine/bevy/pull/3253)

Adds ability to specify scaling factor for `WindowSize`, size of the fixed axis for `FixedVertical` and `FixedHorizontal` and a new `ScalingMode` that is a mix of `FixedVertical` and `FixedHorizontal`

### [Allow closing windows at runtime](https://github.com/bevyengine/bevy/pull/3575)

`bevy::input::system::exit_on_esc_system` has been removed. Use `bevy::window::close_on_esc` instead.

`CloseWindow` has been removed. Use `Window::close` instead.
The Close variant has been added to `WindowCommand`. Handle this by closing the relevant window.

### [Make RunOnce a non-manual System impl](https://github.com/bevyengine/bevy/pull/3922)

The run criterion `RunOnce`, which would make the controlled systems run only once, has been replaced with a new run criterion function ShouldRun::once. Replace all instances of RunOnce with `ShouldRun::once`.

### [Move system_param fetch struct into anonymous scope to avoid name collisions](https://github.com/bevyengine/bevy/pull/4100)

For code that was using a system param's fetch struct, such as EventReader's EventReaderState, the fetch struct can now be identified via the SystemParam trait associated type Fetch, e.g. for `EventReader<T>` it can be identified as `<EventReader<'static, 'static, T>` as SystemParam>::Fetch

### [Task doesn't impl Component](https://github.com/bevyengine/bevy/pull/4113)

If you need a `Task` to be a `Component` you should use a wrapper type.

```rs
// 0.7
fn system(mut commands: Commands) {
    let task = thread_pool.spawn(async move {
        let start_time = Instant::now();
        while Instant::now() - start_time < Duration::from_secs_f32(5.0) {
            // Spinning for 'duration', simulating doing hard
        }
        Vec2::ZERO
    });
    commands.spawn().insert(task);
}

// 0.8
#[derive(Component)]
struct ComputeVec2(Task<Vec2>);

fn system(mut commands: Commands) {
    let task = thread_pool.spawn(async move {
        let start_time = Instant::now();
        while Instant::now() - start_time < Duration::from_secs_f32(5.0) {
            // Spinning for 'duration', simulating doing hard
        }
        Vec2::ZERO
    });
    commands.spawn().insert(ComputeVec2(task))
}
```

### [Split time functionality into bevy_time](https://github.com/bevyengine/bevy/pull/4187)

* Time related types (e.g. `Time`, `Timer`, `Stopwatch`, `FixedTimestep`, etc.) should be imported from `bevy::time::*` rather than `bevy::core::*`.
* If you were adding `CorePlugin` manually, you'll also want to add `TimePlugin` from `bevy::time`.
* The `bevy::core::CorePlugin::Time` system label is replaced with `bevy::time::TimeSystem`.

### [Move float_ord from bevy_core to bevy_utils](https://github.com/bevyengine/bevy/pull/4189)

Replace imports of `bevy::core::FloatOrd` with `bevy::utils::FloatOrd`.

### [Move Rect to bevy_ui and rename it to UiRect](https://github.com/bevyengine/bevy/pull/4276)

The `Rect` type has been renamed to `UiRect`.

### [Rename ElementState to ButtonState](https://github.com/bevyengine/bevy/pull/4314)

The `ElementState` type has been renamed to `ButtonState`.

### [Improve docs and naming for RawWindowHandle functionality](https://github.com/bevyengine/bevy/pull/4335)

Renamed `HasRawWindowHandleWrapper` to `ThreadLockedRawWindowHandleWrapper`.

### [Migrate to encase from crevice](https://github.com/bevyengine/bevy/pull/4339)

#### StorageBuffer

* removed `set_body()`, `values()`, `values_mut()`, `clear()`, `push()`, `append()`
* added `set()`, `get()`, `get_mut()`

#### UniformVec -> UniformBuffer

* renamed `uniform_buffer()` to `buffer()`
* removed `len()`, `is_empty()`, `capacity()`, `push()`, `reserve()`, `clear()`, `values()`
* added `set()`, `get()`

#### DynamicUniformVec -> DynamicUniformBuffer

* renamed `uniform_buffer()` to `buffer()`
* removed `capacity()`, `reserve()`

### [Make paused timers update just_finished on tick](https://github.com/bevyengine/bevy/pull/4445)

`Timer::times_finished` has been renamed to `Timer::times_finished_this_tick` for clarity.

### [Change default Image FilterMode to Linear](https://github.com/bevyengine/bevy/pull/4465)

```rs
// 0.7

//TODO

// 0.8

// TODO
```

### [Remove .system()](https://github.com/bevyengine/bevy/pull/4499)

You can no longer use `.system()`. It was deprecated in 0.7.0. You can just remove the method call.

If you needed this for tests purposes, you can use `bevy_ecs::system::assert_is_system` instead.

### [Change gamepad.rs tuples to normal structs](https://github.com/bevyengine/bevy/pull/4519)

The `Gamepad`, `GamepadButton`, `GamepadAxis`, `GamepadEvent` and `GamepadEventRaw` types are now normal structs instead of tuple structs and have a new() function.
To migrate change every instantiation to use the `new()` function instead and use the appropriate field names instead of .0 and .1.

### [Remove EntityMut::get_unchecked](https://github.com/bevyengine/bevy/pull/4547)

Replace calls to `EntityMut::get_unchecked` with calls to `EntityMut::get`.

### [Replace ReadOnlyFetch with ReadOnlyWorldQuery](https://github.com/bevyengine/bevy/pull/4626)

The trait `ReadOnlyFetch` has been replaced with `ReadOnlyWorldQuery` along with the `WorldQueryGats::ReadOnlyFetch` assoc type which has been replaced with `<WorldQuery::ReadOnly as WorldQueryGats>::Fetch`

The trait `ReadOnlyFetch` has been replaced with `ReadOnlyWorldQuery` along with the `WorldQueryGats::ReadOnlyFetch` assoc type which has been replaced with `<WorldQuery::ReadOnly as WorldQueryGats>::Fetch`

* Any where clauses such as `QueryFetch<Q>: ReadOnlyFetch` should be replaced with `Q: ReadOnlyWorldQuery`.
* Any custom world query impls should implement `ReadOnlyWorldQuery` insead of `ReadOnlyFetch`

Functions `update_component_access` and `update_archetype_component_access` have been moved from the `FetchState` trait to `WorldQuery`

* Any callers should now call `Q::update_component_access(state` instead of `state.update_component_access` (and `update_archetype_component_access` respectively)
* Any custom world query impls should move the functions from the `FetchState` impl to `WorldQuery` impl

`WorldQuery` has been made an `unsafe trait`, `FetchState` has been made a safe `trait`. (I think this is how it should have always been, but regardless this is _definitely_ necessary now that the two functions have been moved to `WorldQuery`)

* If you have a custom `FetchState` impl make it a normal `impl` instead of `unsafe impl`
* If you have a custom `WorldQuery` impl make it an `unsafe impl`, if your code was sound before it is going to still be sound

### [Fix unsoundness with Or/AnyOf/Option component access'](https://github.com/bevyengine/bevy/pull/4659)

Query conflicts from `Or`/`AnyOf`/`Option` have been fixed, and made stricter to avoid undefined behaviour.
If you have new query conflicts due to this you must refactor your systems; consider using `ParamSet`.

### [Remove task_pool parameter from par_for_each(_mut)](https://github.com/bevyengine/bevy/pull/4705)

The `task_pool` parameter for `Query(State)::par_for_each(_mut)` has been removed. Remove these parameters from all calls to these functions.

Before:

```rust
fn parallel_system(
   task_pool: Res<ComputeTaskPool>,
   query: Query<&MyComponent>,
) {
   query.par_for_each(&task_pool, 32, |comp| {
        ...
   });
}
```

After:

```rust
fn parallel_system(query: Query<&MyComponent>) {
   query.par_for_each(32, |comp| {
        ...
   });
}
```

If using `Query` or `QueryState` outside of a system run by the scheduler, you may need to manually configure and initialize a `ComputeTaskPool` as a resource in the `World`.

### [Fail to compile on 16-bit platforms](https://github.com/bevyengine/bevy/pull/4736)

`bevy_ecs` will now explicitly fail to compile on 16-bit platforms, , due to an inability to write sound unsafe code for these platforms.

There is currently no alternative, but we're open to adding support.
Please file an issue (<https://github.com/bevyengine/bevy/issues>) to help detail your use case.

### [Enforce type safe usage of Handle::get](https://github.com/bevyengine/bevy/pull/4794)

`Assets::<T>::get` and `Assets::<T>::get_mut` now require that the passed handles are `Handle<T>`, improving the type safety of handles. If you were previously passing in:

* a `HandleId`, use `&Handle::weak(id)` instead, to create a weak handle. You may have been able to store a type safe `Handle` instead.
* a `HandleUntyped`, use `&handle_untyped.typed_weak()` to create a weak handle of the specified type. This is most likely to be the useful when using [load_folder](https://docs.rs/bevy_asset/latest/bevy_asset/struct.AssetServer.html#method.load_folder)
* a `Handle<U>` of  of a different type, consider whether this is the correct handle type to store. If it is (i.e. the same handle id is used for multiple different Asset types) use `Handle::weak(handle.id)` to cast to a different type.

### [Allow higher order systems](https://github.com/bevyengine/bevy/pull/4833)

`SystemParamFunction` has changed. It was not previously part of the public API, so no migration instructions are provided. (It is now included in the public API, although you still should not implement this trait for your own types).

If possible, any custom `System` implementations should be migrated to use higher order systems, which are significantly less error-prone.

Research is needed into allowing this to work for more cases.

### [Added offset parameter to TextureAtlas::from_grid_with_padding](https://github.com/bevyengine/bevy/pull/4836)

Calls to `TextureAtlas::from_grid_with_padding` should be modified to include a new parameter, which can be set to `Vec2::ZERO` to retain old behaviour.

```rust
from_grid_with_padding(texture, tile_size, columns, rows, padding)
                                  |
                                  V
from_grid_with_padding(texture, tile_size, columns, rows, padding, Vec2::ZERO)
```

### [Split mesh shader files](https://github.com/bevyengine/bevy/pull/4867)

* In shaders for 3D meshes:
  * `#import bevy_pbr::mesh_view_bind_group` -> `#import bevy_pbr::mesh_view_bindings`
  * `#import bevy_pbr::mesh_struct` -> `#import bevy_pbr::mesh_types`
    * NOTE: If you are using the mesh bind group at bind group index 2, you can remove those binding statements in your shader and just use `#import bevy_pbr::mesh_bindings` which itself imports the mesh types needed for the bindings.
* In shaders for 2D meshes:
  * `#import bevy_sprite::mesh2d_view_bind_group` -> `#import bevy_sprite::mesh2d_view_bindings`
  * `#import bevy_sprite::mesh2d_struct` -> `#import bevy_sprite::mesh2d_types`
    * NOTE: If you are using the mesh2d bind group at bind group index 2, you can remove those binding statements in your shader and just use `#import bevy_sprite::mesh2d_bindings` which itself imports the mesh2d types needed for the bindings.

### [Camera Driven Viewports](https://github.com/bevyengine/bevy/pull/4898)

`Camera::projection_matrix` is no longer a public field. Use the new `Camera::projection_matrix()` method instead:

```rust

// 0.7
let projection = camera.projection_matrix;

// 0.8
let projection = camera.projection_matrix();
```

### [diagnostics: meaningful error when graph node has wrong number of inputs](https://github.com/bevyengine/bevy/pull/4924)

Exhaustive matches on `RenderGraphRunnerError` will need to add a branch to handle the new `MismatchedInputCount` variant.

### [Make Reflect safe to implement](https://github.com/bevyengine/bevy/pull/5010)

* Reflect derives should not have to change anything
* Manual reflect impls will need to remove the `unsafe` keyword, add `any()` implementations, and rename the old `any` and `any_mut` to `as_any` and `as_mut_any`.
* Calls to `any`/`any_mut` must be changed to `as_any`/`as_mut_any`

### [Mark mutable APIs under ECS storage as pub(crate)](https://github.com/bevyengine/bevy/pull/5065)

<!-- Dear God, I hope not. -->

If you experienced any problems caused by this change, please [create an issue](https://github.com/bevyengine/bevy/issues) explaining _in detail_ what you were doing with those apis.

### [Add global init and get accessors for all newtyped TaskPools](https://github.com/bevyengine/bevy/pull/2250)

Thread pools don't need to be stored in a resource anymore since they are now stored globally. You can now use `get()` to access it.

```rust
// 0.7
fn spawn_tasks(thread_pool: Res<AsyncComputeTaskPool>) {
    // Do something with thread_pool
}

// 0.8
fn spawn_tasks() {
    let thread_pool = AsyncComputeTaskPool::get();
    // Do something with thread_pool
}
```

### [Simplify design for *Labels](https://github.com/bevyengine/bevy/pull/4957)

* Any previous use of `Box<dyn SystemLabel>` should be replaced with `SystemLabelId`.
* `AsSystemLabel` trait has been modified.
  * No more output generics.
  * Method `as_system_label` now returns `SystemLabelId`, removing an unnecessary level of indirection.
* If you _need_ a label that is determined at runtime, you can use `Box::leak`. Not recommended.

### [Move get_short_name utility method from bevy_reflect into bevy_utils](https://github.com/bevyengine/bevy/pull/5174)

* added bevy_utils::get_short_name, which strips the path from a type name for convenient display.
* removed the TypeRegistry::get_short_name method. Use the function in bevy_utils instead.

### [Remove dead SystemLabelMarker struct](https://github.com/bevyengine/bevy/pull/5190)

This struct had no internal use, docs, or intuitable external use.

It has been removed.

### [Add reflection for resources](https://github.com/bevyengine/bevy/pull/5175)

Rename `ReflectComponent::add_component` into `ReflectComponent::insert_component`.

### [Make reflect_partial_eq return more accurate results](https://github.com/bevyengine/bevy/pull/5210)

Updated [struct_trait](https://github.com/bevyengine/bevy/blob/dfe969005264fff54060f9fb148639f80f9cfb29/crates/bevy_reflect/src/struct_trait.rs#L455-L457), [tuple_struct](https://github.com/bevyengine/bevy/blob/dfe969005264fff54060f9fb148639f80f9cfb29/crates/bevy_reflect/src/tuple_struct.rs#L366-L368), [tuple](https://github.com/bevyengine/bevy/blob/dfe969005264fff54060f9fb148639f80f9cfb29/crates/bevy_reflect/src/tuple.rs#L386), [array](https://github.com/bevyengine/bevy/blob/dfe969005264fff54060f9fb148639f80f9cfb29/crates/bevy_reflect/src/array.rs#L335-L337), [list](https://github.com/bevyengine/bevy/blob/dfe969005264fff54060f9fb148639f80f9cfb29/crates/bevy_reflect/src/list.rs#L309-L311) and [map](https://github.com/bevyengine/bevy/blob/dfe969005264fff54060f9fb148639f80f9cfb29/crates/bevy_reflect/src/map.rs#L361-L363) to return `None` when comparison couldn't be performed.

### [Make RenderStage::Extract run on the render world](https://github.com/bevyengine/bevy/pull/4402)

The `Extract` `RenderStage` now runs on the render world (instead of the main world as before).
You must use the `Extract` `SystemParam` to access the main world during the extract phase. `Extract` takes a single type parameter, which is any system parameter (such as `Res`, `Query` etc.).
It will extract this from the main world.
Note that `Commands` will not work correctly in `Extract` - it will currently silently do nothing.

```rust
// 0.7
fn extract_clouds(mut commands: Commands, clouds: Query<Entity, With<Cloud>>) {
    for cloud in clouds.iter() {
        commands.get_or_spawn(cloud).insert(Cloud);
    }
}

// 0.8
fn extract_clouds(mut commands: Commands, mut clouds: Extract<Query<Entity, With<Cloud>>>) {
    for cloud in clouds.iter() {
        commands.get_or_spawn(cloud).insert(Cloud);
    }
}
```

You can now also access resources from the render world using the normal system parameters during `Extract`:

```rust
fn extract_assets(mut render_assets: ResMut<MyAssets>, source_assets: Extract<Res<MyAssets>>) {
     *render_assets = source_assets.clone();
}
```

Please note that all existing extract systems need to be updated to match this new style; even if they currently compile they will not run as expected. A warning will be emitted on a best-effort basis if this is not met.

### [Improve Gamepad DPad Button Detection](https://github.com/bevyengine/bevy/pull/5220)

D-pad inputs can no longer be accessed as axes.
Acess them as gamepad buttons instead.

### [Change window position types from tuple to vec](https://github.com/bevyengine/bevy/pull/5276)

Changed the following fields

* `WindowCommand::SetWindowMode.resolution` from `(u32, u32)` to `UVec2`
* `WindowCommand::SetResolution.logical_resolution` from `(f32, f32)` to `Vec2`

### [Full documentation for bevy_asset](https://github.com/bevyengine/bevy/pull/3536)

* Rename `FileAssetIo::get_root_path` uses to `FileAssetIo::get_base_path`

    `FileAssetIo::root_path()` is a getter for the `root_path` field, while `FileAssetIo::get_root_path` returned the parent directory of the asset root path, which was the executable's directory unless `CARGO_MANIFEST_DIR` was set. This change solves the ambiguity between the two methods.

### [Use Affine3A for GlobalTransform to allow any affine transformation](https://github.com/bevyengine/bevy/pull/4379)

`GlobalTransform` fields have changed

* Replace `global_transform.translation` by `global_transform.translation()` (For other fields, use the `compute_transform` method)
* `GlobalTransform` do not support non-linear scales anymore, we'd like to hear from you if it is an inconvenience for you

<!-- TODO mention .to_scale_rotation_translation() -->

### [Hierarchy commandization](https://github.com/bevyengine/bevy/pull/4197)

The `Parent` and `Children` component fields are now private.

* Replace `parent.0` by `parent.get()`
* Replace `children.0` with `*children`
* You can't construct `Children` or `Parent` component anymore, you can use this as a stopgap measure, which may introduce a single frame delay

```rust
#[derive(Component)]
pub struct MakeChildOf(pub Entity);

fn add_parent(
    mut commands: Commands,
    orphans: Query<(Entity, &MakeChildOf)>,
) {
    for (child, MakeChildOf(parent)) in &orphans {
        commands.entity(parent).add_child(child);
        commands.entity(child).remove::<MakeChildOf>();
    }
}
```

### [Visibilty Inheritance, universal ComputedVisibility and RenderLayers support](https://github.com/bevyengine/bevy/pull/5310)

If you were previously reading `Visibility::is_visible` as the "actual visibility" for sprites or lights, use `ComputedVisibilty::is_visible()` instead:

```rust
// before (0.7)
fn system(query: Query<&Visibility>) {
  for visibility in query.iter() {
    if visibility.is_visible {
       log!("found visible entity");
    }
  }
}

// after (0.8)
fn system(query: Query<&ComputedVisibility>) {
  for visibility in query.iter() {
    if visibility.is_visible() {
       log!("found visible entity");
    }
  }
}
```

### [remove blanket Serialize + Deserialize requirement for Reflect on generic types](https://github.com/bevyengine/bevy/pull/5197)

* `.register_type` for generic types like `Option<T>`, `Vec<T>`, `HashMap<K, V>` will no longer insert `ReflectSerialize` and `ReflectDeserialize` type data. Instead you need to register it separately for concrete generic types like so:

```rust
    .register_type::<Option<String>>()
    .register_type_data::<Option<String>, ReflectSerialize>()
    .register_type_data::<Option<String>, ReflectDeserialize>()
```

### [add a SceneBundle to spawn a scene](https://github.com/bevyengine/bevy/pull/2424)

```rust
// 0.7
commands.spawn_scene(asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"));

//0.8
commands.spawn_bundle(SceneBundle {
    scene: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"),
    ..Default::default()
});
```

The scene will be spawned as a child of the entity with the `SceneBundle`

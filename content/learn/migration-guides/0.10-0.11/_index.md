+++
title = "0.10 to 0.11"
weight = 2
sort_by = "weight"
template = "docs-section.html"
page_template = "docs-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.10 to 0.11"
+++

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.
<div class="migration-guide">

### [Merge ScheduleRunnerSettings into ScheduleRunnerPlugin](https://github.com/bevyengine/bevy/pull/8585)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">App</div>
</div>

- instead of inserting the `ScheduleRunnerSettings` resource, configure the `ScheduleRunnerPlugin`

### [Allow tuples and single plugins in `add_plugins`, deprecate `add_plugin`](https://github.com/bevyengine/bevy/pull/8097)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">App</div>
</div>

- Replace `app.add_plugin(plugin)` calls with `app.add_plugins(plugin)`.

### [Add support for custom glTF vertex attributes.](https://github.com/bevyengine/bevy/pull/5370)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

- If you were instantiating `GltfPlugin` using the unit-like struct syntax, you must instead use `GltfPlugin::default()` as the type is no longer unit-like.

### [Delay asset hot reloading](https://github.com/bevyengine/bevy/pull/8503)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

- Replace `AssetPlugin::watch_for_changes: true` with e.g. `ChangeWatcher::with_delay(Duration::from_millis(200))`

### [Allow systems using Diagnostics to run in parallel](https://github.com/bevyengine/bevy/pull/8677)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Diagnostics</div>
</div>

- Register `Diagnostic`’s using the new `app.register_diagnostic(Diagnostic::new(DIAGNOSTIC_ID, "diagnostic_name", 10));`
- In systems for writing new measurements, change `mut diagnostics: ResMut<Diagnostics>` to `mut diagnostics: Diagnostics` to allow the systems to run in parallel.
- In systems for reading measurements, change `diagnostics: Res<Diagnostics>` to `diagnostics: Res<DiagnosticsStore>`.

### [log to stderr instead of stdout](https://github.com/bevyengine/bevy/pull/8886)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Diagnostics</div>
</div>

- Capture logs from `stderr` instead of from `stdout`
  - Use `2> output.log` on the command line to save `stderr` to a file

### [Make `WorldQuery` meta types unnameable](https://github.com/bevyengine/bevy/pull/7964)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The `State` and `Fetch` types for types created using `#[derive(WorldQuery)]` are now unnameable. If you need to refer to them, use the syntax `<T as WorldQuery>::State`, `<T as WorldQuery>::Fetch`.

### [Increase type safety and clarity for change detection](https://github.com/bevyengine/bevy/pull/7905)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The engine now uses the type `Tick` for dealing with change ticks, instead of `u32`. Any code that interfaced with engine internals will need to be updated, including:

- Manual implementers of the traits `SystemParam`, `WorldQuery`, `DetectChanges`, and `DetectChangesMut`.
- The methods `World::change_tick` and `read_change_tick`.
- `System::set_last_change_tick` and `get_last_change_tick`. Also, these methods have been renamed to `set_last_run` and `get_last_run`, respectively.
- The methods `SystemChangeTick::change_tick` and `last_change_tick`. These methods have been renamed to `this_run` and `last_run`, respectively.
- The method `Tick::set_changed`, which has been renamed to just `set`.

### [Remove ChangeTrackers](https://github.com/bevyengine/bevy/pull/7902)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`ChangeTrackers` has been removed .Use `Ref<T>` queries instead.

### [Check for conflicting accesses in `assert_is_system`](https://github.com/bevyengine/bevy/pull/8154)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The functions `assert_is_system` and `assert_is_read_only_system` (in `bevy_ecs::system`) now panic if the passed system has invalid world accesses. Any tests that called this function on a system with invalid accesses will now fail. Either fix the system’s conflicting accesses, or specify that the test is meant to fail:

- For regular tests (that is, functions annotated with `#[test]`), add the `#[should_panic]` attribute to the function.
- For documentation tests, add `should_panic` to the start of the code block: ` ```should_panic`

### [`Or<T>` should be a new type of `PhantomData<T>`](https://github.com/bevyengine/bevy/pull/8212)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`Or<T>` is just used as a type annotation and shouldn’t be constructed.

### [Remove base set error variants of `ScheduleBuildError`](https://github.com/bevyengine/bevy/pull/8269)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Remove the now unused variants of `ScheduleBuildError`

### [Remove `#[system_param(ignore)]` and `#[world_query(ignore)]`](https://github.com/bevyengine/bevy/pull/8265)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The attributes `#[system_param(ignore)]` and `#[world_query]` ignore have been removed. If you were using either of these with `PhantomData` fields, you can simply remove the attribute:

```rust
#[derive(SystemParam)]
struct MyParam<'w, 's, Marker> {
    ...
    // 0.10
    #[system_param(ignore)
    _marker: PhantomData<Marker>,

    // 0.11
    _marker: PhantomData<Marker>,
}
#[derive(WorldQuery)]
struct MyQuery<Marker> {
    ...
    // 0.10
    #[world_query(ignore)
    _marker: PhantomData<Marker>,

    // 0.11
    _marker: PhantomData<Marker>,
}
```

If you were using this for another type that implements `Default`, consider wrapping that type in `Local<>` (this only works for `SystemParam`):

```rust
#[derive(SystemParam)]
struct MyParam<'w, 's> {
    // 0.10
    #[system_param(ignore)]
    value: MyDefaultType, // This will be initialized using `Default` each time `MyParam` is created.

    // 0.11
    value: Local<MyDefaultType>, // This will be initialized using `Default` the first time `MyParam` is created.
}
```

If you are implementing either trait and need to preserve the exact behavior of the old `ignore` attributes, consider manually implementing `SystemParam` or `WorldQuery` for a wrapper struct that uses the `Default` trait:

```rust
// 0.10

#[derive(WorldQuery)
struct MyQuery {
   #[world_query(ignore)]
    str: String,
}

// 0.11

#[derive(WorldQuery)
struct MyQuery {
    str: DefaultQuery<String>,
}

pub struct DefaultQuery<T: Default>(pub T);

unsafe impl<T: Default> WorldQuery for DefaultQuery<T> {
    type Item<'w> = Self;
    ...
    unsafe fn fetch<'w>(...) -> Self::Item<'w> {
        Self(T::default())
    }
}
```

### [Replace some unsafe system executor code with safe code](https://github.com/bevyengine/bevy/pull/8274)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The function `bevy_utils::SyncUnsafeCell::get_mut` now returns a value of type `&mut SyncUnsafeCell<T>`. Previously, this returned an immutable reference.

### [Use `UnsafeWorldCell` to increase code quality for `SystemParam`](https://github.com/bevyengine/bevy/pull/8174)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

For manual implementers of `SystemParam`: the function `get_item` now takes `UnsafeWorldCell` instead of `&World`. To access world data, use:

- `.get_entity()`, which returns an `UnsafeEntityCell` which can be used to access component data.
- `get_resource()` and its variants, to access resource data.

### [Update `increment_change_tick` to return a strongly-typed `Tick`](https://github.com/bevyengine/bevy/pull/8295)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The function `UnsafeWorldCell::increment_change_tick` is now strongly-typed, returning a value of type `Tick` instead of a raw `u32`.

### [Make state private and only accessible through getter for State resource](https://github.com/bevyengine/bevy/pull/8009)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Use `State::get` instead of accessing the tuple field directly.

### [Only trigger state transitons if `next_state != old_state`](https://github.com/bevyengine/bevy/pull/8359)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

State transitions are now only triggered when the exited and entered state differ. This means that if the world is currently in state `A`, the `OnEnter(A)` schedule (or `OnExit`) will no longer be run if you queue up a state transition to the same state `A`.

### [Simplify system piping and make it more flexible](https://github.com/bevyengine/bevy/pull/8377)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The `IntoPipeSystem` trait has been removed, and the `pipe` method has been moved to the `IntoSystem` trait.

```rust

// 0.10
use bevy_ecs::system::IntoPipeSystem;
schedule.add_systems(first.pipe(second));

// 0.11
use bevy_ecs::system::IntoSystem;
schedule.add_systems(first.pipe(second));
```

### [Simplify world schedule methods](https://github.com/bevyengine/bevy/pull/8403)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The method `World::run_schedule_ref` has been deprecated, and will be removed in the next version of Bevy. Use `run_schedule` instead.

### [Rename `UnsafeWorldCell::read_change_tick`](https://github.com/bevyengine/bevy/pull/8588)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The `UnsafeWorldCell` method `read_change_tick` has been renamed to `change_tick`.

### [Improve safety for the multi-threaded executor using `UnsafeWorldCell`](https://github.com/bevyengine/bevy/pull/8292)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The `System` trait now uses `UnsafeWorldCell` instead of `&World`. This type provides a robust API for interior mutable world access.

- The method `run_unsafe` uses this type to manage world mutations across multiple threads.
- The method `update_archetype_component_access` uses this type to ensure that only world metadata can be used.

```rust
let mut system = IntoSystem::into_system(my_system);
system.initialize(&mut world);

// 0.10
system.update_archetype_component_access(&world);
unsafe { system.run_unsafe(&world) }

// 0.11
system.update_archetype_component_access(world.as_unsafe_world_cell_readonly());
unsafe { system.run_unsafe(world.as_unsafe_world_cell()) }
```

### [Improve encapsulation for commands and add docs](https://github.com/bevyengine/bevy/pull/8725)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The `Command` types `Remove` and `RemoveResource` may no longer be constructed manually.

```rust
// 0.10
commands.add(Remove::<T> {
    entity: id,
    phantom: PhantomData,
});

// 0.11
commands.add(Remove::<T>::new(id));

// 0.10
commands.add(RemoveResource::<T> { phantom: PhantomData });

// 0.11
commands.add(RemoveResource::<T>::new());
```

The command type `GetOrSpawn` has been removed. It was not possible to use this type outside of `bevy_ecs`.

### [Rename apply_system_buffers to apply_deferred](https://github.com/bevyengine/bevy/pull/8726)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- `apply_system_buffers` has been renamed to `apply_deferred`
- the `apply_system_buffers` method on the `System` trait has been renamed to `apply_deferred`
- the `is_apply_system_buffers` function has been replaced by `is_apply_deferred`
- `Executor::set_apply_final_buffers` is now `Executor::set_apply_final_deferred`
- `Schedule::apply_system_buffers` is now `Schedule::apply_deferred`

### [Rename Command's "write" method to "apply"](https://github.com/bevyengine/bevy/pull/8814)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- `Command::write` implementations need to be changed to implement `Command::apply` instead. This is a mere name change, with no further actions needed.
- `EntityCommand::write` implementations need to be changed to implement `EntityCommand::apply` instead. This is a mere name change, with no further actions needed.

### [Require read-only queries in `QueryState::par_iter`](https://github.com/bevyengine/bevy/pull/8832)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The function `QueryState::par_iter` now forces any world accesses to be read-only, similar to how `QueryState::iter` works. Any code that previously mutated the world using this method was _unsound_. If you need to mutate the world, use `par_iter_mut` instead.

### [Migrate the rest of the engine to `UnsafeWorldCell`](https://github.com/bevyengine/bevy/pull/8833)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Mutating any world data using `&World` is now considered unsound – the type `UnsafeWorldCell` must be used to achieve interior mutability. The following methods now accept `UnsafeWorldCell` instead of `&World`:

- `QueryState`: `get_unchecked`, `iter_unchecked`, `iter_combinations_unchecked`, `for_each_unchecked`, `get_single_unchecked`, `get_single_unchecked_manual`.
- `SystemState`: `get_unchecked_manual`

```rust
let mut world = World::new();
let mut query = world.query::<&mut T>();

// 0.10
let t1 = query.get_unchecked(&world, entity_1);
let t2 = query.get_unchecked(&world, entity_2);

// 0.11
let world_cell = world.as_unsafe_world_cell();
let t1 = query.get_unchecked(world_cell, entity_1);
let t2 = query.get_unchecked(world_cell, entity_2);
```

The methods `QueryState::validate_world` and `SystemState::matches_world` now take a `WorldId` instead of `&World`:

```rust
// 0.10
query_state.validate_world(&world);

// 0.11
query_state.validate_world(world.id());
```

The methods `QueryState::update_archetypes` and `SystemState::update_archetypes` now take `UnsafeWorldCell` instead of `&World`:

```rust
// 0.10
query_state.update_archetypes(&world);

// 0.11
query_state.update_archetypes(world.as_unsafe_world_cell_readonly());
```

### [Simplify the `ComponentIdFor` type](https://github.com/bevyengine/bevy/pull/8845)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The type `ComponentIdFor<T>` now implements `SystemParam` instead of `FromWorld` – this means it should be used as the parameter for a system directly instead of being used in a `Local`.

```rust
// 0.10
fn my_system(
    component_id: Local<ComponentIdFor<MyComponent>>,
) {
    let component_id = **component_id;
}

// 0.11
fn my_system(
    component_id: ComponentIdFor<MyComponent>,
) {
    let component_id = component_id.get();
}
```

### [Make `QueryParIter::for_each_unchecked` private](https://github.com/bevyengine/bevy/pull/8848)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The method `QueryParIter::for_each_unchecked` has been removed – use `for_each` or `for_each_mut` instead. If your use case can not be achieved using either of these, then your code was likely unsound.

If you have a use-case for `for_each_unchecked` that you believe is sound, please [open an issue](https://github.com/bevyengine/bevy/issues/new/choose).

### [Deprecate type aliases for `WorldQuery::Fetch`](https://github.com/bevyengine/bevy/pull/8843)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The type aliases `bevy_ecs::query::QueryFetch` and `ROQueryFetch` have been deprecated. If you need to refer to a `WorldQuery` struct’s fetch type, refer to the associated type defined on `WorldQuery` directly:

```rust
// 0.10
type MyFetch<'w> = QueryFetch<'w, MyQuery>;
type MyFetchReadOnly<'w> = ROQueryFetch<'w, MyQuery>;

// 0.11
type MyFetch<'w> = <MyQuery as WorldQuery>::Fetch;
type MyFetchReadOnly<'w> = <<MyQuery as WorldQuery>::ReadOnly as WorldQuery>::Fetch;
```

### [Implement WorldQuery for EntityRef](https://github.com/bevyengine/bevy/pull/6960)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

TODO

### [Move AppTypeRegistry to bevy_ecs](https://github.com/bevyengine/bevy/pull/8901)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Reflection</div>
    <div class="migration-guide-area-tag">App</div>
</div>

- If you were not using a `prelude::*` to import `AppTypeRegistry`, you should update your imports:

```diff
- use bevy::app::AppTypeRegistry;
+ use bevy::ecs::reflect::AppTypeRegistry
```

### [Make scene handling of entity references robust](https://github.com/bevyengine/bevy/pull/7335)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Scenes</div>
</div>

- `MapEntities` implementations must change from a `&EntityMap` parameter to a `&mut EntityMapper` parameter and can no longer return a `Result`. Finally, they should switch from calling `EntityMap::get` to calling `EntityMapper::get_or_reserve`.

### [Rename map_entities and map_specific_entities](https://github.com/bevyengine/bevy/pull/7570)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Scenes</div>
</div>

- In `bevy_ecs`, `ReflectMapEntities::map_entites` now requires an additional `entities` parameter to specify which entities it applies to. To keep the old behavior, use the new `ReflectMapEntities::map_all_entities`, but consider if passing the entities in specifically might be better for your use case to avoid bugs.

### [Rename keys like `LAlt` to `AltLeft`](https://github.com/bevyengine/bevy/pull/8792)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Input</div>
</div>

Migrate by replacing:

- `LAlt` → `AltLeft`
- `RAlt` → `AltRight`
- `LBracket` → `BracketLeft`
- `RBracket` → `BracketRight`
- `LControl` → `ControlLeft`
- `RControl` → `ControlRight`
- `LShift` → `ShiftLeft`
- `RShift` → `ShiftRight`
- `LWin` → `SuperLeft`
- `RWin` → `SuperRight`

### [Don't ignore additional entries in `UntypedReflectDeserializerVisitor`](https://github.com/bevyengine/bevy/pull/7112)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

If you were deserializing `Box<dyn Reflect>` values with multiple entries (i.e. entries other than `"type": { /* fields */ }`) you should remove them or deserialization will fail.

### [Construct Box<dyn Reflect> from world for ReflectComponent](https://github.com/bevyengine/bevy/pull/7407)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
    <div class="migration-guide-area-tag">Scenes</div>
</div>

<!-- TODO -->

### [Added Globals struct to prepass shader](https://github.com/bevyengine/bevy/pull/8070)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

<!-- TODO -->

### [Make render graph slots optional for most cases](https://github.com/bevyengine/bevy/pull/8109)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

You can now get the view_entity directly from the `RenderGraphContext`.

When implementing the Node:

```rust
// 0.10
struct FooNode;
impl FooNode {
    const IN_VIEW: &'static str = "view";
}
impl Node for FooNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![SlotInfo::new(Self::IN_VIEW, SlotType::Entity)]
    }
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        // ...
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.get_input_entity(Self::IN_VIEW)?;
        // ...
        Ok(())
    }
}

// 0.11
struct FooNode;
impl Node for FooNode {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        // ...
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.view_entity();
        // ...
        Ok(())
    }
}
```

When adding the node to the graph, you don’t need to specify a slot_edge for the view_entity.

```rust
// 0.10
let mut graph = RenderGraph::default();
graph.add_node(FooNode::NAME, node);
let input_node_id = draw_2d_graph.set_input(vec![SlotInfo::new(
    graph::input::VIEW_ENTITY,
    SlotType::Entity,
)]);
graph.add_slot_edge(
    input_node_id,
    graph::input::VIEW_ENTITY,
    FooNode::NAME,
    FooNode::IN_VIEW,
);
// add_node_edge ...

// 0.11
let mut graph = RenderGraph::default();
graph.add_node(FooNode::NAME, node);
// add_node_edge ...
```

### [Remove unnecesssary values Vec from DynamicUniformBuffer and DynamicStorageBuffer](https://github.com/bevyengine/bevy/pull/8299)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

<!-- TODO -->

### [Changed (Vec2, Vec2) to Rect in Camera::logical_viewport_rect](https://github.com/bevyengine/bevy/pull/7867)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

```rust
// 0.10
fn view_logical_camera_rect(camera_query: Query<&Camera>) {
    let camera = camera_query.single();
    let Some((min, max)) = camera.logical_viewport_rect() else { return };
    dbg!(min, max);
}

// 0.11
fn view_logical_camera_rect(camera_query: Query<&Camera>) {
    let camera = camera_query.single();
    let Some(Rect { min, max }) = camera.logical_viewport_rect() else { return };
    dbg!(min, max);
}
```

### [make glsl and spirv support optional](https://github.com/bevyengine/bevy/pull/8491)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- If you want to use shaders in `spirv`, enable the `shader_format_spirv` feature
- If you want to use shaders in `glsl`, enable the `shader_format_glsl` feature

### [Updated to wgpu 0.16.0, wgpu-hal 0.16.0 and naga 0.12.0](https://github.com/bevyengine/bevy/pull/8446)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

<!-- TODO -->

### [Change default tonemapping method](https://github.com/bevyengine/bevy/pull/8685)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- The default tonemapper has been changed from ReinhardLuminance to TonyMcMapface. Explicitly set ReinhardLuminance on your cameras to get back the previous look.

### [Apply codebase changes in preparation for `StandardMaterial` transmission](https://github.com/bevyengine/bevy/pull/8704)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- `ViewTarget::main_texture()` and `ViewTarget::main_texture_other()` now return `&Texture` instead of `&TextureView`. If you were relying on these methods, replace your usage with `ViewTarget::main_texture_view()`and `ViewTarget::main_texture_other_view()`, respectively;
- `ViewTarget::sampled_main_texture()` now returns `Option<&Texture>` instead of a `Option<&TextureView>`. If you were relying on this method, replace your usage with `ViewTarget::sampled_main_texture_view()`;
- The `apply_fog()`, `linear_fog()`, `exponential_fog()`, `exponential_squared_fog()` and `atmospheric_fog()` functions now take a configurable `Fog` struct. If you were relying on them, update your usage by adding the global `fog` uniform as their first argument;

### [Remove `Component` derive for AlphaMode](https://github.com/bevyengine/bevy/pull/8804)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`AlphaMode` is not a component anymore.

It wasn’t used anywhere in the engine. If you were using it as a component for your own purposes, you should use a newtype instead, as follow:

```rust
#[derive(Component, Deref)]
struct MyAlphaMode(AlphaMode);
```

Then replace uses of `AlphaMode` with `MyAlphaMode`

```diff
- Query<&AlphaMode, …>,
+ Query<&MyAlphaMode, …>,
```

### [Rename `Plane` struct to `HalfSpace`](https://github.com/bevyengine/bevy/pull/8744)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Change instances of `render::primitives::Plane` to `render::primitives::HalfSpace`
- Change instances of the `planes` member in `render::primitives::Frustum` to `half_spaces`

### [Add `RenderTarget::TextureView`](https://github.com/bevyengine/bevy/pull/8042)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

References to the `RenderTarget` enum will need to handle the additional field, ie in `match` statements.

### [Consistent screen-space coordinates](https://github.com/bevyengine/bevy/pull/8306)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">Windowing</div>
    <div class="migration-guide-area-tag">UI</div>
</div>

`Window::cursor_position` now returns the position of the cursor relative to the top left instead of the bottom left.
This now matches other screen-space coordinates like `RelativeCursorPosition`, UI, and viewports.

The `world_to_viewport`, `viewport_to_world`, and `viewport_to_world_2d` methods on `Camera` now return/take the viewport position relative to the top left instead of the bottom left.

If you were using `world_to_viewport` to position a UI node the returned `y` value should now be passed into the `top` field on `Style` instead of the `bottom` field.
Note that this might shift the position of the UI node as it is now anchored at the top.

If you were passing `Window::cursor_position` to `viewport_to_world` or `viewport_to_world_2d` no change is necessary.

### [Fix look_to variable naming](https://github.com/bevyengine/bevy/pull/8627)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Transform</div>
</div>

- `Transform::look_to` method changed default value of `direction.try_normalize()` from `Vec3::Z` to `Vec3::NEG_Z`

### [Fix transform propagation of orphaned entities](https://github.com/bevyengine/bevy/pull/7264)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Transform</div>
    <div class="migration-guide-area-tag">Hierarchy</div>
</div>

- If you called `bevy_transform::systems::sync_simple_transforms` and `bevy_transform::systems::propagate_transforms` (which is not re-exported by bevy) you need to account for the additional `RemovedComponents<Parent>` parameter.

### [Remove `Val::Undefined`](https://github.com/bevyengine/bevy/pull/7485)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- `Val::Undefined` has been removed. Bevy UI’s behaviour with default values should remain the same.
- The default values of `UiRect`’s fields have been changed to `Val::Px(0.)`.
- `Style`’s `position` field has been removed. Its `left`, `right`, `top` and `bottom` fields have been added to `Style` directly.
- For the `size`, `margin`, `border`, and `padding` fields of `Style`, `Val::Undefined` should be replaced with `Val::Px(0.)`.
- For the `min_size`, `max_size`, `left`, `right`, `top` and `bottom` fields of `Style`, `Val::Undefined` should be replaced with  `Val::Auto`

### [Changed spelling linebreak_behaviour to linebreak_behavior](https://github.com/bevyengine/bevy/pull/8285)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

Update where `linebreak_behaviour` is used to `linebreak_behavior`
Updated the event `FileDragAndDrop::HoveredFileCancelled` where used to `HoveredFileCanceled`
Update `Touches.just_cancelled` where used as `Touches.just_canceled`
The event `TouchPhase::Cancelled` is now called `TouchPhase::Canceled`

### [Add CSS Grid support to `bevy_ui`](https://github.com/bevyengine/bevy/pull/8026)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

-The `UiSystem::Flex` system set has been renamed to `UiSystem::Layout`

### [`MeasureFunc` improvements](https://github.com/bevyengine/bevy/pull/8402)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- `CalculatedSize` has been renamed to `ContentSize`.
- The `upsert_leaf` function has been removed from `UiSurface` and replaced with `update_measure` which updates the `MeasureFunc` without node insertion.
- The `dyn_clone` method has been removed from the `Measure` trait.
- The new function of `CalculatedSize` has been replaced with the method `set`.

### [Flatten UI `Style` properties that use `Size` + remove `Size`](https://github.com/bevyengine/bevy/pull/8548)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- The `size`, `min_size`, `max_size`, and `gap` properties have been replaced by the `width`, `height`, `min_width`, `min_height`, `max_width`, `max_height`, `row_gap`, and `column_gap` properties. Use the new properties instead.

### [update ahash and hashbrown](https://github.com/bevyengine/bevy/pull/8623)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">No area label</div>
</div>

- If you were using hashes to an asset or using one of the fixed hasher exposed by Bevy with a previous version, you will have to update the hashes

### [Move bevy_ui accessibility systems to `PostUpdate`.](https://github.com/bevyengine/bevy/pull/8653)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">No area label</div>
</div>

<!-- TODO -->
</div>

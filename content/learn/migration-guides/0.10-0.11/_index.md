+++
title = "0.10 to 0.11"
weight = 6
sort_by = "weight"
template = "docs-section.html"
page_template = "docs-section.html"
insert_anchor_links = "right"
aliases = ["learn/migration-guides/0.10-0.11"]
[extra]
long_title = "Migration Guide: 0.10 to 0.11"
+++

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.
<div class="migration-guide">

### [Schedule-First: the new and improved add_systems](https://github.com/bevyengine/bevy/pull/8079)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

We have [unified adding systems to schedules under a single API](https://github.com/bevyengine/bevy/pull/8079)! `add_systems` now accepts a `ScheduleLabel` as the first parameter. `app.add_system`, `app.add_startup_system`, `app.add_startup_systems`, `system.on_startup()`, and `system.in_schedule()` have been deprecated in favor of the unified `app.add_systems` API.

“base sets” have been removed entirely in favor of Schedules. The built in `CoreSet` and `StartupSet` base sets have been replaced with top level schedules. (ex: `CoreSet::Update` is now the `Update` schedule).

This removes a ton of redundant APIs, removes implicit defaults entirely, and clears up a lot of the confusion introduced by base sets. We believe the consistency and ergonomics of the new `add_systems` API speaks for itself:

```rust
// 0.10
app.add_system(a)
// 0.11
app.add_systems(Update, a)

// 0.10
app.add_systems((a, b).in_schedule(CoreSchedule::Startup))
// 0.11
app.add_systems(Startup, (a, b))

// 0.10
app.add_systems((a, b).in_schedule(CoreSchedule::Startup).in_base_set(StartupSet::PreStartup))
// 0.11
app.add_systems(PreStartup, (a, b))

// 0.10
app.add_startup_systems((a, b))
// 0.11
app.add_systems(Startup, (a, b))

// 0.10
app.add_systems((a, b).on_startup())
// 0.11
app.add_systems(Startup, (a, b))

// 0.10
app.add_systems((c, d, e))
// 0.11 (Update is no longer implied by default)
app.add_systems(Update, (c, d, e))

// 0.10
app.add_systems((f, g).in_schedule(CoreSchedule::FixedUpdate))
// 0.11
app.add_systems(FixedUpdate, (f, g))

// 0.10
app.add_systems(h.in_base_set(CoreSet::PostUpdate))
// 0.11
app.add_systems(PostUpdate, h)

// 0.10
app.add_systems(enter_menu.in_schedule(OnEnter(AppState::Menu)))
// 0.11
app.add_systems(OnEnter(AppState::Menu), enter_menu)

// 0.10
app.add_systems(exit_menu.in_schedule(OnExit(AppState::Menu)))
// 0.11
app.add_systems(OnExit(AppState::Menu), exit_menu)

// 0.10
render_app.add_systems((a, b).in_set(RenderSet::Queue))
// 0.11
render_app.add_systems(Render, (a, b).in_set(RenderSet::Queue))
```

Set configuration now also accepts a schedule:

```rust
// 0.10
app.configure_set(A.in_schedule(PostUpdate).after(B))
// 0.11
app.configure_set(PostUpdate, A.after(B))

// 0.10
app.configure_set(A.after(B))
// 0.11 (Update is no longer implied by default)
app.configure_set(Update, A.after(B))

// 0.10
app.configure_sets((A, B).in_schedule(PostUpdate).after(C))
// 0.11
app.configure_sets(PostUpdate, (A, B).after(C))

// 0.10
app.configure_sets((A, B).after(C))
// 0.11 (Update is no longer implied by default)
app.configure_sets(Update, (A, B).after(C))
```

### [bevy_audio: ECS-based API redesign](https://github.com/bevyengine/bevy/pull/8424)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Audio</div>
</div>

```rust
// 0.10

/// Need to store handles somewhere
#[derive(Resource)]
struct MyMusic {
    sink: Handle<AudioSink>,
}

fn play_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    mut commands: Commands,
) {
    let weak_handle = audio.play_with_settings(
        asset_server.load("music.ogg"),
        PlaybackSettings::LOOP.with_volume(0.5),
    );
    // upgrade to strong handle and store it
    commands.insert_resource(MyMusic {
        sink: audio_sinks.get_handle(weak_handle),
    });
}

fn toggle_pause_music(
    audio_sinks: Res<Assets<AudioSink>>,
    mymusic: Option<Res<MyMusic>>,
) {
    if let Some(mymusic) = &mymusic {
        if let Some(sink) = audio_sinks.get(&mymusic.sink) {
            sink.toggle();
        }
    }
}

// 0.11

/// Marker component for our music entity
#[derive(Component)]
struct MyMusic;

fn play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(0.5)),
        },
        MyMusic,
    ));
}

fn toggle_pause_music(
    // `AudioSink` will be inserted by Bevy when the audio starts playing
    query_music: Query<&AudioSink, With<MyMusic>>,
) {
    if let Ok(sink) = query_music.get_single() {
        sink.toggle();
    }
}
```

### [Allow tuples and single plugins in `add_plugins`, deprecate `add_plugin`](https://github.com/bevyengine/bevy/pull/8097)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">App</div>
</div>

Replace `app.add_plugin(plugin)` calls with `app.add_plugins(plugin)`.

### [Improve shader import model](https://github.com/bevyengine/bevy/pull/5703)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Shaders that don't use `#import` directives should work without changes.

The most notable user-facing difference is that imported functions/variables/etc need to be qualified at point of use, and there's no "leakage" of visible stuff into your shader scope from the imports of your imports, so if you used things imported by your imports, you now need to import them directly and qualify them.

The current strategy of including/'spreading' `mesh_vertex_output` directly into a struct doesn't work any more, so these need to be modified as per the examples (e.g. `color_material.wgsl`, or many others). Mesh data is assumed to be in bindgroup 2 by default, if mesh data is bound into bindgroup 1 instead then the shader def `MESH_BINDGROUP_1` needs to be added to the pipeline `shader_defs`.

```rust
// 0.10
struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
}
@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {}

// 0.11
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {}
```

If you were importing something like `mesh_view_bindings` but only for the `globals` uniform buffer you can now import it directly.

```rust
// 0.10
#import bevy_pbr::mesh_view_bindings
// use globals.time after this

// 0.11
#import bevy_pbr::mesh_view_bindings globals
// globals is now in scope, but nothing else is imported
```

### [Flatten UI `Style` properties that use `Size` + remove `Size`](https://github.com/bevyengine/bevy/pull/8548)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The `size`, `min_size`, `max_size`, and `gap` properties have been replaced by the `width`, `height`, `min_width`, `min_height`, `max_width`, `max_height`, `row_gap`, and `column_gap` properties. Use the new properties instead.

### [Merge ScheduleRunnerSettings into ScheduleRunnerPlugin](https://github.com/bevyengine/bevy/pull/8585)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">App</div>
</div>

Instead of inserting the `ScheduleRunnerSettings` resource, configure the `ScheduleRunnerPlugin`

```rust
// 0.10
.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs(5)))
.add_plugin(ScheduleRunnerPlugin::default())

// 0.11
.add_plugin(ScheduleRunnerPlugin::run_loop(Duration::from_secs(5)))
```

### [Add support for custom glTF vertex attributes.](https://github.com/bevyengine/bevy/pull/5370)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

If you were instantiating `GltfPlugin` using the unit-like struct syntax, you must instead use `GltfPlugin::default()` as the type is no longer unit-like.

### [Delay asset hot reloading](https://github.com/bevyengine/bevy/pull/8503)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

```rust
// 0.10
.add_plugins(DefaultPlugins.set(AssetPlugin {
    watch_for_changes: true,
    ..default()
}))

// 0.11
.add_plugins(DefaultPlugins.set(AssetPlugin {
    // You can now give it a configurable delay. This is a safe default.
    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
    ..default()
}))
```

### [Allow systems using Diagnostics to run in parallel](https://github.com/bevyengine/bevy/pull/8677)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Diagnostics</div>
</div>

- Register `Diagnostic`’s using the new `app.register_diagnostic(Diagnostic::new(DIAGNOSTIC_ID, "diagnostic_name", 10));`
- In systems for writing new measurements, change `mut diagnostics: ResMut<Diagnostics>` to `mut diagnostics: Diagnostics` to allow the systems to run in parallel.

```diff
- fn system(mut diagnostics: ResMut<Diagnostics>) {}
+ fn system(mut diagnostics: Diagnostics) {}
```

- In systems for reading measurements, change `diagnostics: Res<Diagnostics>` to `diagnostics: Res<DiagnosticsStore>`.

```diff
- fn system(diagnostics: Res<Diagnostics>) {}
+ fn system(diagnostics: Res<DiagnosticsStore>) {}
```

### [Log to stderr instead of stdout](https://github.com/bevyengine/bevy/pull/8886)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Diagnostics</div>
</div>

On unix systems, when printing logs like `info!`, `trace!`, `error!`, etc, read from `stderr` instead of from `stdout`

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

`ChangeTrackers` has been removed. Use `Ref<T>` queries instead.

### [Check for conflicting accesses in `assert_is_system`](https://github.com/bevyengine/bevy/pull/8154)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The functions `assert_is_system` and `assert_is_read_only_system` (in `bevy_ecs::system`) now panic if the passed system has invalid world accesses. Any tests that called this function on a system with invalid accesses will now fail. Either fix the system’s conflicting accesses, or specify that the test is meant to fail:

- For regular tests (that is, functions annotated with `#[test]`), add the `#[should_panic]` attribute to the function.
- For documentation tests, add `should_panic` to the start of the code block: ` ```should_panic`

### [Remove base set error variants of `ScheduleBuildError`](https://github.com/bevyengine/bevy/pull/8269)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

With the removal of base sets, the related variants of `ScheduleBuildError` have also been removed. If you were handling any of them you can safely remove the code handling them.

### [Remove `#[system_param(ignore)]` and `#[world_query(ignore)]`](https://github.com/bevyengine/bevy/pull/8265)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The attributes `#[system_param(ignore)]` and `#[world_query(ignore)]` have been removed. If you were using either of these with `PhantomData` fields, you can simply remove the attribute:

```rust
#[derive(SystemParam)]
struct MyParam<'w, 's, Marker> {
    ...
    // 0.10
    #[system_param(ignore)]
    _marker: PhantomData<Marker>,
    // 0.11
    _marker: PhantomData<Marker>,
}

#[derive(WorldQuery)]
struct MyQuery<Marker> {
    ...
    // 0.10
    #[world_query(ignore)]
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
#[derive(WorldQuery)]
struct MyQuery {
   #[world_query(ignore)]
    str: String,
}
// 0.11
#[derive(WorldQuery)]
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

Use `State::get` instead of accessing the tuple field directly.

### [Only trigger state transitions if `next_state != old_state`](https://github.com/bevyengine/bevy/pull/8359)

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
```

```rust
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
fn my_system(component_id: Local<ComponentIdFor<MyComponent>>) {
    let component_id = **component_id;
}

// 0.11
fn my_system(component_id: ComponentIdFor<MyComponent>) {
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

`EntityRef::world` has been removed to make `EntityRef` sound to use as a query result. If you were retrieving `EntityRef` via `World::entity` or `World::get_entity`. Save a copy of the reference to the `World` before calling `World::entity`.

```rust
// In 0.10
let entity_ref = world.entity(id);
let world_2 = entity_ref.world();

// In 0.11
let world_2 = &world;
let entity_ref = world.entity(id);
```

### [Move AppTypeRegistry to bevy_ecs](https://github.com/bevyengine/bevy/pull/8901)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Reflection</div>
    <div class="migration-guide-area-tag">App</div>
</div>

If you were **not** using a `prelude::*` to import `AppTypeRegistry`, you should update your imports:

```rust
// 0.10
use bevy::app::AppTypeRegistry;
// 0.11
use bevy::ecs::reflect::AppTypeRegistry
```

### [Make scene handling of entity references robust](https://github.com/bevyengine/bevy/pull/7335)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Scenes</div>
</div>

`MapEntities` implementations must change from a `&EntityMap` parameter to a `&mut EntityMapper` parameter and can no longer return a `Result`. Finally, they should switch from calling `EntityMap::get` to calling `EntityMapper::get_or_reserve`.

### [Rename map_entities and map_specific_entities](https://github.com/bevyengine/bevy/pull/7570)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Scenes</div>
</div>

In `bevy_ecs`, `ReflectMapEntities::map_entites` now requires an additional `entities` parameter to specify which entities it applies to. To keep the old behavior, use the new `ReflectMapEntities::map_all_entities`, but consider if passing the entities in specifically might be better for your use case to avoid bugs.

### [Require `#[derive(Event)]` on all Events](https://github.com/bevyengine/bevy/pull/7086)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Add the `#[derive(Event)]` macro for events. Third-party types used as events should be wrapped in a newtype.

### [Fix boxed labels](https://github.com/bevyengine/bevy/pull/8436)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The `ScheduleLabel` trait has been refactored to no longer depend on the traits `std::any::Any`, `bevy_utils::DynEq`, and `bevy_utils::DynHash`.
Any manual implementations will need to implement new trait methods instead.

```rust
impl ScheduleLabel for MyType {
    // 0.10
    fn dyn_clone(&self) -> Box<dyn ScheduleLabel> { ... }

    // 0.11
    fn dyn_clone(&self) -> Box<dyn ScheduleLabel> { ... }

    fn as_dyn_eq(&self) -> &dyn DynEq {
        self
    }

    // No, `mut state: &mut` is not a typo.
    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        self.hash(&mut state);
        // Hashing the TypeId isn't strictly necessary, but it prevents collisions.
        TypeId::of::<Self>().hash(&mut state);
    }
}
```

### [Remove `OnUpdate` system set](https://github.com/bevyengine/bevy/pull/8260)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Replace `OnUpdate` with `run_if(in_state(xxx))`.

### [Document query errors](https://github.com/bevyengine/bevy/pull/8692)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`QueryEntityError::QueryDoesNotMatch`'s display message changed from "The given entity does not have the requested component." to "The given entity's components do not match the query.".

### [Update syn, encase, glam and hexasphere](https://github.com/bevyengine/bevy/pull/8573)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Using `#[bundle]` attribute when deriving `Bundle` for nested bundles now throws an error. It was already not required since version 0.9, see [the migration guide](https://bevyengine.org/learn/migration-guides/0.8-0.9/#implement-bundle-for-component-use-bundle-tuples-for-insertion).

```rust
#[derive(Bundle)]
struct PlayerBundle {
    #[bundle] // Remove this line
    sprite_bundle: SpriteBundle,
    collider: Collider,
}
```

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

### [Rename Interaction::Clicked -> Interaction::Pressed](https://github.com/bevyengine/bevy/pull/9027)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Input</div>
    <div class="migration-guide-area-tag">UI</div>
</div>

Rename all instances of Interaction::Clicked -> Interaction::Pressed

### [Don't ignore additional entries in `UntypedReflectDeserializerVisitor`](https://github.com/bevyengine/bevy/pull/7112)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

If you were deserializing `Box<dyn Reflect>` values with multiple entries (i.e. entries other than `"type": { /* fields */ }`) you should remove them or deserialization will fail.

### [`FromReflect` Ergonomics Implementation](https://github.com/bevyengine/bevy/pull/6056)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

`FromReflect` is now automatically derived within the `Reflect` derive macro. Items with both derives will need to remove the `FromReflect` one.

```rust
// 0.10
#[derive(Reflect, FromReflect)]
struct Foo;

// 0.11
#[derive(Reflect)]
struct Foo;
```

If using a manual implementation of `FromReflect` and the `Reflect` derive, users will need to opt-out of the automatic implementation.

```rust
// 0.10
#[derive(Reflect)]
struct Foo;

impl FromReflect for Foo {/* ... */}

// 0.11
#[derive(Reflect)]
#[reflect(from_reflect = false)]
struct Foo;

impl FromReflect for Foo {/* ... */}
```

### [bevy_reflect: stable type path v2](https://github.com/bevyengine/bevy/pull/7184)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

- Implementors of `Asset`, `Material` and `Material2d` now also need to derive `TypePath`.
- Manual implementors of `Reflect` will need to implement the new `get_type_path` method.
- To register types with generic parameters with the type registry, generic parameters must also implement `TypePath` even if they are only used for a `#[reflect(ignore)]` field.

  ```rust
  // 0.10
  struct MyType {}

  #[derive(Reflect)]
  struct MyTypeWithGeneric<A>{
      #[reflect(ignore)]
      _phantom: PhantomData<A>
  }

  fn main() {
      App::new().add_plugins(DefaultPlugins)
          .register_type::<MyTypeWithGeneric<MyType>>() // in 0.11 this would error
          .run();
  }

  // 0.11
  #[derive(TypePath)] // New. Can also just use #[derive(Reflect)]
  struct MyType {}

  #[derive(Reflect)]
  struct MyTypeWithGeneric<A: TypePath>{ // changed
      #[reflect(ignore)]
      _phantom: PhantomData<A>
  }

  fn main() {
      App::new().add_plugins(DefaultPlugins)
          .register_type::<MyTypeWithGeneric<MyType>>()
          .run();
  }
  ```

  If you don't own a type you may need to wrap it in a newtype and manually implement `TypePath` for the newtype.

  ```rust
  // 0.10
  use other_crate::RemoteType;

  #[derive(Reflect, Default)]
  struct MyTypeWithGeneric<A>{
      #[reflect(ignore)]
      _phantom: PhantomData<A>
  }

  fn main() {
      App::new().add_plugins(DefaultPlugins)
          .register_type::<MyTypeWithGeneric<RemoteType>>()
          .run();
  }

  // 0.11
  use other_crate::RemoteType;

  #[derive(Default)]
  struct MyType(RemoteType);

  impl TypePath for MyType {
      fn type_path() -> &'static str {
          "my_crate::my_module::MyType"
      }
      fn short_type_path() -> &'static str {
          "MyType"
      }
  }

  #[derive(Reflect, Default)]
  struct MyTypeWithGeneric<A: TypePath> {
      #[reflect(ignore)]
      _phantom: PhantomData<A>
  }

  fn main() {
      App::new().add_plugins(DefaultPlugins)
          .register_type::<MyTypeWithGeneric::<MyType>>()
          .run();
  }
  ```

### [Add `get_at_mut` to `bevy_reflect::Map` trait](https://github.com/bevyengine/bevy/pull/8691)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

Implementor of the `Map` trait now need to implement `get_at_mut`.

### [bevy_reflect: Better proxies](https://github.com/bevyengine/bevy/pull/6971)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

- The Dynamic types no longer take a string type name. Instead, they require a static reference to `TypeInfo`:

```rust
#[derive(Reflect)]
struct MyTupleStruct(f32, f32);

let mut dyn_tuple_struct = DynamicTupleStruct::default();
dyn_tuple_struct.insert(1.23_f32);
dyn_tuple_struct.insert(3.21_f32);

// 0.10
let type_name = std::any::type_name::<MyTupleStruct>();
dyn_tuple_struct.set_name(type_name);

// 0.11
let type_info = <MyTupleStruct as Typed>::type_info();
dyn_tuple_struct.set_represented_type(Some(type_info));
```

- `Reflect::get_type_info` has been renamed to `Reflect::represented_type_info` and now also returns an `Option<&'static TypeInfo>` (instead of just `&'static TypeInfo`):

```rust
// 0.10
let info: &'static TypeInfo = value.get_type_info();
// 0.11
let info: &'static TypeInfo = value.represented_type_info().unwrap();
```

- `TypeInfo::Dynamic` and `DynamicInfo` has been removed. Use `Reflect::is_dynamic instead`:

```rust
// 0.10
if matches!(value.get_type_info(), TypeInfo::Dynamic) {
  // ...
}
// 0.11
if value.is_dynamic() {
  // ...
}
```

### [Construct `Box<dyn Reflect>` from world for `ReflectComponent`](https://github.com/bevyengine/bevy/pull/7407)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
    <div class="migration-guide-area-tag">Scenes</div>
</div>

If you were manually creating `ReflectComponentFns` you now need to add a `from_world` function pointer.

```rust
ReflectComponentFns {
  from_world: |world| Box::new(MyComponent::from_world(world)),
  // where `from_world: fn(&mut World) -> Box<dyn Reflect>`
  // ...
}
```

### [Added Globals struct to prepass shader](https://github.com/bevyengine/bevy/pull/8070)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The `Globals` shader struct is now accessible in prepass shaders. If you were manually binding it you can remove that code and use the globals directly.

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

### [Remove unnecessary values Vec from DynamicUniformBuffer and DynamicStorageBuffer](https://github.com/bevyengine/bevy/pull/8299)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The `len()` accessor has been removed because internal changes made it non-trivial to compute. If you were using it and don't have a workaround, please create an issue.

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

### [Make glsl and spirv support optional](https://github.com/bevyengine/bevy/pull/8491)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- If you want to use shaders in `spirv`, enable the `shader_format_spirv` feature
- If you want to use shaders in `glsl`, enable the `shader_format_glsl` feature

### [Change default tonemapping method](https://github.com/bevyengine/bevy/pull/8685)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The default tonemapper has been changed from ReinhardLuminance to TonyMcMapface. Explicitly set ReinhardLuminance on your cameras to get back the previous look.

TonyMcMapface requires the `ktx2`, `tonemapping_luts`, and `zstd` features, which are enabled by default. If you disable the default features and notice that your scene is pink, you can either add the `ktx2`, `tonemapping_luts`, and `zstd` features, or use a different tonemapper.

Of the tonemappers that don't require a lookup table (LUT), SomewhatBoringDisplayTransform is the closest to TonyMcMapface. LUT based tonemappers are preferable as they tend to be faster.

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

### [Fix `Plane` UVs / texture flip](https://github.com/bevyengine/bevy/pull/8878)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Flip the textures you use on `Plane` shapes.

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

### [Webgpu support](https://github.com/bevyengine/bevy/pull/8336)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">App</div>
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- `Plugin::setup` has been renamed `Plugin::cleanup`
- `Plugin::finish` has been added, and plugins adding pipelines should do it in this function instead of `Plugin::build`

```rust
// 0.10
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<MyResource>
            .add_systems(Update, my_system);

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app
            .init_resource::<RenderResourceNeedingDevice>()
            .init_resource::<OtherRenderResource>();
    }
}

// 0.11
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<MyResource>
            .add_systems(Update, my_system);

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app
            .init_resource::<OtherRenderResource>();
    }

    fn finish(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app
            .init_resource::<RenderResourceNeedingDevice>();
    }
}
```

### [Take example screenshots in CI](https://github.com/bevyengine/bevy/pull/8488)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`TimeUpdateStrategy::ManualDuration`'s meaning has changed. Instead of setting time to `Instant::now()` plus the given duration, it sets time to last update plus the given duration.

### [Compute `vertex_count` for indexed meshes on `GpuMesh`](https://github.com/bevyengine/bevy/pull/8460)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`vertex_count` is now stored directly on `GpuMesh` instead of `GpuBufferInfo::NonIndexed`.

### [Built-in skybox](https://github.com/bevyengine/bevy/pull/8275)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Flip `EnvironmentMapLight` maps if needed to match how they previously rendered (which was backwards).

### [Left-handed y-up cubemap coordinates](https://github.com/bevyengine/bevy/pull/8122)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

When sampling from the point light shadow cubemap, use the (expected) light to fragment direction vector but negate the z coordinate. Previously, you would have used the fragment to light direction vector.

### [Add `Aabb` calculation for `Sprite`, `TextureAtlasSprite` and `Mesh2d`](https://github.com/bevyengine/bevy/pull/7885)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- 2D entities are now subject to frustum culling, check your 2D camera's z coordinate and projection `far` if some of them are not rendered anymore
- In particular, 2D entities with negative z values are now culled by frustum culling with the `default` `Camera2dBundle`. We plan on re-adding support for negative values with the default 2D camera in version `0.11.1`.

### [Add morph targets](https://github.com/bevyengine/bevy/pull/8158)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Animation</div>
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- `MeshPipeline` now has a single `mesh_layouts` field rather than separate `mesh_layout` and `skinned_mesh_layout` fields. You should handle all possible mesh bind group layouts in your implementation
- You should also handle properly the new `MORPH_TARGETS` shader def and mesh pipeline key. A new function is exposed to make this easier: `setup_moprh_and_skinning_defs`
- The `MeshBindGroup` is now `MeshBindGroups`, cached bind groups are now accessed through the `get` method.

### [bevy_scene: Add `SceneFilter`](https://github.com/bevyengine/bevy/pull/6793)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Scenes</div>
</div>

`DynamicScene::from_scene` and `DynamicScene::from_world` no longer require an `AppTypeRegistry` reference:

```rust
// 0.10
let registry = world.resource::<AppTypeRegistry>();
let dynamic_scene = DynamicScene::from_world(&world, registry);
// let dynamic_scene = DynamicScene::from_scene(&scene, registry);

// 0.11
let dynamic_scene = DynamicScene::from_world(&world);
// let dynamic_scene = DynamicScene::from_scene(&scene);
```

Removed `DynamicSceneBuilder::from_world_with_type_registry`. Now the registry is automatically taken from the given world:

```rust
// 0.10
let registry = world.resource::<AppTypeRegistry>();
let builder = DynamicSceneBuilder::from_world_with_type_registry(&world, registry);

// 0.11
let builder = DynamicSceneBuilder::from_world(&world);
```

### [(De) serialize resources in scenes](https://github.com/bevyengine/bevy/pull/6846)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Scenes</div>
</div>

The scene format has been changed, the user may not be able to use scenes saved prior to this version due to the resources scene field being missing.

### [Fix look_to variable naming](https://github.com/bevyengine/bevy/pull/8627)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Transform</div>
</div>

`Transform::look_to` method changed default value of `direction.try_normalize()` from `Vec3::Z` to `Vec3::NEG_Z`

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

- The `UiSystem::Flex` system set has been renamed to `UiSystem::Layout`.
- It is not possible to use the struct literal update syntax in const time with `Style` anymore, since one of its field implements `Drop`, doing so would raise a "the destructor for this type cannot be evaluated in constants" error (see [this issue](https://github.com/bevyengine/bevy/issues/9095)).

```rust
// 0.10
pub const ABSOLUTE_STYLE: Style = Style {
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};

// 0.11
pub const ABSOLUTE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute;
    style
};
```

### [`MeasureFunc` improvements](https://github.com/bevyengine/bevy/pull/8402)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- `CalculatedSize` has been renamed to `ContentSize`.
- The `upsert_leaf` function has been removed from `UiSurface` and replaced with `update_measure` which updates the `MeasureFunc` without node insertion.
- The `dyn_clone` method has been removed from the `Measure` trait.
- The new function of `CalculatedSize` has been replaced with the method `set`.
- `ImageBundle` and `TextBundle` don't implement `Clone` anymore. [You can either](https://github.com/bevyengine/bevy-website/issues/699):

    1. Wrap yourself the bundle type and implement `Clone` by skipping cloning the `ContentSize` field.
    2. Use a closure instead of `clone`:

    ```rust
    // 0.10
    let circle = ImageBundle {
        style: image_style,
        image: materials.circle.clone(),
        ..Default::default()
    };
    commands.spawn(circle.clone());
    commands.spawn(circle.clone());
    commands.spawn(circle.clone());
    commands.spawn(circle.clone());

    // 0.11
    let circle = || ImageBundle {
        style: image_style,
        image: materials.circle.clone(),
        ..Default::default()
    };
    commands.spawn(circle());
    commands.spawn(circle());
    commands.spawn(circle());
    commands.spawn(circle());
    ```

### [Divide by `UiScale` when converting UI coordinates from physical to logical](https://github.com/bevyengine/bevy/pull/8720)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

Physical UI coordinates are now divided by both the `UiScale` and the window’s scale factor to compute the logical sizes and positions of UI nodes.

This ensures that UI Node size and position values, held by the `Node` and `GlobalTransform` components, conform to the same logical coordinate system as the style constraints from which they are derived, irrespective of the current `scale_factor` and `UiScale`.

### [`NoWrap` `Text` feature](https://github.com/bevyengine/bevy/pull/8947)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

`bevy_text::text::BreakLineOn` has a new variant `NoWrap` that disables text wrapping for the `Text`.
Text wrapping can also be disabled using the `with_no_wrap` method of `TextBundle`.

### [Replace the local text queues in the text systems with flags stored in a component](https://github.com/bevyengine/bevy/pull/8549)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

`TextBundle` has a new field `text_flag` of type `TextFlags`.

### [Split UI `Overflow` by axis](https://github.com/bevyengine/bevy/pull/8095)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The `Style` property `Overflow` is now a struct with `x` and `y` fields, that allow for per-axis overflow control.

Use these helper functions to replace the variants of `Overflow`:

- Replace `Overflow::Visible` with `Overflow::visible()`
- Replace `Overflow::Hidden` with `Overflow::clip()`

### [`text_system` split](https://github.com/bevyengine/bevy/pull/7779)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

`ImageBundle` has a new field `image_size` of type `UiImageSize` which contains the size of the image bundle's texture and is updated automatically by `update_image_calculated_size_system`.

### [Update ahash and hashbrown](https://github.com/bevyengine/bevy/pull/8623)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">No area label</div>
</div>

If you were using hashes to an asset or using one of the fixed hasher exposed by Bevy with a previous version, you will have to update the hashes

### [Move bevy_ui accessibility systems to `PostUpdate`](https://github.com/bevyengine/bevy/pull/8653)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">No area label</div>
</div>

`bevy_ui` accessibility systems have been moved to `PostUpdate`, if you were scheduling systems relative to these, make sure you now do it in `PostUpdate`.

</div>

+++
title = "0.11 to 0.12"
weight = 7
sort_by = "weight"
template = "docs-section.html"
page_template = "docs-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.11 to 0.12"
+++

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.
<div class="migration-guide">

### [API updates to the AnimationPlayer](https://github.com/bevyengine/bevy/pull/9002)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Animation</div>
</div>

- Removed `set_elapsed`.
- Removed `stop_repeating` in favour of `AnimationPlayer::set_repeat(RepeatAnimation::Never)`.
- Introduced `seek_to` to seek to a given timestamp inside of the animation.
- Introduced `seek_time` accessor for the `PlayingAnimation::seek_to`.
- Introduced `AnimationPlayer::replay` to reset the `PlayingAnimation` to a state where no time has elapsed.

### [Fix run-once runners](https://github.com/bevyengine/bevy/pull/10195)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">App</div>
</div>

`app.ready()` has been replaced by `app.plugins_state()` which will return more details on the current state of plugins in the app

### [Copy on Write AssetPaths](https://github.com/bevyengine/bevy/pull/9729)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

```rust
// 0.11
AssetPath::new("logo.png", None);

// 0.12
AssetPath::new("logo.png");

// 0.11
AssetPath::new("scene.gltf", Some("Mesh0"));

// 0.12
AssetPath::new("scene.gltf").with_label("Mesh0");
```

`AssetPath` now serializes as `AssetPath("some_path.extension#Label")` instead of as `AssetPath { path: "some_path.extension", label: Some("Label) }`

### [Removed `anyhow`](https://github.com/bevyengine/bevy/pull/10003)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

- `anyhow` is no longer exported by `bevy_asset`; Add it to your own project (if required).
- `AssetLoader` and `AssetSaver` have an associated type `Error`; Define an appropriate error type (e.g., using `thiserror`), or use a pre-made error type (e.g., `anyhow::Error`). Note that using `anyhow::Error` is a drop-in replacement.
- `AssetLoaderError` has been removed; Define a new error type, or use an alternative (e.g., `anyhow::Error`)
- All the first-party `AssetLoader`’s and `AssetSaver`’s now return relevant (and narrow) error types instead of a single ambiguous type; Match over the specific error type, or encapsulate (`Box<dyn>`, `thiserror`, `anyhow`, etc.)

### [More ergonomic spatial audio](https://github.com/bevyengine/bevy/pull/9800)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Audio</div>
</div>

Spatial audio now automatically uses the transform of the `AudioBundle` and of an entity with a `SpatialListener` component.

If you were manually scaling emitter/listener positions, you can use the `spatial_scale` field of `AudioPlugin` instead.

```rust
// 0.11
commands.spawn(
    SpatialAudioBundle {
        source: asset_server.load("sounds/Windless Slopes.ogg"),
        settings: PlaybackSettings::LOOP,
        spatial: SpatialSettings::new(listener_position, gap, emitter_position),
    },
);

fn update(
    emitter_query: Query<(&Transform, &SpatialAudioSink)>,
    listener_query: Query<&Transform, With<Listener>>,
) {
    let listener = listener_query.single();

    for (transform, sink) in &emitter_query {
        sink.set_emitter_position(transform.translation);
        sink.set_listener_position(*listener, gap);
    }
}

// 0.12
commands.spawn((
    SpatialBundle::from_transform(Transform::from_translation(emitter_position)),
    AudioBundle {
        source: asset_server.load("sounds/Windless Slopes.ogg"),
        settings: PlaybackSettings::LOOP.with_spatial(true),
    },
));

commands.spawn((
    SpatialBundle::from_transform(Transform::from_translation(listener_position)),
    SpatialListener::new(gap),
));
```

### [Simplify parallel iteration methods](https://github.com/bevyengine/bevy/pull/8854)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The method `QueryParIter::for_each_mut` has been deprecated and is no longer functional. Use `for_each` instead, which now supports mutable queries.

```rust
// 0.11:
query.par_iter_mut().for_each_mut(|x| ...);

// 0.12:
query.par_iter_mut().for_each(|x| ...);
```

The method `QueryParIter::for_each` now takes ownership of the `QueryParIter`, rather than taking a shared reference.

```rust
// 0.11:
let par_iter = my_query.par_iter().batching_strategy(my_batching_strategy);
par_iter.for_each(|x| {
    // ...Do stuff with x...
    par_iter.for_each(|y| {
        // ...Do nested stuff with y...
    });
});

// 0.12:
my_query.par_iter().batching_strategy(my_batching_strategy).for_each(|x| {
    // ...Do stuff with x...
    my_query.par_iter().batching_strategy(my_batching_strategy).for_each(|y| {
        // ...Do nested stuff with y...
    });
});
```

### [Fix safety invariants for `WorldQuery::fetch` and simplify cloning](https://github.com/bevyengine/bevy/pull/8246)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

__`fetch` invariants__

The function `WorldQuery::fetch` has had the following safety invariant added:

> If `update_component_access` includes any mutable accesses, then the caller must ensure that `fetch` is called no more than once for each `entity`/`table_row` in each archetype.
> </br>
> If `Self` implements `ReadOnlyWorldQuery`, then this can safely be called multiple times.

This invariant was always required for soundness, but was previously undocumented. If you called this function manually anywhere, you should check to make sure that this invariant is not violated.

__Removed `clone_fetch`__

The function `WorldQuery::clone_fetch` has been removed. The associated type `WorldQuery::Fetch` now has the bound `Clone`.

```rust
// 0.11
struct MyFetch<'w> { ... }

unsafe impl WorldQuery for MyQuery {
    ...
    type Fetch<'w> = MyFetch<'w>
    unsafe fn clone_fetch<'w>(fetch: &Self::Fetch<'w>) -> Self::Fetch<'w> {
        MyFetch {
            field1: fetch.field1,
            field2: fetch.field2.clone(),
            ...
        }
    }
}

// 0.12
#[derive(Clone)]
struct MyFetch<'w> { ... }

unsafe impl WorldQuery for MyQuery {
    ...
    type Fetch<'w> = MyFetch<'w>;
}
```

### [Opt-out `multi-threaded` feature flag](https://github.com/bevyengine/bevy/pull/9269)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The `multi-threaded` feature in `bevy_ecs` and `bevy_tasks` is no longer enabled by default. However, this remains a default feature for the umbrella `bevy` crate. If you depend on `bevy_ecs` or `bevy_tasks` directly, you should consider enabling this to allow systems to run in parallel.

### [Refactor build_schedule and related errors](https://github.com/bevyengine/bevy/pull/9579)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`ScheduleBuildError` now has strings in more of its variants. You may need to adjust code that is handling these variants.

### [Replaced EntityMap with HashMap](https://github.com/bevyengine/bevy/pull/9461)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Calls to `EntityMap::world_scope` can be directly replaced with the following:
`map.world_scope(&mut world)` -> `world.world_scope(&mut map)`
- Calls to legacy `EntityMap` methods such as `EntityMap::get` must explicitly include de/reference symbols:
`let entity = map.get(parent);` -> `let &entity = map.get(&parent);`

### [Rename `ManualEventIterator`](https://github.com/bevyengine/bevy/pull/9592)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The type `ManualEventIterator` has been renamed to `EventIterator`. Additonally, `ManualEventIteratorWithId` has been renamed to `EventIteratorWithId`.

### [Replaced `EntityCommand` Implementation for `FnOnce`](https://github.com/bevyengine/bevy/pull/9604)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

__1. New-Type `FnOnce`__

Create an `EntityCommand` type which implements the method you previously wrote:

```rust
pub struct ClassicEntityCommand<F>(pub F);

impl<F> EntityCommand for ClassicEntityCommand<F>
where
    F: FnOnce(Entity, &mut World) + Send + 'static,
{
    fn apply(self, id: Entity, world: &mut World) {
        (self.0)(id, world);
    }
}

commands.add(ClassicEntityCommand(|id: Entity, world: &mut World| {
    /* ... */
}));
```

__2. Extract `(Entity, &mut World)` from `EntityMut`__

The method `into_world_mut` can be used to gain access to the `World` from an `EntityMut`.

```rust
let old = |id: Entity, world: &mut World| {
    /* ... */
};

let new = |mut entity: EntityMut| {
    let id = entity.id();
    let world = entity.into_world_mut();
    /* ... */
};
```

### [Move schedule name into `Schedule`](https://github.com/bevyengine/bevy/pull/9600)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`Schedule::new` and `App::add_schedule`

```rust
// 0.11
let schedule = Schedule::new();
app.add_schedule(MyLabel, schedule);

// 0.12
let schedule = Schedule::new(MyLabel);
app.add_schedule(schedule);
```

if you aren’t inserting the schedule into the world and are using the schedule directly you can use the default constructor which reuses a default label.

```rust
// 0.11
let schedule = Schedule::new();
schedule.run(world);

// 0.12
let schedule = Schedule::default();
schedule.run(world);
```

`Schedules:insert`

```rust
// 0.11
let schedule = Schedule::new();
schedules.insert(MyLabel, schedule);

// 0.12
let schedule = Schedule::new(MyLabel);
schedules.insert(schedule);
```

`World::add_schedule`

```rust
// 0.11
let schedule = Schedule::new();
world.add_schedule(MyLabel, schedule);

// 0.12
let schedule = Schedule::new(MyLabel);
world.add_schedule(schedule);
```

### [Refactor `EventReader::iter` to `read`](https://github.com/bevyengine/bevy/pull/9631)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Existing usages of `EventReader::iter` and `EventReader::iter_with_id` will have to be changed to `EventReader::read` and `EventReader::read_with_id` respectively.
- Existing usages of `ManualEventReader::iter` and `ManualEventReader::iter_with_id` will have to be changed to `ManualEventReader::read` and `ManualEventReader::read_with_id` respectively.

### [Replace `IntoSystemSetConfig` with `IntoSystemSetConfigs`](https://github.com/bevyengine/bevy/pull/9247)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Use `App::configure_sets` instead of `App::configure_set`
- Use `Schedule::configure_sets` instead of `Schedule::configure_set`

### [Moved `get_component(_unchecked_mut)` from `Query` to `QueryState`](https://github.com/bevyengine/bevy/pull/9686)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- `use bevy_ecs::system::QueryComponentError;` -> `use bevy_ecs::query::QueryComponentError;`

### [Fix naming on "tick" Column and ComponentSparseSet methods](https://github.com/bevyengine/bevy/pull/9744)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The following method names were renamed, from `foo_ticks_bar` to `foo_tick_bar` (`ticks` is now singular, `tick`):

- `ComponentSparseSet::get_added_ticks` → `get_added_tick`
- `ComponentSparseSet::get_changed_ticks` → `get_changed_tick`
- `Column::get_added_ticks` → `get_added_tick`
- `Column::get_changed_ticks` → `get_changed_tick`
- `Column::get_added_ticks_unchecked` → `get_added_tick_unchecked`
- `Column::get_changed_ticks_unchecked` → `get_changed_tick_unchecked`

### [Return a boolean from `set_if_neq`](https://github.com/bevyengine/bevy/pull/9801)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The trait method `DetectChangesMut::set_if_neq` now returns a boolean value indicating whether or not the value was changed. If you were implementing this function manually, you must now return `true` if the value was overwritten and `false` if the value was not.

### [Rename RemovedComponents::iter/iter_with_id to read/read_with_id](https://github.com/bevyengine/bevy/pull/9778)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Rename calls of RemovedComponents::iter/iter_with_id to  read/read_with_id

Replace IntoIterator iteration (&mut <RemovedComponents>) with .read()

### [Remove States::variants and remove enum-only restriction its derive](https://github.com/bevyengine/bevy/pull/9945)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- `States::variants` no longer exists. If you relied on this function, consider using a library that provides enum iterators.

### [Replace all labels with interned labels](https://github.com/bevyengine/bevy/pull/7762)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Replace `BoxedScheduleLabel` and `Box<dyn ScheduleLabel>` with `InternedScheduleLabel` or `Interned<dyn ScheduleLabel>`.
- Replace `BoxedSystemSet` and `Box<dyn SystemSet>` with `InternedSystemSet` or `Interned<dyn SystemSet>`.
- Replace `AppLabelId` with `InternedAppLabel` or `Interned<dyn AppLabel>`.
- Types manually implementing `ScheduleLabel`, `AppLabel` or `SystemSet` need to implement:
  - `dyn_hash` directly instead of implementing `DynHash`
  - `as_dyn_eq`

- Pass labels to `World::try_schedule_scope`, `World::schedule_scope`, `World::try_run_schedule`. `World::run_schedule`, `Schedules::remove`, `Schedules::remove_entry`, `Schedules::contains`, `Schedules::get` and `Schedules::get_mut` by value instead of by reference.

### [Only run event systems if they have tangible work to do](https://github.com/bevyengine/bevy/pull/7728)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">App</div>
</div>

`Events<T>::update_system` has been split off from the the type and can be found at `bevy_ecs::event::event_update_system`.

### [Allow disjoint mutable world access via `EntityMut`](https://github.com/bevyengine/bevy/pull/9419)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Reflection</div>
</div>

**Note for maintainers: ensure that the guide for #9604 is updated accordingly.**

Removed the method `EntityRef::world`, to fix a soundness issue with queries. If you need access to `&World` while using an `EntityRef`, consider passing the world as a separate parameter.

`EntityMut` can no longer perform ‘structural’ world mutations, such as adding or removing components, or despawning the entity. Additionally, `EntityMut::world`, `EntityMut::world_mut` , and `EntityMut::world_scope` have been removed.
Instead, use the newly-added type `EntityWorldMut`, which is a helper type for working with `&mut World`.

### [Make builder types take and return `Self`](https://github.com/bevyengine/bevy/pull/10001)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Scenes</div>
</div>

When using `bevy_ecs::DynamicSceneBuilder` and `bevy_ecs::SceneBuilder`, instead of binding the builder to a variable, directly use it. Methods on those types now consume `Self`, so you will need to re-bind the builder if you don’t `build` it immediately.

```rust
// 0.11
let mut scene_builder = DynamicSceneBuilder::from_world(&world);
let scene = scene_builder.extract_entity(a).extract_entity(b).build();

// 0.12
let scene = DynamicSceneBuilder::from_world(&world)
   .extract_entity(a)
   .extract_entity(b)
   .build();
```

### [Change `AxisSettings` livezone default](https://github.com/bevyengine/bevy/pull/10090)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Input</div>
</div>

If the default 0.05 was relied on, the default or gamepad `AxisSettings` on the resource `GamepadSettings` will have to be changed.

### [Rename bevy_math::rects conversion methods](https://github.com/bevyengine/bevy/pull/9159)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Math</div>
</div>

Replace `Rect::as_urect` with `Rect::as_irect`, `Rect::as_rect` with `Rect::as_urect`, and `URect::as_urect` with `URect::as_irect`.

### [Remove the bevy_dylib feature](https://github.com/bevyengine/bevy/pull/9516)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Meta</div>
</div>

If you were using Bevy’s `bevy_dylib` feature, use Bevy’s `dynamic_linking` feature instead.

```shell
# 0.11
cargo run --features bevy/bevy_dylib

# 0.12
cargo run --features bevy/dynamic_linking
```

```toml
[dependencies]
# 0.11
bevy = { version = "0.11", features = ["bevy_dylib"] }

# 0.12
bevy = { version = "0.12", features = ["dynamic_linking"] }
```

### [Fix typo in NamedTypePathDef](https://github.com/bevyengine/bevy/pull/9102)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

Renamed NamedTypePathDef::Primtive to NamedTypePathDef::Primitive

```rust
// 0.11
let type_path = NamedTypePathDef::Primtive(ident);
// 0.12
let type_path = NamedTypePathDef::Primitive(ident);
```

### [Refactor `path` module of `bevy_reflect`](https://github.com/bevyengine/bevy/pull/8887)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

If you were matching on the `Err(ReflectPathError)` value returned by `GetPath` and `ParsedPath` methods, now only the parse-related errors and the offset are publicly accessible. You can always use the `fmt::Display` to get a clear error message, but if you need programmatic access to the error types, please open an issue.

### [Make it so `ParsedPath` can be passed to GetPath](https://github.com/bevyengine/bevy/pull/9373)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

`GetPath` now requires `Reflect`. This reduces a lot of boilerplate on bevy’s side. If you were implementing manually `GetPath` on your own type, please get in touch!

`ParsedPath::element[_mut]` isn’t an inherent method of `ParsedPath`, you must now import `ReflectPath`. This is only relevant if you weren’t importing the bevy prelude.

```diff
-use bevy::reflect::ParsedPath;
+use bevy::reflect::{ParsedPath, ReflectPath};

parsed_path.element(reflect_type).unwrap()
```

### [Remove TypeRegistry re-export rename](https://github.com/bevyengine/bevy/pull/9807)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

- `TypeRegistry` as re-exported by the wrapper `bevy` crate is now `TypeRegistryArc`
- `TypeRegistryInternal` as re-exported by the wrapper `bevy` crate is now `TypeRegistry`

### [Provide getters for fields of ReflectFromPtr](https://github.com/bevyengine/bevy/pull/9748)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

- `ReflectFromPtr::as_reflect_ptr` is now `ReflectFromPtr::as_reflect`
- `ReflectFromPtr::as_reflect_ptr_mut` is now `ReflectFromPtr::as_reflect_mut`

### [bevy_reflect: Fix ignored/skipped field order](https://github.com/bevyengine/bevy/pull/7575)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

- Fields marked `#[reflect(skip_serializing)]` now must implement `Default` or specify a custom default function with `#[reflect(default = "path::to::some_func")]`

```rust
#[derive(Reflect)]
struct MyStruct {
  #[reflect(skip_serializing)]
  #[reflect(default = "get_foo_default")]
  foo: Foo, // <- `Foo` does not impl `Default` so requires a custom function
  #[reflect(skip_serializing)]
  bar: Bar, // <- `Bar` impls `Default`
}

#[derive(Reflect)]
struct Foo(i32);

#[derive(Reflect, Default)]
struct Bar(i32);

fn get_foo_default() -> Foo {
  Foo(123)
}
```

- `SerializationData::new` has been changed to expect an iterator of `(usize, SkippedField)` rather than one of just `usize`

```rust
// 0.11
SerializationData::new([0, 3].into_iter());

// 0.12
SerializationData::new([
  (0, SkippedField::new(field_0_default_fn)),
  (3, SkippedField::new(field_3_default_fn)),
].into_iter());
```

- `Serialization::is_ignored_field` has been renamed to `Serialization::is_field_skipped`
- Fields marked `#[reflect(skip_serializing)]` are now included in deserialization output. This may affect logic that expected those fields to be absent.

### [Return URect instead of (UVec2, UVec2) in Camera::physical_viewport_rect](https://github.com/bevyengine/bevy/pull/9085)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

```rust
// 0.11
fn view_physical_camera_rect(camera_query: Query<&Camera>) {
    let camera = camera_query.single();
    let Some((min, max)) = camera.physical_viewport_rect() else { return };
    dbg!(min, max);
}

// 0.12
fn view_physical_camera_rect(camera_query: Query<&Camera>) {
    let camera = camera_query.single();
    let Some(URect { min, max }) = camera.physical_viewport_rect() else { return };
    dbg!(min, max);
}
```

### [Update `bevy_window::PresentMode` to mirror `wgpu::PresentMode`](https://github.com/bevyengine/bevy/pull/9230)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Handle `bevy_window::PresentMode::FifoRelaxed` when tweaking window present mode manually.

### [Split `ComputedVisibility` into two components to allow for accurate change detection and speed up visibility propagation](https://github.com/bevyengine/bevy/pull/9497)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The `ComputedVisibilty` component has been split into `InheritedVisiblity` and
`ViewVisibility`. Replace any usages of `ComputedVisibility::is_visible_in_hierarchy`
with `InheritedVisibility::get`, and replace `ComputedVisibility::is_visible_in_view`
with `ViewVisibility::get`.

```rust
// 0.11:
commands.spawn(VisibilityBundle {
    visibility: Visibility::Inherited,
    computed_visibility: ComputedVisibility::default(),
});

// 0.12:
commands.spawn(VisibilityBundle {
    visibility: Visibility::Inherited,
    inherited_visibility: InheritedVisibility::default(),
    view_visibility: ViewVisibility::default(),
});
```

```rust
// 0.11:
fn my_system(q: Query<&ComputedVisibilty>) {
    for vis in &q {
        if vis.is_visible_in_hierarchy() {

// 0.12:
fn my_system(q: Query<&InheritedVisibility>) {
    for inherited_visibility in &q {
        if inherited_visibility.get() {
```

```rust
// 0.11:
fn my_system(q: Query<&ComputedVisibilty>) {
    for vis in &q {
        if vis.is_visible_in_view() {

// 0.12:
fn my_system(q: Query<&ViewVisibility>) {
    for view_visibility in &q {
        if view_visibility.get() {
```

```rust
// 0.11:
fn my_system(mut q: Query<&mut ComputedVisibilty>) {
    for vis in &mut q {
        vis.set_visible_in_view();

// 0.12:
fn my_system(mut q: Query<&mut ViewVisibility>) {
    for view_visibility in &mut q {
        view_visibility.set();
```

### [Cleanup `visibility` module](https://github.com/bevyengine/bevy/pull/9850)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- The `check_visibility` system’s `Option<&NoFrustumCulling>` parameter has been replaced by  `Has<NoFrustumCulling>`, if you were calling it manually, you should change the type to match it

### [Update defaults for OrthographicProjection](https://github.com/bevyengine/bevy/pull/9878)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Migration guide steps from #9537 should be removed for next release.

### [Revert "Update defaults for OrthographicProjection (#9537)"](https://github.com/bevyengine/bevy/pull/9878)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Migration guide steps from #9537 should be removed for next release.

### [PCF For DirectionalLight/SpotLight Shadows](https://github.com/bevyengine/bevy/pull/8006)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Shadows cast by directional lights or spotlights now have smoother edges. To revert to the old behavior, add `ShadowFilteringMethod::Hardware2x2` to your cameras.

### [Deferred Renderer](https://github.com/bevyengine/bevy/pull/9258)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

<!-- TODO -->

### [`*_PREPASS` Shader Def Cleanup](https://github.com/bevyengine/bevy/pull/10136)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- When using functions from `bevy_pbr::prepass_utils` (`prepass_depth()`, `prepass_normal()`, `prepass_motion_vector()`) in contexts where these prepasses might be disabled, you should now wrap your calls with the appropriate `#ifdef` guards, (`#ifdef DEPTH_PREPASS`, `#ifdef NORMAL_PREPASS`, `#ifdef MOTION_VECTOR_PREPASS`) providing fallback logic where applicable.

### [allow extensions to StandardMaterial](https://github.com/bevyengine/bevy/pull/7820)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

manual implementations of `AsBindGroup` will need to be adjusted, the changes are pretty straightforward and can be seen in the diff for e.g. the `texture_binding_array` example.

### [Detect cubemap for dds textures](https://github.com/bevyengine/bevy/pull/10222)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

If you are matching on a `TextureError`, you will need to add a new branch to handle `TextureError::IncompleteCubemap`.

### [Add convenient methods for Image](https://github.com/bevyengine/bevy/pull/10221)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Replace calls to the `Image::size()` method with `size_f32()`.
Replace calls to the `Image::aspect_2d()` method with `aspect_ratio()`.

### [Fix fog color being inaccurate](https://github.com/bevyengine/bevy/pull/10226)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Colors in `FogSettings` struct (`color` and `directional_light_color`) are now sent to the GPU in linear space. If you were using `Color::rgb()`/`Color::rgba()` and would like to retain the previous colors, you can quickly fix it by switching to `Color::rgb_linear()`/`Color::rgba_linear()`.

### [Move skin code to a separate module](https://github.com/bevyengine/bevy/pull/9899)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">Animation</div>
</div>

Renamed skinning systems, resources and components:

- extract_skinned_meshes -> extract_skins
- prepare_skinned_meshes -> prepare_skins
- SkinnedMeshUniform -> SkinUniform
- SkinnedMeshJoints -> SkinIndex

### [Move scene spawner systems to SpawnScene schedule](https://github.com/bevyengine/bevy/pull/9260)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Scenes</div>
</div>

- Move scene spawner systems to a new SpawnScene schedule which is after Update and before PostUpdate (schedule order: [PreUpdate] [Update] [SpawnScene] [PostUpdate]), you might remove system ordering code related to scene spawning as the execution order has been guaranteed by bevy engine.

### [Remove Resource and add Debug to TaskPoolOptions](https://github.com/bevyengine/bevy/pull/9485)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Tasks</div>
</div>

If for some reason anyone is still using `TaskPoolOptions` as a Resource, they would now have to use a wrapper type:

```rust
#[derive(Resource)]
pub struct MyTaskPoolOptions(pub TaskPoolOptions);
```

### [Global TaskPool API improvements](https://github.com/bevyengine/bevy/pull/10008)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Tasks</div>
</div>

- Uses of `ComputeTaskPool::init`, `AsyncComputeTaskPool::init` and `IoTaskPool::init` should be changed to `::get_or_init`.

### [Unify `FixedTime` and `Time` while fixing several problems](https://github.com/bevyengine/bevy/pull/8964)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Time</div>
</div>

- Change all `Res<Time>` instances that access `raw_delta()`, `raw_elapsed()` and related methods to `Res<Time<Real>>` and `delta()`, `elapsed()`, etc.
- Change access to `period` from `Res<FixedTime>` to `Res<Time<Fixed>>` and use `delta()`.
- The default timestep has been changed from 60 Hz to 64 Hz. If you wish to restore the old behaviour, use `app.insert_resource(Time::<Fixed>::from_hz(60.0))`.
- Change `app.insert_resource(FixedTime::new(duration))` to `app.insert_resource(Time::<Fixed>::from_duration(duration))`
- Change `app.insert_resource(FixedTime::new_from_secs(secs))` to `app.insert_resource(Time::<Fixed>::from_seconds(secs))`
- Change `system.on_fixed_timer(duration)` to `system.on_timer(duration)`. Timers in systems placed in `FixedUpdate` schedule automatically use the fixed time clock.
- Change `ResMut<Time>` calls to `pause()`, `is_paused()`, `set_relative_speed()`  and related methods to `ResMut<Time<Virtual>>` calls. The API is the same, with the exception that `relative_speed()` will return the actual last ste relative speed, while `effective_relative_speed()` returns 0.0 if the time is paused and corresponds to the speed that was set when the update for the current frame started.

### [Change the default for the `measure_func` field of `ContentSize` to None.](https://github.com/bevyengine/bevy/pull/9346)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The default for `ContentSize` now sets its `measure_func` to `None`, instead of a fixed size measure that returns `Vec2::ZERO`.
The helper function `fixed_size` can be called with `ContentSize::fixed_size(Vec2::ZERO)` to get the previous behaviour.

### [Change `UiScale` to a tuple struct](https://github.com/bevyengine/bevy/pull/9444)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

Replace initialization of `UiScale` like `UiScale { scale: 1.0 }` with `UiScale(1.0)`

### [Cleanup some bevy_text pipeline.rs](https://github.com/bevyengine/bevy/pull/9111)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- The `ResMut<TextPipeline>` argument to  `measure_text_system` doesn’t exist anymore. If you were calling this system manually, you should remove the argument.
- The `{min,max}_width_content_size` fields of `TextMeasureInfo` are renamed to `min` and `max` respectively
- Other changes to `TextMeasureInfo` may also break your code if you were manually building it. Please consider using the new `TextMeasureInfo::from_text` to build one instead.
- `TextPipeline::create_text_measure` has been removed in favor of `TextMeasureInfo::from_text`

### [Remove `Val`'s `try_*` arithmetic methods](https://github.com/bevyengine/bevy/pull/9609)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

`Val`’s `try_*` arithmetic methods have been removed. To perform arithmetic on `Val`s deconstruct them using pattern matching.

### [Rename `Val` `evaluate` to `resolve` and implement viewport variant support](https://github.com/bevyengine/bevy/pull/9568)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- Renamed the following `Val` methods and added a `viewport_size` parameter:
  - `evaluate` to `resolve`
  - `try_add_with_size` to `try_add_with_context`
  - `try_add_assign_with_size` to `try_add_assign_with_context`
  - `try_sub_with_size` to `try_sub_with_context`
  - `try_sub_assign_with_size` to `try_sub_assign_with_context`

### [`TextLayoutInfo::size` should hold the drawn size of the text, and not a scaled value.](https://github.com/bevyengine/bevy/pull/7794)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The `size` value of `TextLayoutInfo` is stored in logical pixels and has been renamed to `logical_size`. There is no longer any need to divide by the window’s scale factor to get the logical size.

### [Have a separate implicit viewport node per root node + make viewport node `Display::Grid`](https://github.com/bevyengine/bevy/pull/9637)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- Bevy UI now lays out root nodes independently of each other in separate layout contexts. If you were relying on your root nodes being able to affect each other’s layouts, then you may need to wrap them in a single root node.
- The implicit viewport node (which contains each user-specified root node) is now `Display::Grid` with `align_items` and `justify_items` both set to `Start`. You may need to add `height: Val::Percent(100.)` to your root nodes if you were previously relying on being implicitly set.

### [Rename `num_font_atlases`  to `len`.](https://github.com/bevyengine/bevy/pull/9879)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The `num_font_atlases` method of `FontAtlasSet` has been renamed to `len`.

### [Add option to toggle window control buttons](https://github.com/bevyengine/bevy/pull/9083)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Windowing</div>
</div>

- Added an `enabled_buttons` member to the `Window` struct through which users can enable or disable specific window control buttons.

### [Improve `bevy_winit` documentation](https://github.com/bevyengine/bevy/pull/7609)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Windowing</div>
</div>

- `UpdateMode::Reactive { max_wait: .. }` -> `UpdateMode::Reactive { wait: .. }`
- `UpdateMode::ReactiveLowPower { max_wait: .. }` -> `UpdateMode::ReactiveLowPower { wait: .. }`

### [Remove `IntoIterator` impl for `&mut EventReader`](https://github.com/bevyengine/bevy/pull/9583)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">No area label</div>
</div>

- `&mut EventReader` does not implement `IntoIterator` anymore. replace `for foo in &mut events` by `for foo in events.iter()`

</div>

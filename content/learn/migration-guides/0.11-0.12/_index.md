+++
title = "0.11 to 0.12"
weight = 7
sort_by = "weight"
template = "docs-section.html"
page_template = "docs-section.html"
insert_anchor_links = "right"
aliases = ["learn/migration-guides/0.11-0.12"]
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

Some methods on [`AnimationPlayer`](https://docs.rs/bevy/0.12.0/bevy/animation/struct.AnimationPlayer.html) have changed.

- `elapsed` was removed. Use `seek_time`.
- `set_elapsed` was removed. Use `seek_to`.
- `stop_repeating` was removed. Use `set_repeat(RepeatAnimation::Never)`.

If you were manually resetting animation state, you can use the new `replay` method instead.

### [Fix run-once runners](https://github.com/bevyengine/bevy/pull/10195)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">App</div>
</div>

`app.ready()` has been replaced by `app.plugins_state()` which will return more details on the current state of plugins in the app

### [Add support for KHR_materials_emissive_strength](https://github.com/bevyengine/bevy/pull/9553)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

The GLTF asset loader will now factor in `emissiveStrength` when converting to Bevy’s `StandardMaterial::emissive`. Blender will export emissive materials using this field. Remove the field from your GLTF files or manually modify your materials post-asset-load to match how Bevy would load these files in previous versions.

### [Bevy Asset V2](https://github.com/bevyengine/bevy/pull/8624)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

#### Migrating a custom asset loader

Existing asset loaders will need a few small changes to get them to work with Bevy Assets V2.

First, you’ll need to add the asset type as an associated type of the loader. This type is called `Asset` and represents the type of the “default asset” produced by the loader.

You’ll also need to add a `Settings` type which represents options that can be passed to the loader when you request an asset. If your asset has no settings, then you can just set it to the unit type.

```rust
pub struct MyAssetLoader;

impl AssetLoader for MyAssetLoader {
    type Asset = MyAsset;
    type Settings = ();
```

You’ll need to make a couple small changes to the `load` function as well. The load function now takes a `settings` parameter whose type is, you guessed it, `Settings`:

```rust
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
```

Again, if you are not using settings, then you can just ignore the parameter (prefix it with “_”).

Also, the second argument is now a reader rather than vector of bytes. If your existing code expects bytes, you can simply read the entire stream:

```rust
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
```

Finally, you’ll need to write the code which returns the default asset. This used to be done via a call to `load_context.set_default_asset()`, however in V2 you simply return the asset from the `load` function:

```rust
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
        let mut asset: MyAsset =
            serde_json::from_slice(&bytes).expect("unable to decode asset");
        Ok(asset)
    }
```

To use the new loader, make sure you register both the loader and the asset type:

```rust
app.register_asset_loader(MyAssetLoader)
    .init_asset::<MyAsset>()
```

#### Asset hot-reloading

The feature `filesystem_watcher` has been renamed to `file_watcher`.
In addition, you no longer need to manually configure the `ChangeWatcher` in the `AssetPlugin` as it is now configured automatically when the feature is enabled.

#### Labeled assets

If your loader allows labeled assets, there are a couple of different ways to handle them. The simplest is to call `load_context.labeled_asset_scope`:

```rust
// Assume `asset.children` is a HashMap or something.
// Using `drain` here so that we take ownership and don't end up with
// multiple references to the same asset.
asset.children.drain().for_each(|(label, mut item)| {
    load_context.labeled_asset_scope(label, |lc| {
        // Do any additional processing on the item
        // Use 'lc' to load dependencies
        item
    });
});
```

You can use the provided load context (`lc`) to load additional assets. These will automatically be registered as dependencies of the labeled asset.

#### Using assets

The actual call to `load` hasn’t changed:

```rust
let handle = server.load("path/to/my/asset.json");

// ...

let data = assets.get(&handle).unwrap();
```

#### Asset events

There are a few changes to asset events. The event no longer contains a `handle` field, instead the event contains a field called `id`:

```rust
for ev in ev_template.read() {
    match ev {
        AssetEvent::Added { id } => {
            println!("Asset added");
        }
        AssetEvent::LoadedWithDependencies { id } => {
            println!("Asset loaded");
        }
        AssetEvent::Modified { id } => {
            println!("Asset modified");
        }
        AssetEvent::Removed { id } => {
            println!("Asset removed");
        }
    }
}
```

The `id` can be used to get access to the asset data, the asset’s path or load status. Asset handles also contain an `id` field which can be used to compare for equality:

```rust
AssetEvent::Modified { id } => {
    for cmp in query.iter() {
       if cmp.handle.id() == id {
           println!("Found it!");
       }
    }
}
```

Also, as you may have noticed, the set of events has changed. The most important of these is `LoadedWithDependencies` which tells you that the asset and all its dependencies have finished loading into memory.

#### `UntypedHandle`

`HandleUntyped` has been renamed to `UntypedHandle`.
`HandleId` has been replaced with `UntypedAssetId` and its typed equivalent `AssetId<T>`.

The new way to construct an untyped handle looks like this:

```rust
// 0.11
const MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 0x1f40128bac02a9b);
// 0.12
const MESH_HANDLE: UntypedHandle =
    UntypedHandle::Weak(UntypedAssetId::Uuid { type_id: TypeId::of::<Mesh>(), uuid: Uuid::from_u128(0x1f40128bac02a9b) });
```

### [Copy on Write AssetPaths](https://github.com/bevyengine/bevy/pull/9729)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

```rust
// 0.11
AssetPath::new("logo.png", None);

// 0.12
AssetPath::from("logo.png");

// 0.11
AssetPath::new("scene.gltf", Some("Mesh0"));

// 0.12
AssetPath::from("scene.gltf").with_label("Mesh0");
```

`AssetPath` now serializes as `AssetPath("some_path.extension#Label")` instead of as `AssetPath { path: "some_path.extension", label: Some("Label) }`

### [Removed `anyhow`](https://github.com/bevyengine/bevy/pull/10003)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

- `anyhow` is no longer exported by `bevy_asset`; Add it to your own project (if required).
- `AssetLoader` and `AssetSaver` have an associated type `Error`; Define an appropriate error type (e.g., using `thiserror`), or use a pre-made error type (e.g., `anyhow::Error`). Note that using `anyhow::Error` is a drop-in replacement.
- `AssetLoaderError` has been removed; Define a new error type, or use an alternative (e.g., `anyhow::Error`)
- All the first-party `AssetLoader`s and `AssetSaver`s now return relevant (and narrow) error types instead of a single ambiguous type; Match over the specific error type, or encapsulate (`Box<dyn>`, `thiserror`, `anyhow`, etc.)

### [Non-blocking load_untyped using a wrapper asset](https://github.com/bevyengine/bevy/pull/10198)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

Whenever possible use the typed API in order to directly get a handle to your asset. If you do not know the type or need to use `load_untyped` for a different reason, Bevy 0.12 introduces an additional layer of indirection. The asset server will return a handle to a `LoadedUntypedAsset`, which will load in the background. Once it is loaded, the untyped handle to the asset file can be retrieved from the `LoadedUntypedAsset`s field `handle`.

### [reflect: `TypePath` part 2](https://github.com/bevyengine/bevy/pull/8768)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
    <div class="migration-guide-area-tag">Reflection</div>
</div>

- Rely on `TypePath` instead of `std::any::type_name` for all stability guarantees and for use in all reflection contexts, this is used through with one of the following APIs:
  - `TypePath::type_path` if you have a concrete type and not a value.
  - `DynamicTypePath::reflect_type_path` if you have an `dyn Reflect` value without a concrete type.
  - `TypeInfo::type_path` for use through the registry or if you want to work with the represented type of a `DynamicFoo`.
- Remove `type_name` from manual `Reflect` implementations.
- Use `type_path` and `type_path_table` in place of `type_name` on `TypeInfo`-like structs.
- Use `get_with_type_path(_mut)` over `get_with_type_name(_mut)`.

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
// 0.11
query.par_iter_mut().for_each_mut(|x| ...);

// 0.12
query.par_iter_mut().for_each(|x| ...);
```

The method `QueryParIter::for_each` now takes ownership of the `QueryParIter`, rather than taking a shared reference.

```rust
// 0.11
let par_iter = my_query.par_iter().batching_strategy(my_batching_strategy);
par_iter.for_each(|x| {
    // ...Do stuff with x...
    par_iter.for_each(|y| {
        // ...Do nested stuff with y...
    });
});

// 0.12
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

**`fetch` invariants**

The function `WorldQuery::fetch` has had the following safety invariant added:

> If `update_component_access` includes any mutable accesses, then the caller must ensure that `fetch` is called no more than once for each `entity`/`table_row` in each archetype.
> </br>
> If `Self` implements `ReadOnlyWorldQuery`, then this can safely be called multiple times.

This invariant was always required for soundness, but was previously undocumented. If you called this function manually anywhere, you should check to make sure that this invariant is not violated.

**Removed `clone_fetch`**

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

The `multi-threaded` feature in `bevy_ecs` and `bevy_tasks` is no longer enabled by default. However, this remains a default feature for the umbrella `bevy` crate.

if you are using `bevy` without `default-features`, or if you depend on `bevy_ecs` or `bevy_tasks` directly, you most likely want to enable this to allow systems to run in parallel.

### [Refactor build_schedule and related errors](https://github.com/bevyengine/bevy/pull/9579)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`ScheduleBuildError` now has strings in more of its variants. You may need to adjust code that is handling these variants.

### [Add `system.map(...)` for transforming the output of a system](https://github.com/bevyengine/bevy/pull/8526)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The `system_adapter` functions have been deprecated: use `.map` instead, which is a lightweight alternative to `.pipe`.

```rust
// 0.11
my_system.pipe(system_adapter::ignore)
my_system.pipe(system_adapter::unwrap)
my_system.pipe(system_adapter::new(T::from))

// 0.12
my_system.map(std::mem::drop)
my_system.map(Result::unwrap)
my_system.map(T::from)

// 0.11
my_system.pipe(system_adapter::info)
my_system.pipe(system_adapter::dbg)
my_system.pipe(system_adapter::warn)
my_system.pipe(system_adapter::error)

// 0.12
my_system.map(bevy_utils::info)
my_system.map(bevy_utils::dbg)
my_system.map(bevy_utils::warn)
my_system.map(bevy_utils::error)
```

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

The type `ManualEventIterator` has been renamed to `EventIterator`. Additionally, `ManualEventIteratorWithId` has been renamed to `EventIteratorWithId`.

### [Replaced `EntityCommand` Implementation for `FnOnce`](https://github.com/bevyengine/bevy/pull/9604)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

**1. New-Type `FnOnce`**

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

**2. Extract `(Entity, &mut World)` from `EntityMut`**

The method `into_world_mut` can be used to gain access to the `World` from an `EntityMut`.

```rust
let old = |id: Entity, world: &mut World| {
    /* ... */
};

let new = |mut entity: EntityWorldMut| {
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

`Schedules::insert`

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

`use bevy_ecs::system::QueryComponentError;` -> `use bevy_ecs::query::QueryComponentError;`

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

```rust
fn react_on_removal(mut removed: RemovedComponents<MyComponent>) {
    // 0.11
    for entity in removed.iter() { /* ... */ }
    for (entity, id) in removed.iter_with_id() { /* ... */ }
    for entity in &mut removed { /* ... */ }

    // 0.12
    for entity in removed.read() { /* ... */ }
    for (entity, id) in removed.read_with_id() { /* ... */ }
    for entity in removed.read() { /* ... */ }
}
```

### [Remove States::variants and remove enum-only restriction its derive](https://github.com/bevyengine/bevy/pull/9945)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`States::variants` no longer exists. If you relied on this function, consider using a library that provides enum iterators.

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

### [Add configure_schedules to App and Schedules to apply `ScheduleBuildSettings` to all schedules](https://github.com/bevyengine/bevy/pull/9514)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">App</div>
</div>

- No breaking changes.
- Adds `Schedule::get_build_settings()` getter for the schedule’s `ScheduleBuildSettings`.
- Can replaced manual configuration of all schedules:

```rust
// 0.11
for (_, schedule) in app.world.resource_mut::<Schedules>().iter_mut() {
    schedule.set_build_settings(build_settings);
}

// 0.l2
app.configure_schedules(build_settings);
```

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

Removed the method `EntityRef::world`, to fix a soundness issue with queries. If you need access to `&World` while using an `EntityRef`, consider passing the world as a separate parameter.

`EntityMut` can no longer perform ‘structural’ world mutations, such as adding or removing components, or despawning the entity. Additionally, `EntityMut::world`, `EntityMut::world_mut`, `EntityMut::into_world_mut`, and `EntityMut::world_scope` have been removed.
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

The default live zone bounds have been changed from `-0.95..=0.95` to `-1.0..=1.0` to align with more common usage. If you were relying on the old default, you can change change this by modifying [`GamepadSettings::default_axis_settings`](https://docs.rs/bevy/0.12.0/bevy/input/gamepad/struct.GamepadSettings.html#structfield.default_axis_settings).

### [Rename bevy_math::rects conversion methods](https://github.com/bevyengine/bevy/pull/9159)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Math</div>
</div>

Replace `Rect::as_urect` with `Rect::as_irect`, `Rect::as_rect` with `Rect::as_urect`, and `URect::as_urect` with `URect::as_irect`.

### [Rename `Bezier` to `CubicBezier` for clarity](https://github.com/bevyengine/bevy/pull/9554)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Math</div>
</div>

Change all `Bezier` references to `CubicBezier`

### [Add `Cubic` prefix to all cubic curve generators](https://github.com/bevyengine/bevy/pull/10299)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Math</div>
</div>

- Rename: `BSpline` -> `CubicBSpline`
- Rename: `CardinalSpline` -> `CubicCardinalSpline`
- Rename: `Hermite` -> `CubicHermite`

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

### [Use GpuArrayBuffer for MeshUniform](https://github.com/bevyengine/bevy/pull/9254)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Accessing the `model` member of an individual mesh object’s shader `Mesh` struct the old way where each `MeshUniform` was stored at its own dynamic offset:

```rust
struct Vertex {
    @location(0) position: vec3<f32>,
};

fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        mesh.model,
        vec4<f32>(vertex.position, 1.0)
    );
    return out;
}
```

The new way where one needs to index into the array of `Mesh`es for the batch:

```rust
struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        mesh[vertex.instance_index].model,
        vec4<f32>(vertex.position, 1.0)
    );
    return out;
}
```

Note that using the instance_index is the default way to pass the per-object index into the shader, but if you wish to do custom rendering approaches you can pass it in however you like.

### [Reduce the size of MeshUniform to improve performance](https://github.com/bevyengine/bevy/pull/9416)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`Sphere::intersects_obb` and `Frustum::intersects_obb` now take an `Affine3A` instead of a `Mat4`. You can use `Affine3A::from_mat4` or `Transform::compute_affine` to get an `Affine3A`.

`MeshUniform` now stores its current and previous model transforms as 4x3 matrices. Helper functions were added to bevy_pbr::mesh_functions to unpack the data.

```rust
// 0.11
var model = mesh[instance_index].model;

// 0.12
#import bevy_pbr::mesh_functions::affine_to_square

var model = affine_to_square(mesh[instance_index].model);
```

### [Reorder render sets, refactor bevy_sprite to take advantage](https://github.com/bevyengine/bevy/pull/9236)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Assets such as materials and meshes should now be created in `PrepareAssets` e.g. `prepare_assets<Mesh>`
- Queueing entities to `RenderPhase`s continues to be done in `Queue` e.g. `queue_sprites`
- Preparing resources (textures, buffers, etc.) should now be done in `PrepareResources`, e.g. `prepare_prepass_textures`, `prepare_mesh_uniforms`
- Prepare bind groups should now be done in `PrepareBindGroups` e.g. `prepare_mesh_bind_group`
- Any batching or instancing can now be done in `Prepare` where the order of the phase items is known e.g. `prepare_sprites`

### [Split `ComputedVisibility` into two components to allow for accurate change detection and speed up visibility propagation](https://github.com/bevyengine/bevy/pull/9497)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The `ComputedVisibility` component has been split into `InheritedVisibility` and
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
fn my_system(q: Query<&ComputedVisibility>) {
    for vis in &q {
        if vis.is_visible_in_hierarchy() {

// 0.12:
fn my_system(q: Query<&InheritedVisibility>) {
    for inherited_visibility in &q {
        if inherited_visibility.get() {
```

```rust
// 0.11:
fn my_system(q: Query<&ComputedVisibility>) {
    for vis in &q {
        if vis.is_visible_in_view() {

// 0.12:
fn my_system(q: Query<&ViewVisibility>) {
    for view_visibility in &q {
        if view_visibility.get() {
```

```rust
// 0.11:
fn my_system(mut q: Query<&mut ComputedVisibility>) {
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

The `check_visibility` system’s `Option<&NoFrustumCulling>` parameter has been replaced by  `Has<NoFrustumCulling>`, if you were calling it manually, you should change the type to match it

### [Allow other plugins to create renderer resources](https://github.com/bevyengine/bevy/pull/9925)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The `RenderPlugin` now takes a `RenderCreation` enum instead of `WgpuSettings`. `RenderSettings::default()` returns `RenderSettings::Automatic(WgpuSettings::default())`. `RenderSettings` also implements `From<WgpuSettings>`.

```rust
// 0.11
RenderPlugin {
    wgpu_settings: WgpuSettings {
    ...
    },
}

// 0.12
RenderPlugin {
    render_creation: RenderCreation::Automatic(WgpuSettings {
    ...
    }),
}
// or
RenderPlugin {
    render_creation: WgpuSettings {
    ...
    }.into(),
}
```

### [Use EntityHashMap<Entity, T> for render world entity storage for better performance](https://github.com/bevyengine/bevy/pull/9903)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Previously the render app extracted mesh entities and their component data from the main world and stored them as entities and components in the render world. Now they are extracted into essentially `EntityHashMap<Entity, T>` where `T` are structs containing an appropriate group of data. This means that while extract set systems will continue to run extract queries against the main world they will store their data in hash maps. Also, systems in later sets will either need to look up entities in the available resources such as `RenderMeshInstances`, or maintain their own `EntityHashMap<Entity, T>` for their own data.

```rust
// 0.11
fn queue_custom(
    material_meshes: Query<(Entity, &MeshTransforms, &Handle<Mesh>), With<InstanceMaterialData>>,
) {
    ...
    for (entity, mesh_transforms, mesh_handle) in &material_meshes {
        ...
    }
}

// 0.12
fn queue_custom(
    render_mesh_instances: Res<RenderMeshInstances>,
    instance_entities: Query<Entity, With<InstanceMaterialData>>,
) {
    ...
    for entity in &instance_entities {
        let Some(mesh_instance) = render_mesh_instances.get(&entity) else { continue; };
        // The mesh handle in `AssetId<Mesh>` form, and the `MeshTransforms` can now
        // be found in `mesh_instance` which is a `RenderMeshInstance`
        ...
    }
}
```

### [PCF For DirectionalLight/SpotLight Shadows](https://github.com/bevyengine/bevy/pull/8006)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Shadows cast by directional lights or spotlights now have smoother edges. To revert to the old behavior, add `ShadowFilteringMethod::Hardware2x2` to your cameras.

### [use `Material` for wireframes](https://github.com/bevyengine/bevy/pull/5314)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`WireframePipeline` was removed. If you were using it directly, please create an issue explaining your use case.

### [Deferred Renderer](https://github.com/bevyengine/bevy/pull/9258)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

<!-- TODO -->

### [pbr shader cleanup](https://github.com/bevyengine/bevy/pull/10105)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

in custom material shaders:

- `pbr_functions::pbr` no longer calls to `pbr_functions::alpha_discard`. if you were using the `pbr` function in a custom shader with alpha mask mode you now also need to call alpha_discard manually
- rename imports of `bevy_pbr::mesh_vertex_output` to `bevy_pbr::forward_io`
- rename instances of `MeshVertexOutput` to `VertexOutput`

in custom material prepass shaders:

- rename instances of `VertexOutput::clip_position` to `VertexOutput::position`

### [`*_PREPASS` Shader Def Cleanup](https://github.com/bevyengine/bevy/pull/10136)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

When using functions from `bevy_pbr::prepass_utils` (`prepass_depth()`, `prepass_normal()`, `prepass_motion_vector()`) in contexts where these prepasses might be disabled, you should now wrap your calls with the appropriate `#ifdef` guards, (`#ifdef DEPTH_PREPASS`, `#ifdef NORMAL_PREPASS`, `#ifdef MOTION_VECTOR_PREPASS`) providing fallback logic where applicable.

### [Allow extensions to StandardMaterial](https://github.com/bevyengine/bevy/pull/7820)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Manual implementations of `AsBindGroup` will need to be adjusted, the changes are pretty straightforward and can be seen in the diff for e.g. the `texture_binding_array` example.

### [Variable `MeshPipeline` View Bind Group Layout](https://github.com/bevyengine/bevy/pull/10156)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`MeshPipeline::view_layout` and `MeshPipeline::view_layout_multisampled` have been replaced with a private array to accommodate for variable view bind group layouts. To obtain a view bind group layout for the current pipeline state, use the new `MeshPipeline::get_view_layout()` or `MeshPipeline::get_view_layout_from_key()` methods.

### [Update shader imports](https://github.com/bevyengine/bevy/pull/10180)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

naga_oil 0.10 reworks the import mechanism to support more syntax to make it more rusty, and test for item use before importing to determine which imports are modules and which are items, which allows:

- use rust-style imports

```rust
#import bevy_pbr::{
    pbr_functions::{alpha_discard as discard, apply_pbr_lighting},
    mesh_bindings,
}
```

- import partial paths:

```rust
#import part::of::path
// ...
path::remainder::function();
```

which will call to `part::of::path::remainder::function`

- use fully qualified paths without importing:

```rust
// #import bevy_pbr::pbr_functions
bevy_pbr::pbr_functions::pbr()
```

- use imported items without qualifying

```rust
#import bevy_pbr::pbr_functions::pbr
// for backwards compatibility the old style is still supported:
// #import bevy_pbr::pbr_functions pbr
// ...
pbr()
```

- allows most imported items to end with `_` and numbers (naga_oil#30). still doesn’t allow struct members to end with `_` or numbers but it’s progress.
- the vast majority of existing shader code will work without changes, but will emit “deprecated” warnings for old-style imports. these can be suppressed with the `allow-deprecated` feature.
- partly breaks overrides (as far as i’m aware nobody uses these yet) - now overrides will only be applied if the overriding module is added as an additional import in the arguments to `Composer::make_naga_module` or `Composer::add_composable_module`. this is necessary to support determining whether imports are modules or items.

### [Bind group entries](https://github.com/bevyengine/bevy/pull/9694)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Calls to `RenderDevice::create_bind_group({BindGroupDescriptor { label, layout, entries })` must be amended to `RenderDevice::create_bind_group(label, layout, entries)`.
- If `label`s have been specified as `"bind_group_name".into()`, they need to change to just `"bind_group_name"`. `Some("bind_group_name")` and `None` will still work, but `Some("bind_group_name")` can optionally be simplified to just `"bind_group_name"`.

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

### [Use “specular occlusion” term to consistently extinguish fresnel on Ambient and Environment Map lights](https://github.com/bevyengine/bevy/pull/10182)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- If Fresnel highlights from Ambient and Environment Map lights are no longer visible in your materials, make sure you’re using a higher, physically plausible value of `reflectance` (⪆ 0.35).

### [Fix fog color being inaccurate](https://github.com/bevyengine/bevy/pull/10226)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Colors in `FogSettings` struct (`color` and `directional_light_color`) are now sent to the GPU in linear space. If you were using `Color::rgb()`/`Color::rgba()` and would like to retain the previous colors, you can quickly fix it by switching to `Color::rgb_linear()`/`Color::rgba_linear()`.

### [Image Sampler Improvements](https://github.com/bevyengine/bevy/pull/10254)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- When using the `Image` API, use `ImageSamplerDescriptor` instead of `wgpu::SamplerDescriptor`
- If writing custom wgpu renderer features that work with `Image`, call `&image_sampler.as_wgpu()` to convert to a wgpu descriptor.

### [`StandardMaterial` Light Transmission](https://github.com/bevyengine/bevy/pull/8015)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- `SsaoPipelineKey::temporal_noise` has been renamed to `SsaoPipelineKey::temporal_jitter`
- The `TAA` shader def (controlled by the presence of the `TemporalAntiAliasSettings` component in the camera) has been replaced with the `TEMPORAL_JITTER` shader def (controlled by the presence of the `TemporalJitter` component in the camera)
- `MeshPipelineKey::TAA` has been replaced by `MeshPipelineKey::TEMPORAL_JITTER`
- The `TEMPORAL_NOISE` shader def has been consolidated with `TEMPORAL_JITTER`

### [Increase default normal bias to avoid common artifacts](https://github.com/bevyengine/bevy/pull/10346)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The default `shadow_normal_bias` value for `DirectionalLight` and `SpotLight` has changed to accommodate artifacts introduced with the new shadow PCF changes. It is unlikely (especially given the new PCF shadow behaviors with these values), but you might need to manually tweak this value if your scene requires a lower bias and it relied on the previous default value.

### [Make `DirectionalLight` `Cascades` computation generic over `CameraProjection`](https://github.com/bevyengine/bevy/pull/9226)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

If you have a component `MyCustomProjection` that implements `CameraProjection`:

- You need to implement a new required associated method, `get_frustum_corners`, returning an array of the corners of a subset of the frustum with given `z_near` and `z_far`, in local camera space.
- You can now add the `build_directional_light_cascades::<MyCustomProjection>` system in `SimulationLightSystems::UpdateDirectionalLightCascades` after `clear_directional_light_cascades` for your projection to work with directional lights.

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

`scene_spawner_system` was moved to a new `SpawnScene` schedule which is run between `Update` and `PostUpdate`.

If you were ordering your own systems to run before `scene_spawner_system` in `Update`, that might no longer be necessary. If your system needs to run after `scene_spawner_system`, it should be moved to the `SpawnScene` or `PostUpdate` schedule.

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

Uses of `ComputeTaskPool::init`, `AsyncComputeTaskPool::init` and `IoTaskPool::init` should be changed to `::get_or_init`.

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

### [Make `GridPlacement`'s fields non-zero and add accessor functions.](https://github.com/bevyengine/bevy/pull/9486)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

`GridPlacement`’s constructor functions no longer accept values of `0`. Given any argument of `0` they will panic with a `GridPlacementError`.

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

### [Various accessibility API updates.](https://github.com/bevyengine/bevy/pull/9989)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

Change direct accesses of `AccessibilityRequested` to use `AccessibilityRequested.::get()`/`AccessibilityRequested::set()`

```rust
// 0.11
use std::sync::atomic::Ordering;

// To access
accessibility_requested.load(Ordering::SeqCst)
// To update
accessibility_requested.store(true, Ordering::SeqCst);

// 0.12
// To access
accessibility_requested.get()
// To update
accessibility_requested.set(true);
```

### [Add some more docs for bevy_text.](https://github.com/bevyengine/bevy/pull/9873)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

Usages of `TextSettings.max_font_atlases` from `bevy_text` must be changed to `TextSettings.soft_max_font_atlases`.

### [Update UI alignment docs](https://github.com/bevyengine/bevy/pull/10303)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The `JustifyContents` enum has been expanded to include `JustifyContents::Stretch`.

### [Add option to toggle window control buttons](https://github.com/bevyengine/bevy/pull/9083)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Windowing</div>
</div>

Added an `enabled_buttons` member to the `Window` struct through which users can enable or disable specific window control buttons.

### [Improve `bevy_winit` documentation](https://github.com/bevyengine/bevy/pull/7609)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Windowing</div>
</div>

- `UpdateMode::Reactive { max_wait: .. }` -> `UpdateMode::Reactive { wait: .. }`
- `UpdateMode::ReactiveLowPower { max_wait: .. }` -> `UpdateMode::ReactiveLowPower { wait: .. }`

### [Work around naga/wgpu WGSL instance_index -> GLSL gl_InstanceID bug on WebGL2](https://github.com/bevyengine/bevy/pull/9383)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Shader code before:

```rust
struct Vertex {
    @builtin(instance_index) instance_index: u32,
...
}

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    // ...
    var model = mesh[vertex_no_morph.instance_index].model;
}
```

After:

```rust
#import bevy_render::instance_index

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    // ...
}

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    // ...
    let instance_index = bevy_render::instance_index::get_instance_index(vertex_no_morph.instance_index);
    var model = mesh[instance_index].model;
}
```

### [Remove `IntoIterator` impl for `&mut EventReader`](https://github.com/bevyengine/bevy/pull/9583)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">No area label</div>
</div>

`&mut EventReader` does not implement `IntoIterator` anymore. replace `for foo in &mut events` by `for foo in events.iter()`

### [Update default `ClearColor`` to better match Bevy's branding](https://github.com/bevyengine/bevy/pull/10339)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The default app background color has changed. To use the old default, add a `ClearColor` resource.

```rust
App::new()
    .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
    .add_plugins(DefaultPlugins)
```

</div>

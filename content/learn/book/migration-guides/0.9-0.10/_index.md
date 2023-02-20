+++
title = "0.9 to 0.10"
weight = 6
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.9 to 0.10"
+++

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.
<div class="migration-guide">

### [bevy_reflect: Pre-parsed paths](https://github.com/bevyengine/bevy/pull/7321)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Animation</div>
    <div class="migration-guide-area-tag">Reflection</div>
</div>

`GetPath` methods have been renamed according to the following:

- `path` -> `reflect_path`
- `path_mut` -> `reflect_path_mut`
- `get_path` -> `path`
- `get_path_mut` -> `path_mut`

### [Remove App::add_sub_app](https://github.com/bevyengine/bevy/pull/7290)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">App</div>
</div>

`App::add_sub_app` has been removed in favor of `App::insert_sub_app`. Use `SubApp::new` and insert it via `App::add_sub_app`

Old:

```rust
let mut sub_app = App::new()
// Build subapp here
app.add_sub_app(MySubAppLabel, sub_app);
```

New:

```rust
let mut sub_app = App::new()
// Build subapp here
app.insert_sub_app(MySubAppLabel, SubApp::new(sub_app, extract_fn));
```

### [asset: make HandleUntyped::id private](https://github.com/bevyengine/bevy/pull/7076)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Assets</div>
</div>

- Instead of directly accessing the ID of a `HandleUntyped` as `handle.id`, use the new getter `handle.id()`.

### [Break `CorePlugin` into `TaskPoolPlugin`, `TypeRegistrationPlugin`, `FrameCountPlugin`.](https://github.com/bevyengine/bevy/pull/7083)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Core</div>
</div>

- `CorePlugin` broken into separate plugins.  If not using `DefaultPlugins` or `MinimalPlugins` `PluginGroup`s, the replacement for `CorePlugin` is now to add `TaskPoolPlugin`, `TypeRegistrationPlugin`, and `FrameCountPlugin` to the app.

### [Remove redundant table and sparse set component IDs from Archetype](https://github.com/bevyengine/bevy/pull/4927)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Do I still need to do this? I really hope people were not relying on the public facing APIs changed here.

### [Immutable sparse sets for metadata storage](https://github.com/bevyengine/bevy/pull/4928)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`Table::component_capacity()` has been removed as Tables do not support adding/removing columns after construction.

### [Split Component Ticks](https://github.com/bevyengine/bevy/pull/6547)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Various low level APIs interacting with the change detection ticks no longer return `&UnsafeCell<ComponentTicks>`, instead returning `TickCells` which contains two separate `&UnsafeCell<Tick>`s instead.

```rust
// 0.9
column.get_ticks(row).deref().changed

// 0.10
column.get_ticks(row).changed.deref()
```

### [Document and lock down types in bevy_ecs::archetype](https://github.com/bevyengine/bevy/pull/6742)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`ArchetypeId`, `ArchetypeGeneration`, and `ArchetypeComponentId` are all now opaque IDs and cannot be turned into a numeric value. Please file an issue if this does not work for your use case.

`Archetype` and `Archetypes` are not constructible outside of `bevy_ecs` now. Use `World::archetypes` to get a read-only reference to either of these types.

### [Lock down access to Entities](https://github.com/bevyengine/bevy/pull/6740)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`Entities`’s `Default` implementation has been removed. You can fetch a reference to a `World`’s `Entities` via `World::entities` and `World::entities_mut`.

`Entities::alloc_at_without_replacement` and `AllocAtWithoutReplacement` has been made private due to difficulty in using it properly outside of `bevy_ecs`. If you still need use of this API, please file an issue.

### [Make the `SystemParam` derive macro more flexible](https://github.com/bevyengine/bevy/pull/6694)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The lifetime `'s` has been removed from `EventWriter`. Any code that explicitly specified the lifetimes for this type will need to be updated.

```rust
// Before
#[derive(SystemParam)]
struct MessageWriter<'w, 's> {
    events: EventWriter<'w, 's, Message>,
}

// After
#[derive(SystemParam)]
struct MessageWriter<'w> {
    events: EventWriter<'w, Message>,
}
```

### [Borrow instead of consuming in `EventReader::clear`](https://github.com/bevyengine/bevy/pull/6851)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`EventReader::clear` now takes a mutable reference instead of consuming the event reader. This means that `clear` now needs explicit mutable access to the reader variable, which previously could have been omitted in some cases:

```rust
// Old (0.9)
fn clear_events(reader: EventReader<SomeEvent>) {
  reader.clear();
}

// New (0.10)
fn clear_events(mut reader: EventReader<SomeEvent>) {
  reader.clear();
}
```

### [Newtype ArchetypeRow and TableRow](https://github.com/bevyengine/bevy/pull/4878)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`Archetype` indices and `Table` rows have been newtyped as `ArchetypeRow` and `TableRow`.

### [Simplify trait hierarchy for `SystemParam`](https://github.com/bevyengine/bevy/pull/6865)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

_Merged with the guide for #6919._

### [Round out the untyped api s](https://github.com/bevyengine/bevy/pull/7009)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- `MutUntyped::into_inner` now marks things as changed.

### [Extend EntityLocation with TableId and TableRow](https://github.com/bevyengine/bevy/pull/6681)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

A `World` can only hold a maximum of 2<sup>32</sup> - 1 archetypes and tables now. If your use case requires more than this, please file an issue explaining your use case.

### [Remove the `SystemParamState` trait and remove types like `ResState`](https://github.com/bevyengine/bevy/pull/6919)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Note: this replaces the migration guide for #6865.
This is relative to Bevy 0.9, not main.

The traits `SystemParamState` and `SystemParamFetch` have been removed, and their functionality has been transferred to `SystemParam`.

```rust
// Before (0.9)
impl SystemParam for MyParam<'_, '_> {
    type State = MyParamState;
}
unsafe impl SystemParamState for MyParamState {
    fn init(world: &mut World, system_meta: &mut SystemMeta) -> Self { ... }
}
unsafe impl<'w, 's> SystemParamFetch<'w, 's> for MyParamState {
    type Item = MyParam<'w, 's>;
    fn get_param(&mut self, ...) -> Self::Item;
}
unsafe impl ReadOnlySystemParamFetch for MyParamState { }

// After (0.10)
unsafe impl SystemParam for MyParam<'_, '_> {
    type State = MyParamState;
    type Item<'w, 's> = MyParam<'w, 's>;
    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State { ... }
    fn get_param<'w, 's>(state: &mut Self::State, ...) -> Self::Item<'w, 's>;
}
unsafe impl ReadOnlySystemParam for MyParam<'_, '_> { }
```

The trait `ReadOnlySystemParamFetch` has been replaced with `ReadOnlySystemParam`.

```rust
// Before
unsafe impl ReadOnlySystemParamFetch for MyParamState {}

// After
unsafe impl ReadOnlySystemParam for MyParam<'_, '_> {}
```

### [Panic on dropping NonSend in non-origin thread.](https://github.com/bevyengine/bevy/pull/6534)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Normal resources and `NonSend` resources no longer share the same backing storage. If `R: Resource`, then `NonSend<R>` and `Res<R>` will return different instances from each other. If you are using both `Res<T>` and `NonSend<T>` (or their mutable variants), to fetch the same resources, it’s strongly advised to use `Res<T>`.

### [Document alignment requirements of `Ptr`, `PtrMut` and `OwningPtr`](https://github.com/bevyengine/bevy/pull/7151)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Safety invariants on `bevy_ptr` types’ `new` `byte_add` and `byte_offset` methods have been changed. All callers should re-audit for soundness.

### [Support piping exclusive systems](https://github.com/bevyengine/bevy/pull/7023)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

_Merged with the guide for #7675_.

### [Basic adaptive batching for parallel query iteration](https://github.com/bevyengine/bevy/pull/4777)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

The `batch_size` parameter for `Query(State)::par_for_each(_mut)` has been removed. These calls will automatically compute a batch size for you. Remove these parameters from all calls to these functions.

Before:

```rust
fn parallel_system(query: Query<&MyComponent>) {
   query.par_for_each(32, |comp| {
        ...
   });
}
```

After:

```rust
fn parallel_system(query: Query<&MyComponent>) {
   query.par_iter().for_each(|comp| {
        ...
   });
}
```

### [Added `resource_id` and changed `init_resource` and `init_non_send_resource` to return `ComponentId`](https://github.com/bevyengine/bevy/pull/7284)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Changed `World::init_resource` to return the generated `ComponentId`.
- Changed `World::init_non_send_resource` to return the generated `ComponentId`.

### [add `UnsafeWorldCell` abstraction](https://github.com/bevyengine/bevy/pull/6404)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

<!-- TODO  no migration required, will remove later -->

### [Remove `ExclusiveSystemParam::apply`](https://github.com/bevyengine/bevy/pull/7489)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

_Note for maintainers: this migration guide makes more sense if it’s placed above the one for #6919._

The trait method `ExclusiveSystemParamState::apply` has been removed. If you have an exclusive system with buffers that must be applied, you should apply them within the body of the exclusive system.

### [Replace `RemovedComponents<T>` backing with `Events<Entity>`](https://github.com/bevyengine/bevy/pull/5680)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Add a `mut` for `removed: RemovedComponents<T>` since we are now modifying an event reader internally.
- Iterating over removed components now requires `&mut removed_components` or `removed_components.iter()` instead of `&removed_components`.

### [Remove broken `DoubleEndedIterator` impls on event iterators](https://github.com/bevyengine/bevy/pull/7469)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`ManualEventIterator` and `ManualEventIteratorWithId` are no longer `DoubleEndedIterator`s.

### [Rename `Tick::is_older_than` to `Tick::is_newer_than`](https://github.com/bevyengine/bevy/pull/7561)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Replace usages of `Tick::is_older_than` with `Tick::is_newer_than`.

### [Rename `UnsafeWorldCellEntityRef` to `UnsafeEntityCell`](https://github.com/bevyengine/bevy/pull/7568)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

_Note for maintainers:_ This PR has no breaking changes relative to bevy 0.9. Instead of this PR having its own migration guide, we should just edit the changelog for #6404.

The type `UnsafeWorldCellEntityRef` has been renamed to `UnsafeEntityCell`.

### [Cleanup system sets called labels](https://github.com/bevyengine/bevy/pull/7678)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`PrepareAssetLabel` is now called `PrepareAssetSet`

### [Simplify generics for the `SystemParamFunction` trait](https://github.com/bevyengine/bevy/pull/7675)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

_The guide for #7023 has been merged into this one._

For the `SystemParamFunction` trait, the type parameters `In`, `Out`, and `Param` have been turned into associated types.

```rust
// Before
fn my_generic_system<T, In, Out, Param, Marker>(system_function: T)
where
    T: SystemParamFunction<In, Out, Param, Marker>,
    T: Param: SystemParam,
{ ... }

// After
fn my_generic_system<T, Marker>(system_function: T)
where
    T: SystemParamFunction<Marker>,
{ ... }
```

For the `ExclusiveSystemParamFunction` trait, the type parameter `Param` has been turned into an associated type.
Also, `In` and `Out` associated types have been added, since exclusive systems now support system piping.

```rust
// Before
fn my_exclusive_system<T, Param, Marker>(system_function: T)
where
    T: ExclusiveSystemParamFunction<Param, Marker>,
    T: Param: ExclusiveSystemParam,
{ ... }

// After
fn my_exclusive_system<T, Marker>(system_function: T)
where
    T: ExclusiveSystemParamFunction<Marker>,
{ ... }
```

### [Cleanup ScheduleBuildSettings](https://github.com/bevyengine/bevy/pull/7721)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

<!-- TODO -->

### [Deprecate `ChangeTrackers<T>` in favor of `Ref<T>`](https://github.com/bevyengine/bevy/pull/7306)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`ChangeTrackers<T>` has been deprecated, and will be removed in the next release. Any usage should be replaced with `Ref<T>`.

```rust
// Before (0.9)
fn my_system(q: Query<(&MyComponent, ChangeTrackers<MyComponent>)>) {
    for (value, trackers) in &q {
        if trackers.is_changed() {
            // Do something with `value`.
        }
    }
}

// After (0.10)
fn my_system(q: Query<Ref<MyComponent>>) {
    for value in &q {
        if value.is_changed() {
            // Do something with `value`.
        }
    }
}
```

### [Move system_commands spans into apply_buffers](https://github.com/bevyengine/bevy/pull/6900)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Diagnostics</div>
</div>

<!-- TODO no migration required, will remove later -->

### [bevy_ecs: ReflectComponentFns without World](https://github.com/bevyengine/bevy/pull/7206)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Reflection</div>
</div>

- Call `World::entity` before calling into the changed `ReflectComponent` methods, most likely user already has a `EntityRef` or `EntityMut` which was being queried redundantly.

### [Allow iterating over with EntityRef over the entire World](https://github.com/bevyengine/bevy/pull/6843)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Scenes</div>
</div>

`World::iter_entities` now returns an iterator of `EntityRef` instead of `Entity`. To get the actual ID, use `EntityRef::id` from the returned `EntityRef`s.

### [Remove `BuildWorldChildren` impl from `WorldChildBuilder`](https://github.com/bevyengine/bevy/pull/6727)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Hierarchy</div>
</div>

Hierarchy editing methods such as `with_children` and `push_children` have been removed from `WorldChildBuilder`.
You can edit the hierarchy via `EntityMut` instead.

### [Rename dynamic feature](https://github.com/bevyengine/bevy/pull/7340)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Meta</div>
</div>

- `dynamic` feature was renamed to `dynamic_linking`

### [reflect: add `insert` and `remove` methods to `List`](https://github.com/bevyengine/bevy/pull/7063)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

- Manual implementors of `List` need to implement the new methods `insert` and `remove` and  consider whether to use the new default implementation of `push` and `pop`.

### [bevy_reflect: Decouple `List` and `Array` traits](https://github.com/bevyengine/bevy/pull/7467)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

The `List` trait is no longer dependent on `Array`. Implementors of `List` can remove the `Array` impl and move its methods into the `List` impl (with only a couple tweaks).

```rust
// BEFORE
impl Array for Foo {
  fn get(&self, index: usize) -> Option<&dyn Reflect> {/* ... */}
  fn get_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {/* ... */}
  fn len(&self) -> usize {/* ... */}
  fn is_empty(&self) -> bool {/* ... */}
  fn iter(&self) -> ArrayIter {/* ... */}
  fn drain(self: Box<Self>) -> Vec<Box<dyn Reflect>> {/* ... */}
  fn clone_dynamic(&self) -> DynamicArray {/* ... */}
}

impl List for Foo {
  fn insert(&mut self, index: usize, element: Box<dyn Reflect>) {/* ... */}
  fn remove(&mut self, index: usize) -> Box<dyn Reflect> {/* ... */}
  fn push(&mut self, value: Box<dyn Reflect>) {/* ... */}
  fn pop(&mut self) -> Option<Box<dyn Reflect>> {/* ... */}
  fn clone_dynamic(&self) -> DynamicList {/* ... */}
}

// AFTER
impl List for Foo {
  fn get(&self, index: usize) -> Option<&dyn Reflect> {/* ... */}
  fn get_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {/* ... */}
  fn insert(&mut self, index: usize, element: Box<dyn Reflect>) {/* ... */}
  fn remove(&mut self, index: usize) -> Box<dyn Reflect> {/* ... */}
  fn push(&mut self, value: Box<dyn Reflect>) {/* ... */}
  fn pop(&mut self) -> Option<Box<dyn Reflect>> {/* ... */}
  fn len(&self) -> usize {/* ... */}
  fn is_empty(&self) -> bool {/* ... */}
  fn iter(&self) -> ListIter {/* ... */}
  fn drain(self: Box<Self>) -> Vec<Box<dyn Reflect>> {/* ... */}
  fn clone_dynamic(&self) -> DynamicList {/* ... */}
}
```

Some other small tweaks that will need to be made include:

- Use `ListIter` for `List::iter` instead of `ArrayIter` (the return type from `Array::iter`)
- Replace `array_hash` with `list_hash` in `Reflect::reflect_hash` for implementors of `List`

### [implement `TypeUuid` for primitives and fix multiple-parameter generics having the same `TypeUuid`](https://github.com/bevyengine/bevy/pull/6633)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
</div>

<!-- TODO -->

### [bevy_reflect: Remove `ReflectSerialize` and `ReflectDeserialize` registrations from most glam types](https://github.com/bevyengine/bevy/pull/6580)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Reflection</div>
    <div class="migration-guide-area-tag">Scenes</div>
</div>

This PR removes `ReflectSerialize` and `ReflectDeserialize` registrations from most glam types. This means any code relying on either of those type data existing for those glam types will need to not do that.

This also means that some serialized glam types will need to be updated. For example, here is `Affine3A`:

```rust
// BEFORE
(
  "glam::f32::affine3a::Affine3A": (1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),

// AFTER
  "glam::f32::affine3a::Affine3A": (
    matrix3: (
      x_axis: (
        x: 1.0,
        y: 0.0,
        z: 0.0,
      ),
      y_axis: (
        x: 0.0,
        y: 1.0,
        z: 0.0,
      ),
      z_axis: (
        x: 0.0,
        y: 0.0,
        z: 1.0,
      ),
    ),
    translation: (
      x: 0.0,
      y: 0.0,
      z: 0.0,
    ),
  )
)
```

### [Add AutoMax next to ScalingMode::AutoMin](https://github.com/bevyengine/bevy/pull/6496)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

just rename `ScalingMode::Auto` to `ScalingMode::AutoMin` if you are using it.

### [Change `From<Icosphere>` to `TryFrom<Icosphere>`](https://github.com/bevyengine/bevy/pull/6484)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

```rust
// 0.9
shape::Icosphere {
    radius: 0.5,
    subdivisions: 5,
}
.into()

// 0.10
shape::Icosphere {
    radius: 0.5,
    subdivisions: 5,
}
.try_into()
.unwrap()
```

### [Add try_* to add_slot_edge, add_node_edge](https://github.com/bevyengine/bevy/pull/6720)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Remove `.unwrap()` from `add_node_edge` and `add_slot_edge`.
For cases where the error was handled, use `try_add_node_edge` and `try_add_slot_edge` instead.

Remove `.unwrap()` from `input_node`.
For cases where the option was handled, use `get_input_node` instead.

### [Shader defs can now have a value](https://github.com/bevyengine/bevy/pull/5900)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- replace `shader_defs.push(String::from("NAME"));` by `shader_defs.push("NAME".into());`
- if you used shader def `NO_STORAGE_BUFFERS_SUPPORT`, check how `AVAILABLE_STORAGE_BUFFER_BINDINGS` is now used in Bevy default shaders

### [get pixel size from wgpu](https://github.com/bevyengine/bevy/pull/6820)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`PixelInfo` has been removed. `PixelInfo::components` is equivalent to `texture_format.describe().components`. `PixelInfo::type_size` can be gotten from `texture_format.describe().block_size/ texture_format.describe().components`. But note this can yield incorrect results for some texture types like Rg11b10Float.

### [run clear trackers on render world](https://github.com/bevyengine/bevy/pull/6878)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The call to `clear_trackers` in `App` has been moved from the schedule to App::update for the main world and calls to `clear_trackers` have been added for sub_apps in the same function. This was due to needing stronger guarantees. If clear_trackers isn’t called on a world it can lead to memory leaks in `RemovedComponents`. If you were ordering systems with clear_trackers this is no longer possible.

### [Rename camera "priority" to "order"](https://github.com/bevyengine/bevy/pull/6908)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

<!-- TODO -->

### [enum `Visibility` component](https://github.com/bevyengine/bevy/pull/6320)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- evaluation of the `visibility.is_visible` field should now check for `visibility == Visibility::Inherited`.
- setting the `visibility.is_visible` field should now directly set the value: `*visibility = Visibility::Inherited`.
- usage of `Visibility::VISIBLE` or `Visibility::INVISIBLE` should now use `Visibility::Inherited` or `Visibility::Hidden` respectively.
- `ComputedVisibility::INVISIBLE` and `SpatialBundle::VISIBLE_IDENTITY` have been renamed to `ComputedVisibility::HIDDEN` and `SpatialBundle::INHERITED_IDENTITY` respectively.

### [Reduce branching in TrackedRenderPass](https://github.com/bevyengine/bevy/pull/7053)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`TrackedRenderPass` now requires a `RenderDevice` to construct. To make this easier, use `RenderContext.begin_tracked_render_pass` instead.

```rust
// 0.9
TrackedRenderPass::new(render_context.command_encoder.begin_render_pass(
  &RenderPassDescriptor {
    ...
  },
));

// 0.10
render_context.begin_tracked_render_pass(RenderPassDescriptor {
  ...
});
```

### [Make PipelineCache internally mutable.](https://github.com/bevyengine/bevy/pull/7205)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Most usages of `resource_mut::<PipelineCache>` and `ResMut<PipelineCache>` can be changed to `resource::<PipelineCache>` and `Res<PipelineCache>` as long as they don’t use any methods requiring mutability - the only public method requiring it is `process_queue`.

### [Changed Msaa to Enum](https://github.com/bevyengine/bevy/pull/7292)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

```rust
let multi = Msaa { samples: 4 }
// is now
let multi = Msaa::Sample4

multi.samples
// is now
multi.samples()
```

### [Support recording multiple CommandBuffers in RenderContext](https://github.com/bevyengine/bevy/pull/7248)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`RenderContext`’s fields are now private. Use the accessors on `RenderContext` instead, and construct it with `RenderContext::new`.

### [Improve `OrthographicCamera` consistency and usability](https://github.com/bevyengine/bevy/pull/6201)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Change `window_origin` to `viewport_origin`; replace `WindowOrigin::Center` with `Vec2::new(0.5, 0.5)` and `WindowOrigin::BottomLeft` with `Vec2::new(0.0, 0.0)`
- For shadow projections and such, replace `left`, `right`, `bottom`, and `top` with `area: Rect::new(left, bottom, right, top)`
- For camera projections, remove l/r/b/t values from `OrthographicProjection` instantiations, as they no longer have any effect in any `ScalingMode`
- Change `ScalingMode::None` to `ScalingMode::Fixed`
  - Replace manual changes of l/r/b/t with:
    - Arguments in `ScalingMode::Fixed` to specify size
    - `viewport_origin` to specify offset

- Change `ScalingMode::WindowSize` to `ScalingMode::WindowSize(1.0)`

### [Changed &mut PipelineCache to &PipelineCache](https://github.com/bevyengine/bevy/pull/7598)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- `SpecializedComputePipelines::specialize` now takes a `&PipelineCache` instead of a `&mut PipelineCache`

### [Introduce detailed_trace macro, use in TrackedRenderPass](https://github.com/bevyengine/bevy/pull/7639)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Some detailed bevy trace events now require the use of the cargo feature `detailed_trace` in addition to enabling `TRACE` level logging to view. Should you wish to see these logs, please compile your code with the bevy feature `detailed_trace`. Currently, the only logs that are affected are the renderer logs pertaining to `TrackedRenderPass` functions

### [added subdivisions to shape::Plane](https://github.com/bevyengine/bevy/pull/7546)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

All the examples needed to be updated to initalize the subdivisions field.
Also there were two tests in tests/window that need to be updated.

A user would have to update all their uses of shape::Plane to initalize the subdivisions field.

### [Change standard material defaults and update docs](https://github.com/bevyengine/bevy/pull/7664)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`StandardMaterial`’s default have now changed to be a fully dielectric material with medium roughness. If you want to use the old defaults, you can set  `perceptual_roughness = 0.089` and `metallic = 0.01` (though metallic should generally only be set to 0.0 or 1.0).

### [Directly extract joints into SkinnedMeshJoints](https://github.com/bevyengine/bevy/pull/6833)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">Animation</div>
</div>

`ExtractedJoints` has been removed. Read the bound bones from `SkinnedMeshJoints` instead.

### [Intepret glTF colors as linear instead of sRGB](https://github.com/bevyengine/bevy/pull/6828)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">Assets</div>
</div>

No api changes are required, but it's possible that your gltf meshes look different

### [The `update_frame_count` system should be placed in CorePlugin](https://github.com/bevyengine/bevy/pull/6676)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">Core</div>
    <div class="migration-guide-area-tag">Time</div>
</div>

The `FrameCount`  resource was previously only updated when using the `bevy_render` feature. If you are not using this feature but still want the `FrameCount` it will now be updated correctly.

### [Migrate engine to Schedule v3](https://github.com/bevyengine/bevy/pull/7267)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Calls to `.label(MyLabel)` should be replaced with `.in_set(MySet)`
- Stages have been removed. Replace these with system sets, and then add command flushes using the `apply_system_buffers` exclusive system where needed.
- The `CoreStage`, `StartupStage,`RenderStage`and`AssetStage`enums have been replaced with`CoreSet`,`StartupSet, `RenderSet` and `AssetSet`. The same scheduling guarantees have been preserved.
  - Systems are no longer added to `CoreSet::Update` by default. Add systems manually if this behavior is needed, although you should consider adding your game logic systems to `CoreSchedule::FixedTimestep` instead for more reliable framerate-independent behavior.
  - Similarly, startup systems are no longer part of `StartupSet::Startup` by default. In most cases, this won’t matter to you.
  - For example, `add_system_to_stage(CoreStage::PostUpdate, my_system)` should be replaced with
  - `add_system(my_system.in_set(CoreSet::PostUpdate)`
- When testing systems or otherwise running them in a headless fashion, simply construct and run a schedule using `Schedule::new()` and `World::run_schedule` rather than constructing stages
- Run criteria have been renamed to run conditions. These can now be combined with each other and with states.
- Looping run criteria and state stacks have been removed. Use an exclusive system that runs a schedule if you need this level of control over system control flow.
- For app-level control flow over which schedules get run when (such as for rollback networking), create your own schedule and insert it under the `CoreSchedule::Outer` label.
- Fixed timesteps are now evaluated in a schedule, rather than controlled via run criteria. The `run_fixed_timestep` system runs this schedule between `CoreSet::First` and `CoreSet::PreUpdate` by default.
- Command flush points introduced by `AssetStage` have been removed. If you were relying on these, add them back manually.
- the `calculate_bounds` system, with the `CalculateBounds` label, is now in `CoreSet::Update`, rather than in `CoreSet::PostUpdate` before commands are applied. You may need to order your movement systems to occur before this system in order to avoid system order ambiguities in culling behavior.
- the `RenderLabel` `AppLabel` was renamed to `RenderApp` for clarity
- `App::add_state` now takes 0 arguments: the starting state is set based on the `Default` impl.
- Instead of creating `SystemSet` containers for systems that run in stages, simply use `.on_enter::<State::Variant>()` or its `on_exit` or `on_update` siblings.
- `SystemLabel` derives should be replaced with `SystemSet`. You will also need to add the `Debug`, `PartialEq`, `Eq`, and `Hash` traits to satisfy the new trait bounds.
- `with_run_criteria` has been renamed to `run_if`. Run criteria have been renamed to run conditions for clarity, and should now simply return a bool.
- States have been dramatically simplified: there is no longer a “state stack”. To queue a transition to the next state, call `NextState::set`
- Strings can no longer be used as a `SystemLabel` or `SystemSet`. Use a type, or use the system function instead.

### [Pipelined Rendering](https://github.com/bevyengine/bevy/pull/6503)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">Tasks</div>
</div>

__App `runner` and SubApp `extract` functions are now required to be Send__

This was changed to enable pipelined rendering. If this breaks your use case please report it as these new bounds might be able to be relaxed.

### [Remove ImageMode](https://github.com/bevyengine/bevy/pull/6674)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">UI</div>
</div>

`ImageNode` never worked, if you were using it please create an issue.

### [Rename the `background_color` of 'ExtractedUiNode` to `color`](https://github.com/bevyengine/bevy/pull/7452)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">UI</div>
</div>

- The `background_color` field of `ExtractedUiNode` is now named `color`.

### [Make spawn_dynamic return InstanceId](https://github.com/bevyengine/bevy/pull/6663)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Scenes</div>
</div>

<!-- TODO no migration required, will remove it later-->

### [Parallelized transform propagation](https://github.com/bevyengine/bevy/pull/4775)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Transform</div>
</div>

<!-- TODO no migration required, will remove it later-->

### [Remove the `GlobalTransform::translation_mut` method](https://github.com/bevyengine/bevy/pull/7134)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Transform</div>
    <div class="migration-guide-area-tag">Hierarchy</div>
</div>

`GlobalTransform::translation_mut` has been removed without alternative,
if you were relying on this, update the `Transform` instead. If the given entity
had children or parent, you may need to remove its parent to make its transform
independent (in which case the new `Commands::set_parent_in_place` and
`Commands::remove_parent_in_place` may be of interest)

Bevy may add in the future a way to toggle transform propagation on
an entity basis.

### [Flip UI image](https://github.com/bevyengine/bevy/pull/6292)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

<!-- TODO -->

### [Remove `TextError::ExceedMaxTextAtlases(usize)` variant](https://github.com/bevyengine/bevy/pull/6796)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

TextError::ExceedMaxTextAtlases(usize)` was never thrown so if you were matching on this variant you can simply remove it.

### [Change default FocusPolicy to Pass](https://github.com/bevyengine/bevy/pull/7161)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- `FocusPolicy` default has changed from `FocusPolicy::Block` to `FocusPolicy::Pass`

### [Remove VerticalAlign from TextAlignment](https://github.com/bevyengine/bevy/pull/6807)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The `alignment` field of `Text` now only affects the text’s internal alignment.

__Change `TextAlignment` to TextAlignment` which is now an enum. Replace:__

- `TextAlignment::TOP_LEFT`, `TextAlignment::CENTER_LEFT`, `TextAlignment::BOTTOM_LEFT` with `TextAlignment::Left`
- `TextAlignment::TOP_CENTER`, `TextAlignment::CENTER_LEFT`, `TextAlignment::BOTTOM_CENTER` with `TextAlignment::Center`
- `TextAlignment::TOP_RIGHT`, `TextAlignment::CENTER_RIGHT`, `TextAlignment::BOTTOM_RIGHT` with `TextAlignment::Right`

__Changes for `Text2dBundle`__

`Text2dBundle` has a new field ‘text_anchor’ that takes an `Anchor` component that controls its position relative to its transform.

### [Remove `QueuedText`](https://github.com/bevyengine/bevy/pull/7414)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

`QueuedText` was never meant to be user facing. If you relied on it, please make an issue.

### [change the default `width` and `height` of `Size` to `Val::Auto`](https://github.com/bevyengine/bevy/pull/7475)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The default values for `Size` `width` and `height` have been changed from `Val::Undefined` to `Val::Auto`.
It’s unlikely to cause any issues with existing code.

### [Fix the `Size` helper functions using the wrong default value and improve the UI examples](https://github.com/bevyengine/bevy/pull/7626)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The `Size::width` constructor function now sets the `height` to `Val::Auto` instead of `Val::Undefined`.
The `Size::height` constructor function now sets the `width` to `Val::Auto` instead of `Val::Undefined`.

### [The `size` field of `CalculatedSize` should not be a `Size`](https://github.com/bevyengine/bevy/pull/7641)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- The size field of `CalculatedSize` has been changed to a `Vec2`.

### [Windows as Entities](https://github.com/bevyengine/bevy/pull/5589)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Windowing</div>
</div>

- Replace `WindowDescriptor` with `Window`.
  - Change `width` and `height` fields in a `WindowResolution`, either by doing

```rust
WindowResolution::new(width, height) // Explicitly
// or using From<_> for tuples for convenience
(1920., 1080.).into()
```

- Replace any `WindowCommand` code to just modify the `Window`’s fields directly  and creating/closing windows is now by spawning/despawning an entity with a `Window` component like so:

```rust
let window = commands.spawn(Window { ... }).id(); // open window
commands.entity(window).despawn(); // close window
```

### [Allow not preventing default event behaviors on wasm](https://github.com/bevyengine/bevy/pull/7304)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Windowing</div>
</div>

<!-- TODO I'm not sure this needs a guide, I assume most people would be using the ..default() anyway and the ones that aren't doing that will just have a clear compile error -->

### [update winit to 0.28](https://github.com/bevyengine/bevy/pull/7480)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Windowing</div>
</div>

before:

```rust
    app.new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                always_on_top: true,
                ..default()
            }),
            ..default()
        }));
```

after:

```rust
    app.new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        }));
```

</div>

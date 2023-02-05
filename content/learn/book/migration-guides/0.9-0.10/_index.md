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

### [Replace `RemovedComponents<T>` backing with `Events<Entity>`](https://github.com/bevyengine/bevy/pull/5680)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Add a `mut` for `removed: RemovedComponents<T>` since we are now modifying an event reader internally.
- Iterating over removed components now requires `&mut removed_components` or `removed_components.iter()` instead of `&removed_components`.

### [Remove `ExclusiveSystemParam::apply`](https://github.com/bevyengine/bevy/pull/7489)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

_Note for maintainers: this migration guide makes more sense if it’s placed above the one for #6919._

The trait method `ExclusiveSystemParamState::apply` has been removed. If you have an exclusive system with buffers that must be applied, you should apply them within the body of the exclusive system.

### [add `UnsafeWorldCell` abstraction](https://github.com/bevyengine/bevy/pull/6404)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

<!-- TODO -->

### [Added `resource_id` and changed `init_resource` and `init_non_send_resource` to return `ComponentId`](https://github.com/bevyengine/bevy/pull/7284)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

<!-- TODO -->

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

### [Support piping exclusive systems](https://github.com/bevyengine/bevy/pull/7023)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Exclusive systems (systems that access `&mut World`) now support system piping, so the `ExclusiveSystemParamFunction` trait now has generics for the `In`put and `Out`put types.

```rust
// Before
fn my_generic_system<T, Param>(system_function: T)
where T: ExclusiveSystemParamFunction<Param>
{ ... }

// After
fn my_generic_system<T, In, Out, Param>(system_function: T)
where T: ExclusiveSystemParamFunction<In, Out, Param>
{ ... }
```

### [Document alignment requirements of `Ptr`, `PtrMut` and `OwningPtr`](https://github.com/bevyengine/bevy/pull/7151)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- Safety invariants on `bevy_ptr` types’ `new` `byte_add` and `byte_offset` methods have been changed. All callers should re-audit for soundness.

### [Panic on dropping NonSend in non-origin thread.](https://github.com/bevyengine/bevy/pull/6534)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Normal resources and `NonSend` resources no longer share the same backing storage. If `R: Resource`, then `NonSend<R>` and `Res<R>` will return different instances from each other. If you are using both `Res<T>` and `NonSend<T>` (or their mutable variants), to fetch the same resources, it’s strongly advised to use `Res<T>`.

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

### [Extend EntityLocation with TableId and TableRow](https://github.com/bevyengine/bevy/pull/6681)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

A `World` can only hold a maximum of 232 - 1 archetypes and tables now. If your use case requires more than this, please file an issue explaining your use case.

### [Round out the untyped api s](https://github.com/bevyengine/bevy/pull/7009)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

- `MutUntyped::into_inner` now marks things as changed.

### [Simplify trait hierarchy for `SystemParam`](https://github.com/bevyengine/bevy/pull/6865)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

_Merged with the guide for #6919._

### [Newtype ArchetypeRow and TableRow](https://github.com/bevyengine/bevy/pull/4878)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

<!-- TODO -->

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

### [Lock down access to Entities](https://github.com/bevyengine/bevy/pull/6740)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

<!-- TODO -->

### [Document and lock down types in bevy_ecs::archetype](https://github.com/bevyengine/bevy/pull/6742)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

<!-- TODO -->

### [Split Component Ticks](https://github.com/bevyengine/bevy/pull/6547)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

TODO

### [Immutable sparse sets for metadata storage](https://github.com/bevyengine/bevy/pull/4928)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

`Table::component_capacity()` has been removed as Tables do not support adding/removing columns after construction.

### [Remove redundant table and sparse set component IDs from Archetype](https://github.com/bevyengine/bevy/pull/4927)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
</div>

Do I still need to do this? I really hope people were not relying on the public facing APIs changed here.

### [Move system_commands spans into apply_buffers](https://github.com/bevyengine/bevy/pull/6900)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">ECS</div>
    <div class="migration-guide-area-tag">Diagnostics</div>
</div>

<!-- TODO -->

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

<!-- TODO -->

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

- Manual implementors of `List` need to implement the new methods `insert` and `remove` and
consider whether to use the new default implementation of `push` and `pop`.

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

### [Support recording multiple CommandBuffers in RenderContext](https://github.com/bevyengine/bevy/pull/7248)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`RenderContext`’s fields are now private. Use the accessors on `RenderContext` instead, and construct it with `RenderContext::new`.

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

### [Make PipelineCache internally mutable.](https://github.com/bevyengine/bevy/pull/7205)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- Most usages of `resource_mut::<PipelineCache>` and `ResMut<PipelineCache>` can be changed to `resource::<PipelineCache>` and `Res<PipelineCache>` as long as they don’t use any methods requiring mutability - the only public method requiring it is `process_queue`.

### [Reduce branching in TrackedRenderPass](https://github.com/bevyengine/bevy/pull/7053)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

TODO

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

### [run clear trackers on render world](https://github.com/bevyengine/bevy/pull/6878)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

The call to `clear_trackers` in `App` has been moved from the schedule to App::update for the main world and calls to `clear_trackers` have been added for sub_apps in the same function. This was due to needing stronger guarantees. If clear_trackers isn’t called on a world it can lead to memory leaks in `RemovedComponents`.

### [get pixel size from wgpu](https://github.com/bevyengine/bevy/pull/6820)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

`PixelInfo` has been removed. `PixelInfo::components` is equivalent to `texture_format.describe().components`. `PixelInfo::type_size` can be gotten from `texture_format.describe().block_size/ texture_format.describe().components`. But note this can yield incorrect results for some texture types like Rg11b10Float.

### [Shader defs can now have a value](https://github.com/bevyengine/bevy/pull/5900)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

- replace `shader_defs.push(String::from("NAME"));` by `shader_defs.push("NAME".into());`
- if you used shader def `NO_STORAGE_BUFFERS_SUPPORT`, check how `AVAILABLE_STORAGE_BUFFER_BINDINGS` is now used in Bevy default shaders

### [Add try_* to add_slot_edge, add_node_edge](https://github.com/bevyengine/bevy/pull/6720)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

Remove `.unwrap()` from `add_node_edge` and `add_slot_edge`.
For cases where the error was handled, use `try_add_node_edge` and `try_add_slot_edge` instead.

Remove `.unwrap()` from `input_node`.
For cases where the option was handled, use `get_input_node` instead.

### [Add AutoMax next to ScalingMode::AutoMin](https://github.com/bevyengine/bevy/pull/6496)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

just rename `ScalingMode::Auto` to `ScalingMode::AutoMin` if you are using it.

### [Change `From<Icosphere>` to `TryFrom<Icosphere>`](https://github.com/bevyengine/bevy/pull/6484)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
</div>

<!-- TODO -->

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

<!-- TODO -->

### [The `update_frame_count` system should be placed in CorePlugin](https://github.com/bevyengine/bevy/pull/6676)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">Core</div>
    <div class="migration-guide-area-tag">Time</div>
</div>

<!-- TODO -->

### [Pipelined Rendering](https://github.com/bevyengine/bevy/pull/6503)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">Tasks</div>
</div>

### [Rename the `background_color` of 'ExtractedUiNode` to `color`](https://github.com/bevyengine/bevy/pull/7452)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">UI</div>
</div>

- The `background_color` field of `ExtractedUiNode` is now named `color`.

### [Remove ImageMode](https://github.com/bevyengine/bevy/pull/6674)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Rendering</div>
    <div class="migration-guide-area-tag">UI</div>
</div>

<!-- TODO -->

### [Make spawn_dynamic return InstanceId](https://github.com/bevyengine/bevy/pull/6663)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Scenes</div>
</div>

<!-- TODO -->

### [Parallelized transform propagation](https://github.com/bevyengine/bevy/pull/4775)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Transform</div>
</div>

<!-- TODO -->

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

### [change the default `width` and `height` of `Size` to `Val::Auto`](https://github.com/bevyengine/bevy/pull/7475)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The default values for `Size` `width` and `height` have been changed from `Val::Undefined` to `Val::Auto`.
It’s unlikely to cause any issues with existing code.

### [Remove `QueuedText`](https://github.com/bevyengine/bevy/pull/7414)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

<!-- TODO -->

### [Remove VerticalAlign from TextAlignment](https://github.com/bevyengine/bevy/pull/6807)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

The `alignment` field of `Text` now only affects the text’s internal alignment.

### [Change default FocusPolicy to Pass](https://github.com/bevyengine/bevy/pull/7161)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

- `FocusPolicy` default has changed from `FocusPolicy::Block` to `FocusPolicy::Pass`

### [Remove `TextError::ExceedMaxTextAtlases(usize)` variant](https://github.com/bevyengine/bevy/pull/6796)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

<!-- TODO -->

### [Flip UI image](https://github.com/bevyengine/bevy/pull/6292)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">UI</div>
</div>

<!-- TODO -->

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

### [Allow not preventing default event behaviors on wasm](https://github.com/bevyengine/bevy/pull/7304)

<div class="migration-guide-area-tags">
    <div class="migration-guide-area-tag">Windowing</div>
</div>

<!-- TODO -->

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

</div>

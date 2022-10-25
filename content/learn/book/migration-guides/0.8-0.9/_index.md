
+++
title = "0.8 to 0.9"
weight = 5
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.8 to 0.9"
+++

### [Rename `play` to `start` and add new `play` method that won't overwrite the existing animation if it's already playing](https://github.com/bevyengine/bevy/pull/6350)

<!-- TODO -->

### [Plugins own their settings. Rework PluginGroup trait.](https://github.com/bevyengine/bevy/pull/6336)

The `WindowDescriptor` settings have been moved from a resource to `WindowPlugin::window`:

```rust
// Old (Bevy 0.8)
app
  .insert_resource(WindowDescriptor {
    width: 400.0,
    ..default()
  })
  .add_plugins(DefaultPlugins)

// New (Bevy 0.9)
app.add_plugins(DefaultPlugins.set(WindowPlugin {
  window: WindowDescriptor {
    width: 400.0,
    ..default()
  },
  ..default()
}))
```

The `AssetServerSettings` resource has been removed in favor of direct `AssetPlugin` configuration:

```rust
// Old (Bevy 0.8)
app
  .insert_resource(AssetServerSettings {
    watch_for_changes: true,
    ..default()
  })
  .add_plugins(DefaultPlugins)

// New (Bevy 0.9)
app.add_plugins(DefaultPlugins.set(AssetPlugin {
  watch_for_changes: true,
  ..default()
}))
```

`add_plugins_with` has been replaced by `add_plugins` in combination with the builder pattern:

```rust
// Old (Bevy 0.8)
app.add_plugins_with(DefaultPlugins, |group| group.disable::<AssetPlugin>());

// New (Bevy 0.9)
app.add_plugins(DefaultPlugins.build().disable::<AssetPlugin>());
```

### [Add a method for accessing the width of a `Table`](https://github.com/bevyengine/bevy/pull/6249)

Any use of `Table::len` should now be `Table::entity_count`. Any use of `Table::capacity` should now be `Table::entity_capacity`.

### [Replace the `bool` argument of `Timer` with `TimerMode`](https://github.com/bevyengine/bevy/pull/6247)

* Replace `Timer::new(duration, false)` with `Timer::new(duration, TimerMode::Once)`.
* Replace `Timer::new(duration, true)` with `Timer::new(duration, TimerMode::Repeating)`.
* Replace `Timer::from_seconds(seconds, false)` with `Timer::from_seconds(seconds, TimerMode::Once)`.
* Replace `Timer::from_seconds(seconds, true)` with `Timer::from_seconds(seconds, TimerMode::Repeating)`.
* Change `timer.repeating()` to `timer.mode() == TimerMode::Repeating`.

### [Rename system chaining to system piping](https://github.com/bevyengine/bevy/pull/6230)

The `.chain(handler_system)` method on systems is now `.pipe(handler_system)`.
The `IntoChainSystem` trait is now `IntoPipeSystem`, and the `ChainSystem` struct is now `PipeSystem`.

### [Make the default background color of `NodeBundle` transparent](https://github.com/bevyengine/bevy/pull/6211)

If you want a `NodeBundle` with a white background color, you must explicitly specify it:

Before:

```rust
let node = NodeBundle {
    ..default()
}
```

After:

```rust
let node = NodeBundle {
    background_color: Color::WHITE.into(),
    ..default()
}
```

### [make `Handle::<T>` field id private, and replace with a getter](https://github.com/bevyengine/bevy/pull/6176)

* If you were accessing the value `handle.id`, you can now do so with `handle.id()`

### [Add `TimeUpdateStrategy` resource for manual `Time` updating](https://github.com/bevyengine/bevy/pull/6159)

<!-- TODO -->

### [Remove `Transform::apply_non_uniform_scale`](https://github.com/bevyengine/bevy/pull/6133)

`Transform::apply_non_uniform_scale` has been removed.
It can be replaced with the following snippet:

```rust
transform.scale *= scale_factor;
```

### [Rename `Transform::mul_vec3` to `transform_point` and improve docs](https://github.com/bevyengine/bevy/pull/6132)

<!-- TODO -->

### [Make `raw_window_handle` field in `Window` and `ExtractedWindow` an `Option`.](https://github.com/bevyengine/bevy/pull/6114)

`Window::raw_window_handle()` now returns `Option<RawWindowHandleWrapper>`.

### [Rename `UiColor`  to `BackgroundColor`](https://github.com/bevyengine/bevy/pull/6087)

`UiColor` has been renamed to `BackgroundColor`. This change affects `NodeBundle`, `ButtonBundle` and `ImageBundle`. In addition, the corresponding field on `ExtractedUiNode` has been renamed to `background_color` for consistency.

### [Merge TextureAtlas::from_grid_with_padding into TextureAtlas::from_grid through option arguments](https://github.com/bevyengine/bevy/pull/6057)

`TextureAtlas::from_grid_with_padding` was merged into `from_grid` which takes two additional parameters for padding and an offset.

```rust
// 0.8
TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
// 0.9
TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None)

// 0.8
TextureAtlas::from_grid_with_padding(texture_handle, Vec2::new(24.0, 24.0), 7, 1, Vec2::new(4.0, 4.0));
// 0.9
TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, Some(Vec2::new(4.0, 4.0)), None)
```

### [Spawn now takes a Bundle](https://github.com/bevyengine/bevy/pull/6054)

```rust
// Old (0.8):
commands
  .spawn()
  .insert_bundle((A, B, C));
// New (0.9)
commands.spawn((A, B, C));

// Old (0.8):
commands.spawn_bundle((A, B, C));
// New (0.9)
commands.spawn((A, B, C));

// Old (0.8):
let entity = commands.spawn().id();
// New (0.9)
let entity = commands.spawn_empty().id();

// Old (0.8)
let entity = world.spawn().id();
// New (0.9)
let entity = world.spawn_empty();
```

### [Accept Bundles for insert and remove. Deprecate insert/remove_bundle](https://github.com/bevyengine/bevy/pull/6039)

Replace `insert_bundle` with `insert`:

```rust
// Old (0.8)
commands.spawn().insert_bundle(SomeBundle::default());
// New (0.9)
commands.spawn().insert(SomeBundle::default());
```

Replace `remove_bundle` with `remove`:

```rust
// Old (0.8)
commands.entity(some_entity).remove_bundle::<SomeBundle>();
// New (0.9)
commands.entity(some_entity).remove::<SomeBundle>();
```

Replace `remove_bundle_intersection` with `remove_intersection`:

```rust
// Old (0.8)
world.entity_mut(some_entity).remove_bundle_intersection::<SomeBundle>();
// New (0.9)
world.entity_mut(some_entity).remove_intersection::<SomeBundle>();
```

Consider consolidating as many operations as possible to improve ergonomics and cut down on archetype moves:

```rust
// Old (0.8)
commands.spawn()
  .insert_bundle(SomeBundle::default())
  .insert(SomeComponent);

// New (0.9) - Option 1
commands.spawn().insert((
  SomeBundle::default(),
  SomeComponent,
))

// New (0.9) - Option 2
commands.spawn_bundle((
  SomeBundle::default(),
  SomeComponent,
))
```

### [`Query` filter types must be `ReadOnlyWorldQuery`](https://github.com/bevyengine/bevy/pull/6008)

Query filter (`F`) generics are now bound by `ReadOnlyWorldQuery`, rather than `WorldQuery`. If for some reason you were requesting `Query<&A, &mut B>`, please use `Query<&A, With<B>>` instead.

### [Change UI coordinate system to have origin at top left corner](https://github.com/bevyengine/bevy/pull/6000)

All flex layout should be inverted (ColumnReverse => Column, FlexStart => FlexEnd, WrapReverse => Wrap)
System where dealing with cursor position should be changed to account for cursor position being based on the top left instead of bottom left

### [Clarify `bevy::ui::Node` field and documentation](https://github.com/bevyengine/bevy/pull/5995)

All references to the old `size` name has been changed, to access `bevy::ui::Node` `size` field use `calculated_size`

### [Remove `AssetServer::watch_for_changes()`](https://github.com/bevyengine/bevy/pull/5968)

`AssetServer::watch_for_changes()` was removed.
Instead, use the `AssetServerSettings` resource.

```rust
app // AssetServerSettings must be inserted before adding the AssetPlugin or DefaultPlugins.
 .insert_resource(AssetServerSettings {
  watch_for_changes: true,
  ..default()
 })
```

### [Remove ambiguity sets](https://github.com/bevyengine/bevy/pull/5916)

Ambiguity sets have been removed.

### [Remove ExactSizeIterator from QueryCombinationIter](https://github.com/bevyengine/bevy/pull/5895)

* Switch to using other methods of getting the length.

### [Support monitor selection for all window modes.](https://github.com/bevyengine/bevy/pull/5878)

`MonitorSelection` was moved out of `WindowPosition::Centered`, into `WindowDescriptor`.
`MonitorSelection::Number` was renamed to `MonitorSelection::Index`.

```rust
// Before
.insert_resource(WindowDescriptor {
    position: WindowPosition::Centered(MonitorSelection::Number(1)),
    ..default()
})
// After
.insert_resource(WindowDescriptor {
    monitor: MonitorSelection::Index(1),
    position: WindowPosition::Centered,
    ..default()
})
```

`Window::set_position` now takes a `MonitorSelection` as argument.

```rust
window.set_position(MonitorSelection::Current, position);
```

### [Miscellaneous code-quality improvements.](https://github.com/bevyengine/bevy/pull/5860)

<!-- TODO -->

### [Add `pop` method for `List` trait.](https://github.com/bevyengine/bevy/pull/5797)

* Any custom type that implements the `List` trait will now need to implement the `pop` method.

### [Add global time scaling](https://github.com/bevyengine/bevy/pull/5752)

* `time.time_since_startup()` -> `time.elapsed()`
* `time.seconds_since_startup()` -> `time.elapsed_seconds_f64()`
* `time.seconds_since_startup_wrapped_f32()` -> `time.elapsed_seconds_wrapped()`

If you aren’t sure which to use, most systems should continue to use “scaled” time (e.g. `time.delta_seconds()`). The realtime “unscaled” time measurements (e.g. `time.raw_delta_seconds()`) are mostly for debugging and profiling.

### [Move `sprite::Rect` into `bevy_math`](https://github.com/bevyengine/bevy/pull/5686)

The `bevy::sprite::Rect` type moved to the math utility crate as
`bevy::math::Rect`. You should change your imports from `use bevy::sprite::Rect` to `use bevy::math::Rect`.

### [Make `Children` constructor `pub(crate)`.](https://github.com/bevyengine/bevy/pull/5532)

<!-- TODO -->

### [Remove `Sync` bound from `Local`](https://github.com/bevyengine/bevy/pull/5483)

* Any code relying on `Local<T>` having `T: Resource` may have to be changed, but this is unlikely.

### [Add associated constant `IDENTITY` to `Transform` and friends.](https://github.com/bevyengine/bevy/pull/5340)

The method `identity()` on `Transform`, `GlobalTransform` and `TransformBundle` has been deprecated.
Use the associated constant `IDENTITY` instead.

### [`Gamepad` type is `Copy`; do not require / return references to it in `Gamepads` API](https://github.com/bevyengine/bevy/pull/5296)

* `Gamepads::iter` now returns an iterator of `Gamepad`. rather than an iterator of `&Gamepad`.
* `Gamepads::contains` now accepts a `Gamepad`, rather than a `&Gamepad`.

### [Add Exponential Moving Average into diagnostics](https://github.com/bevyengine/bevy/pull/4992)

<!-- TODO -->

### [Swap out num_cpus for std::thread::available_parallelism](https://github.com/bevyengine/bevy/pull/4970)

`bevy_tasks::logical_core_count` and `bevy_tasks::physical_core_count` have been removed. `logical_core_count` has been replaced with `bevy_tasks::available_parallelism`, which works identically. If `bevy_tasks::physical_core_count` is required, the `num_cpus` crate can be used directly, as these two were just aliases for `num_cpus` APIs.

### [Extract Resources into their own dedicated storage](https://github.com/bevyengine/bevy/pull/4809)

Resources have been moved to `Resources` under `Storages` in `World`. All code dependent on `Archetype::unique_components(_mut)` should access it via `world.storages().resources()` instead.

All APIs accessing the raw data of individual resources (mutable _and_ read-only) have been removed as these APIs allowed for unsound unsafe code. All usages of these APIs should be changed to use `World::{get, insert, remove}_resource`.

### [Implement `Bundle` for `Component`. Use `Bundle` tuples for insertion](https://github.com/bevyengine/bevy/pull/2975)

<!-- TODO -->

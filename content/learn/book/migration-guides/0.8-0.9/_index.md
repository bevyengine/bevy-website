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

### [bevy_scene: Replace root list with struct](https://github.com/bevyengine/bevy/pull/6354)

The scene file format now uses a struct as the root object rather than a list of entities. The list of entities is now found in the `entities` field of this struct.

```rust
// OLD
[
  (
    entity: 0,
    components: [
      // Components...
    ]
  ),
]

// NEW
(
  entities: [
    (
      entity: 0,
      components: [
        // Components...
      ]
    ),
  ]
)
```

### [Rename `play` to `start` and add new `play` method that won't overwrite the existing animation if it's already playing](https://github.com/bevyengine/bevy/pull/6350)

* If you were using `play` to restart an animation that was already playing, that functionality has been moved to `start`. Now, `play` won’t have any effect if the requested animation is already playing.

### [feat: add GamepadInfo, expose gamepad names](https://github.com/bevyengine/bevy/pull/6342)

* Pattern matches on `GamepadEventType::Connected` will need to be updated, as the form of the variant has changed.
* Code that requires `GamepadEvent`, `GamepadEventRaw` or `GamepadEventType` to be `Copy` will need to be updated.

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

### [Update `wgpu` to 0.14.0, `naga` to `0.10.0`, `winit` to 0.27.4, `raw-window-handle` to 0.5.0, `ndk` to 0.7](https://github.com/bevyengine/bevy/pull/6218)

* Adjust usage of `bevy_window::WindowDescriptor`’s `cursor_locked` to `cursor_grab_mode`, and adjust its type from `bool` to `bevy_window::CursorGrabMode`.

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

`Transform::mul_vec3` has been renamed to `transform_point`.

### [Make `raw_window_handle` field in `Window` and `ExtractedWindow` an `Option`.](https://github.com/bevyengine/bevy/pull/6114)

`Window::raw_window_handle()` now returns `Option<RawWindowHandleWrapper>`.

### [Rename `UiColor`  to `BackgroundColor`](https://github.com/bevyengine/bevy/pull/6087)

`UiColor` has been renamed to `BackgroundColor`. This change affects `NodeBundle`, `ButtonBundle` and `ImageBundle`. In addition, the corresponding field on `ExtractedUiNode` has been renamed to `background_color` for consistency.

### [Merge `TextureAtlas::from_grid_with_padding` into `TextureAtlas::from_grid` through option arguments](https://github.com/bevyengine/bevy/pull/6057)

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

### [Accept Bundles for insert and remove. Deprecate `insert`/`remove_bundle`](https://github.com/bevyengine/bevy/pull/6039)

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

### [Remove `ExactSizeIterator` from `QueryCombinationIter`](https://github.com/bevyengine/bevy/pull/5895)

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

### [Remove unused DepthCalculation enum](https://github.com/bevyengine/bevy/pull/5684)

Remove references to `bevy_render::camera::DepthCalculation`, such as `use bevy_render::camera::DepthCalculation`. Remove `depth_calculation` fields from Projections.

### [remove `ReflectMut` in favor of `Mut<dyn Reflect>`](https://github.com/bevyengine/bevy/pull/5630)

<!-- TODO -->

### [Make internal struct `ShaderData` non-`pub`](https://github.com/bevyengine/bevy/pull/5609)

<!-- TODO -->

### [changed diagnostics from seconds to milliseconds](https://github.com/bevyengine/bevy/pull/5554)

<!-- TODO -->

### [Make `Children` constructor `pub(crate)`.](https://github.com/bevyengine/bevy/pull/5532)

`Children::with()` is now renamed `Children::from_entities()` and is now `pub(crate)`

### [Remove `Sync` bound from `Local`](https://github.com/bevyengine/bevy/pull/5483)

* Any code relying on `Local<T>` having `T: Resource` may have to be changed, but this is unlikely.

### [Add `FromWorld` bound to `T` in `Local<T>`](https://github.com/bevyengine/bevy/pull/5481)

* It might be possible for references to `Local`s without `T: FromWorld` to exist, but these should be exceedingly rare and probably dead code. In the event that one of these is encountered, the easiest solutions are to delete the code or wrap the inner `T` in an `Option` to allow it to be default constructed to `None`.

### [bevy_reflect: Update enum derives](https://github.com/bevyengine/bevy/pull/5473)

Bevy-defined enums have been updated to implement `Enum` and are not considered value types (`ReflectRef::Value`) anymore. This means that their serialized representations will need to be updated. For example, given the Bevy enum:

```rust
pub enum ScalingMode {
  None,
  WindowSize,
  Auto { min_width: f32, min_height: f32 },
  FixedVertical(f32),
  FixedHorizontal(f32),
}
```

You will need to update the serialized versions accordingly.

```js
// OLD FORMAT
{
  "type": "bevy_render::camera::projection::ScalingMode",
  "value": FixedHorizontal(720),
},

// NEW FORMAT
{
  "type": "bevy_render::camera::projection::ScalingMode",
  "enum": {
    "variant": "FixedHorizontal",
    "tuple": [
      {
        "type": "f32",
        "value": 720,
      },
    ],
  },
},
```

This may also have other smaller implications (such as `Debug` representation), but serialization is probably the most prominent.

### [Remove `Size` and `UiRect` generics](https://github.com/bevyengine/bevy/pull/5404)

* The generic `T` of `Size` and `UiRect` got removed and instead they both now always use `Val`. If you used a `Size<f32>` consider replacing it with a `Vec2` which is way more powerful.

### [Add associated constant `IDENTITY` to `Transform` and friends.](https://github.com/bevyengine/bevy/pull/5340)

The method `identity()` on `Transform`, `GlobalTransform` and `TransformBundle` has been deprecated.
Use the associated constant `IDENTITY` instead.

### [`Gamepad` type is `Copy`; do not require / return references to it in `Gamepads` API](https://github.com/bevyengine/bevy/pull/5296)

* `Gamepads::iter` now returns an iterator of `Gamepad`. rather than an iterator of `&Gamepad`.
* `Gamepads::contains` now accepts a `Gamepad`, rather than a `&Gamepad`.

### [remove blanket `Serialize + Deserialize` requirement for `Reflect` on generic types](https://github.com/bevyengine/bevy/pull/5197)

<!-- TODO -->

### [Add Exponential Moving Average into diagnostics](https://github.com/bevyengine/bevy/pull/4992)

`LogDiagnosticsPlugin` now records the smoothed value rather than the raw value.

* For diagnostics recorded less often than every 0.1 seconds, this change to defaults will have no visible effect.
* For discrete diagnostics where this smoothing is not desirable, set a smoothing factor of 0 to disable smoothing.
* The average of the recent history is still shown when available.

### [Swap out `num_cpus` for `std::thread::available_parallelism`](https://github.com/bevyengine/bevy/pull/4970)

`bevy_tasks::logical_core_count` and `bevy_tasks::physical_core_count` have been removed. `logical_core_count` has been replaced with `bevy_tasks::available_parallelism`, which works identically. If `bevy_tasks::physical_core_count` is required, the `num_cpus` crate can be used directly, as these two were just aliases for `num_cpus` APIs.

### [Extract Resources into their own dedicated storage](https://github.com/bevyengine/bevy/pull/4809)

Resources have been moved to `Resources` under `Storages` in `World`. All code dependent on `Archetype::unique_components(_mut)` should access it via `world.storages().resources()` instead.

All APIs accessing the raw data of individual resources (mutable _and_ read-only) have been removed as these APIs allowed for unsound unsafe code. All usages of these APIs should be changed to use `World::{get, insert, remove}_resource`.

### [Change `gamepad.rs` tuples to normal structs](https://github.com/bevyengine/bevy/pull/4519)

* The `Gamepad`, `GamepadButton`, `GamepadAxis`, `GamepadEvent` and `GamepadEventRaw` types are now normal structs instead of tuple structs and have a `new()` function. To migrate change every instantiation to use the `new()` function instead and use the appropriate field names instead of `.0` and `.1`.

### [Rename `ElementState` to `ButtonState`](https://github.com/bevyengine/bevy/pull/4314)

* The `ElementState` type received a rename and is now called `ButtonState`. To migrate you just have to change every occurrence of `ElementState` to `ButtonState`.

### [Move `Size` to `bevy_ui`](https://github.com/bevyengine/bevy/pull/4285)

* The `Size` type got moved from `bevy::math` to `bevy::ui`. To migrate you just have to import `bevy::ui::Size` instead of `bevy::math::Math` or use the `bevy::prelude` instead.

### [Remove `margins.rs`](https://github.com/bevyengine/bevy/pull/4284)

* The `Margins` type got removed. To migrate you just have to change every occurrence of `Margins` to `UiRect`.

### [Remove `face_toward.rs`](https://github.com/bevyengine/bevy/pull/4277)

* The `FaceToward` trait got removed. To migrate you just have to change every occurrence of `Mat4::face_toward` to `Mat4::look_at_rh`.

### [Move `Rect` to `bevy_ui` and rename it to `UiRect`](https://github.com/bevyengine/bevy/pull/4276)

* The `Rect` type got renamed to `UiRect`. To migrate you just have to change every occurrence of `Rect` to `UiRect`.

### [Implement `Bundle` for `Component`. Use `Bundle` tuples for insertion](https://github.com/bevyengine/bevy/pull/2975)

In `derive(Bundle)`, the `bundle` attribute has been removed. Nested bundles are not collapsed automatically. You should remove `#[bundle]` attributes.

+++
title = "Bevy 0.20"
date = 2026-09-17
[extra]
show_image = false
public_draft = 2582
status = 'hidden'
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous donors**](/donate), we're happy to announce the **Bevy 0.20** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevy.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.20**, check out our [0.19 to 0.20 Migration Guide](/learn/migration-guides/0-19-to-0-20/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

- **X**: TODO

<!-- more -->

## BSN Syntax Improvements

{{ heading_metadata(authors=[] prs=[25318, 25626]) }}

BSN landed with a few idiosyncrasies that caused friction in practice. We made some changes to BSN's syntax this cycle in the interest of improving its ergonomics and clarity.

All scene references now require `@` prefixes:

```rust
// Before
bsn! {
    scene_variable
    scene_function()
    {scene_expression}
}

// After
bsn! {
    @scene_variable
    @scene_function()
    @{scene_expression}
}
```

This freed us up to make component values _much_ easier to work with.

```rust
// Before
bsn! {
    template_value(component_variable)
    template_value(component_function())
}

// After
bsn! {
    component_variable
    component_function()
}
```

Enums no longer require `VariantDefaults` or `FromTemplate`, provided they implement `Default` and `Clone`:

```rust
// Before
#[derive(Component, Default, Clone, VariantDefaults)]
enum Foo {
    A { x: u32, y: u32 },
    #[default]
    B,
}

bsn! {
    Foo::B
}

// After
#[derive(Component, Default, Clone)]
enum Foo {
    A { x: u32, y: u32 },
    #[default]
    B,
}

bsn! {
    Foo::B
}
```

If you were using an enum that didn't support `VariantDefaults`, you can remove the `template_value` wrapper:

```rust
// Before
bsn! {
    template_value(Foo::A)
}
// After
bsn! {
    Foo::A
}
```

The "variant defaults" pattern, which relied on defining individual "default" constructors for each variant is what allowed "individual enum field value patching" (ex: `VariantDefaults` and `FromTemplate` would define `Foo::a_default()` and `Foo::b_default()` in the example above). This is no longer supported, as the weirdness factor (and Rust ecosystem compatibility challenges) were too costly. When working with enums in BSN, you must now specify each field in the enum, just like you would in normal Rust (which doesn't have support for individual enum variant defaults).

```rust
// Before (y field is initialized to its default value)
bsn! {
    Foo::A { x: 1 }
}

// After (y field must be manually specified)
bsn! {
    Foo::A { x: 1, y: 0 }
}
```

The "builder pattern" previously required a `template_value` wrapper. This can now be removed:

```rust
// Before
bsn! {
    template_value(Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
}
// After
bsn! {
    Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y)
}
```

Additionally, you can now remove the `template_value` wrapper in cases like this:

```rust
// Before
bsn! {
    template_value(node.clone())
}
// After
bsn! {
    node.clone()
}
```

In general, you should now be able to remove all `template_value` instances from your BSN declarations!

List syntax in BSN has also been improved. BSN used to use commas to separate entities, with optional `()` around entities to make the boundaries clearer. This resulted in a lot of syntax noise, line noise, and over-indentation:

```rust
bsn! {
    Node 
    Children [
        (
            #OkButton
            @button("Ok")
        ),
        (
            #CancelButton
            @button("Cancel")
        ),
    ]
}
```

To avoid this, many developers opted for this syntax instead, which made it very hard to visually distinguish entities:

```rust
bsn! {
    Node 
    Children [
        #OkButton
        @button("Ok"),
        #CancelButton
        @button("Cancel"),
    ]
}
```

BSN now uses `--` to separate entities:

```rust
bsn! {
    Node 
    Children [
        #OkButton
        @button("Ok")
        --
        #CancelButton
        @button("Cancel")
    ]
}
```

This gives us the best of all worlds: entities are visually distinct, and there is no over-indentation, line noise, or syntax noise. Both `()` and `,` have been deprecated in this context.

Using `[]` and `()` for `bsn_list!` (and `bsn!`) is now discouraged / warned against (ex: `bsn_list []`), as it can result in poor rustfmt autoformatting. Instead, use `bsn_list! {}`, which is the only syntax that `rustfmt` won't touch. Don't worry, we
plan to build a BSN auto-formatter!

```rust
bsn_list! {
    #Ok @button("Ok")
    --
    #Cancel @button("Cancel")
}
```

## Ready Event

{{ heading_metadata(authors=["@cart"] prs=[]) }}

We landed BSN, Bevy's next generation scene system, [in our last release](/news/bevy-0-19). It was missing a key piece though: the ability to easily run logic when a scene is fully "ready" and spawned (ex: all dependencies have loaded, the full hierarchy is present, and all of the initial components are inserted in the scene). This is a critical piece for building cohesive, standalone, composable scenes. It is also necessary to properly layer Bevy logic on top of _other_ scene representations (like glTF).

The closest we had was the `Add` event for a given component, which runs "top down" (meaning children are not available). We needed a "bottom up" equivalent to enable building logic that relies on the complete loaded and spawned scene.

The solution is pretty straightforward: trigger a new `Ready` event for each entity in a spawned scene _after_ the full spawn logic has run for that entity (including its descendants).

This enables the following:

```rust
#[derive(SceneComponent, Default, Clone)]
struct Widget;

impl Widget {
    fn scene() -> impl Scene {
        bsn! {
            Node { width: px(100), height: px(100) }
            on(|ready: On<Ready>| {
                info!("The full scene, including 'widget.bsn' contents, is available here")
            })
            Children [
                Text("hello"),
                :"widget.bsn"
            ]
        }
    }
}

world.spawn(bsn!{ @Widget })
```

## Even more Feathers widgets

{{ heading_metadata(authors=["@viridia"] prs=[24092]) }}

Feathers, Bevy's editor-centric UI toolkit, now has more widgets for you to play with:

- scrollbar
- list view

These remain deliberately opinionated to ensure simplicity and consistency of Bevy's development tools.
If you would like to use these widgets in your game projects, we encourage you to vendor these into your project,
tweaking their style to your heart's content.

## FeathersSelect control

{{ heading_metadata(authors=["@gagnus"] prs=[24847]) }}

### Goals

- Adds a new dropdown select control to feathers, for selecting one of a number of options. It is similar to the existing `FeathersListView` (and indeed is implemented using one), it also uses a `FeathersMenuPopup` to show the selection
when opened.
- The dropdown scrolls if there are more than `@max_visible` options
- Adds a helper `caption(...)` function, similar to `label()` which emits `(Text(...) ThemedText)`
- Adds a `SetSelected` event to the underlying `ListBox` bevy_ui_widgets control, which sets its selected value, similar to how `SetChecked` and `SetSliderValue` work.

### Usage

Inside a `bsn!` macro use FeathersSelect scene entity and give it a number of options which contain the `FeathersListRow` component

```rust
(
  @FeathersSelect {
      @options: ... a Box<dyn SceneList> set of rows, see feathers_gallery for an example
      @max_visible: 6,
  }
),
```

## Feathers Color Input and Color Swatch Grid Widgets

{{ heading_metadata(authors=["@viridia"] prs=[25446]) }}

Feathers has two new widgets for editing colors:

- `FeathersColorSwatchGrid` displays a 2D grid of clickable color swatches.
  Internally, these are radio buttons, so only one can be selected at a time.
  As radio buttons they support focus and keyboard navigation. The color values
  can be set from a Vec of colors, and if there are fewer colors in the Vec
  than there are slots in the grid, the extra slots will be filled with an
  empty cell placeholder.
- `FeathersColorInput` is a small button displaying a color swatch. When
  clicked, it brings up a drop-down menu displaying a color picker that supports
  both RGB and HSL picking modes. This widget can be placed anywhere that you
  want to display a color and make it editable. The widget also supports
  editing via hex, and displays a grid of recently-edited colors.

## Headless tab widgets

{{ heading_metadata(authors=["@jbuehler23"] prs=[25515]) }}

`bevy_ui_widgets` now has headless tab-strip behavior: a `TabList` container and `Tab` headers, with no built-in visuals.

Selection is externally owned. `SelectedTab` on the list holds the selected tab; interaction emits `ValueChange<Option<Entity>>` as a request, applied by the app or by the optional `tablist_self_update` observer.

Arrow keys move a roving tab index along the list's axis, Home and End jump to the ends, and Enter or Space activates. `TabActivation::Automatic` selects as focus moves; the default `Manual` keeps focus and selection separate.

Tabs derive `Selected`, install tab accessibility roles, and honor `InteractionDisabled` per tab or on the whole list. Only primary-button clicks change selection.

```rust
(
    TabList::default()
    SelectedTab(Some(first_tab))
    on(tablist_self_update)
    Children [
        (Tab Children [Text("General")]),
        (Tab Children [Text("Rendering")]),
    ]
),
```

See the `headless_tabs` example for controlled and self-updating tab lists in both orientations.

## Feathers menu improvements

{{ heading_metadata(authors=["@viridia"] prs=[24784]) }}

In addition to `FeathersMenu`, there is now `FeathersLazyMenu`. While the former expects a
pre-spawned popup entity which is hidden, the latter dynamically spawns the popup when the menu
is opened, and despawns it when closed. This is the recommended approach for menu popups
whose list of menu items is long, expensive to keep around, or dynamically-generated.

In addition to `FeathersMenuButton`, there is now also `FeathersMenuToolButton` which works the
same as a regular menu button but which has the tool button form factor.

## Solari on Metal

{{ heading_metadata(authors=["@mate-h"] prs=[25123]) }}

Ray tracing on Metal has been available since wgpu 29, and with bindless storage buffers landing in wgpu 30, Solari now runs on Apple Silicon Macs.

Run the Solari example on a compatible Mac:

```sh
cargo run --example solari --features bevy_solari,https,free_camera
```

Denoising is not available on Metal yet, DLSS is NVIDIA-only. MetalFX Ray Reconstruction or Open Image Denoise 3.0 are promising paths for cross-platform denoising in the future.

## Solari TODO

{{ heading_metadata(authors=["@JMS55"] prs=[]) }}

STUB TODO

## Mesh Shaders.

{{ heading_metadata(authors=[] prs=[25627]) }}

Mesh shaders are now integrated with Bevy's pipeline cache and are available for advanced users to take advantage of.
Mesh shaders can be used to render

- Meshlets generated using tools like [meshoptimizer](https://meshoptimizer.org/)
- Procedural grass with dynamic level-of-detail, [as seen here](https://gpuopen.com/learn/mesh_shaders/mesh_shaders-procedural_grass_rendering/)
- Voxels, like [nvidium](https://github.com/MCRcortex/nvidium)
- Particles
- and more

Mesh shaders, at a high level, replace the classic vertex shader with a compute shader.
This allows generating geometry directly on the GPU and passing those generated primitives directly to the fragment shader without using multiple pipelines or intermediary buffers (to pass data from a compute shader to a render pipeline).

A `MeshPipeline` contains:

- an optional task shader (also known as amplification shader)
- a mesh shader
- a fragment shader

The new `MeshPipelineDescriptor` can be used to define a `MeshPipeline`.
That `MeshPipeline` is then used as a `RenderPipeline`, which allows the re-use of Bevy's lower level rendering APIs such as `RenderContext::begin_tracked_render_pass` to take advantage of the new `draw_mesh_tasks` APIs.

```rust
let mut pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
    label: Some("custom_mesh_shader_pass"),
    color_attachments: &[Some(target.get_color_attachment())],
    depth_stencil_attachment: Some(depth.get_attachment(StoreOp::Store)),
    ..default()
});

pass.set_render_pipeline(mesh_pipeline);
pass.set_bind_group(0, &bind_group, &[view_uniform_offset.offset]);

// draw_mesh_tasks dispatches the task shader if there is one,
// or dispatches the mesh shader if there is no task shader.
pass.draw_mesh_tasks(1, 1, 1);
```

It is notable that mesh shaders are an advanced graphics approach with platform-specific performance considerations, and that this is the initial base support for the feature.
Higher level user APIs, and easy integration with Bevy's `StandardMaterial`, are left to future work.

Mesh shaders are not supported on web platforms.

Check out the new `mesh_shader_intro` example for more usage examples.

## TextInput

{{ heading_metadata(authors=["@viridia"] prs=[]) }}

The `EditableText` component has been split into two components, which are now `EditableText`
and `TextInput`. The `EditableText` component, which lives in the `bevy::text` crate, holds the
state of a text input field, but no longer has any built-in observers - that is, it does not
behave like a widget (headless or otherwise), but merely a holder of state.

All of the widget-like behaviors (responding to keystrokes) have been moved to a new `TextInput`
component which lives in the `bevy::ui_widgets` crate. Not only is the arrangement more consistent
with the other widgets, but in addition this new component also has properties which are only
interesting to widgets, like a "read-only" option.

## Sprite materials

{{ heading_metadata(authors=["@cookie1170"] prs=[25415]) }}

**TODO: Add recording showcasing the `sprite_material` example**

Until now, Bevy's sprite renderer has been lacking a major feature: the ability to extend it with custom shaders!
With this release, it's now possible to create custom materials for sprites by implementing the `MaterialExtension2d` trait,
inserting the `SpriteMaterial` component and adding the `SpriteMaterialPlugin` to your app.

The shader can use functions exported from `bevy_sprite_render::sprite_mesh::functions`, including:

```wesl
// Samples the sprite's final color, including the tint and alpha discard, at a given UV.
fn sample_final_color(uv: vec2<f32>, instance_index: u32) -> vec4<f32>;

// Samples the sprite's texture without tint and alpha discard at a given UV.
fn sample_sprite_texture(uv: vec2<f32>, instance_index: u32) -> vec4<f32>;

// Applies tint and alpha discard to the sprite's color.
fn get_final_color(sprite_color: vec4<f32>, instance_index: u32) -> vec4<f32>;
```

Check out the `sprite_material` example to see it in action!

## Extended material 2D

{{ heading_metadata(authors=["@cookie1170"] prs=[25183]) }}

Bevy now provides a 2D analog to 3D's [`ExtendedMaterial`], which can be used to extend an existing material by implementing the `MaterialExtension2d` trait:

```rs
#[derive(AsBindGroup, Reflect, Clone)]
struct MyExtendedMaterial {
    // Make sure to make this high enough to not collide with the base material's uniforms!
    #[uniform(20)]
    important_binding: Vec4,
}

impl MaterialExtension2d for MyExtendedMaterial {
    fn vertex_shader() -> Option<ShaderRef> {
        None // Return `Some` to override the base material's vertex shader
    }

    fn fragment_shader() -> Option<ShaderRef> {
        None // Return `Some` to override the base material's fragment shader
    }

    fn depth_bias(&self) -> Option<f32> {
        None // Return `Some` to override the base material's depth bias
    }

    fn alpha_mode(&self) -> Option<AlphaMode2d> {
        None // Return `Some` to override the base material's alpha mode
    }
}
```

This material can now be used in an `ExtendedMaterial2d` struct:

```rs
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Make sure to add a plugin for the material!
            Material2dPlugin::<ExtendedMaterial2d<ColorMaterial, MyExtendedMaterial>>::default()
        ))
        .add_systems(Startup, spawn_extended_material_mesh)
        .run();
}

fn spawn_extended_material_mesh(
    mut commands: Commands,
    mut materials: ResMut<Assets<ExtendedMaterial2d<ColorMaterial, MyExtendedMaterial>>>,
) {
    // Create an extended material with a `ColorMaterial` as the base and `MyExtendedMaterial` as the extension
    // `ColorMaterial`'s bindings will be available to `MyExtendedMaterial`'s shader
    let material = ExtendedMaterial2d {
        base: ColorMaterial::from_color(Color::WHITE),
        extension: MyExtendedMaterial {
            important_binding: Vec4::ZERO,
        },
    };

    let handle = materials.add(material);
    commands.spawn((
        Mesh2d,
        MeshMaterial2d(handle),
    ));
}
```

[`ExtendedMaterial`]: https://docs.rs/bevy/latest/bevy/pbr/struct.ExtendedMaterial.html

## Sprite render backend migration

{{ heading_metadata(authors=["@IceSentry"] prs=[25432]) }}

The sprite render backend was replaced by a new backend that reuses a lot of the infrastructure made for 3d.
This resulted in improved performance in many cases and also makes future maintenance and improvements easier.

## Pan Orbit Camera

{{ heading_metadata(authors=["@aevyrie, @taishi-sama"] prs=[25434]) }}

Upstream of awesome crate [`bevy_editor_cam`](https://github.com/aevyrie/bevy_editor_cam) made by [@aevyrie](https://github.com/aevyrie) as part of `bevy_camera_controller` crate!

### Usage

Add `MeshPickingPlugin` and `DefaultPanOrbitCameraPlugins` plugin.

```rust
app.add_plugins((
    MeshPickingPlugin,
    DefaultPanOrbitCameraPlugins,
))
```

Then add `PanOrbitCamera` component on any 3D camera.

```rust
commands.spawn((
    Camera3d::default(),
    PanOrbitCamera::default(),
))
```

Full functionality is shown in the `camera/pan_orbit_camera_cad` example

```sh
cargo run --example pan_orbit_camera_cad --features='pan_orbit_camera https 3d_api jpeg'
```

## Weak system ordering with chain_weak

{{ heading_metadata(authors=["@JMS55"] prs=[25128]) }}

Ordering large groups of systems with `.chain()` is convenient, but it can be
overly strict. If system set `X` is chained before system set `Y`, every system
in `X` must finish before *any* system in `Y` can start, even when the systems
involved never touch the same data. This often leaves worker threads idle while they
wait for a handful of stragglers at the end of a system set, a pattern that shows up
frequently in the render world.

The new `chain_weak()`, `before_weak()`, and `after_weak()` functions provide a looser alternative.
Like their regular counterparts, they request an ordering between successive elements, however that
ordering is only kept between systems whose data accesses actually conflict. Systems that don't
conflict are left unordered and may run in any order, including in parallel.

```rust
schedule.configure_sets(
    (
        ExtractCommands,
        PrepareMeshes,
        CreateViews,
        Specialize,
        PrepareViews,
        Queue,
        PhaseSort,
        Prepare,
        Render,
        Cleanup,
        PostCleanup,
    )
        .chain_weak(),
);
```

When two weakly-ordered systems actually conflict on their data access, a normal
ordering is kept between them, so the earlier one still runs first. Two
systems that conflict only through a non-conflicting system between them in the chain
stay ordered as well. Non-conflicting systems, however, are left free to run in any
order and overlap for increased parallelism!

Two kinds of system are treated as always conflicting, so their ordering is always
kept: an earlier system that produces deferred effects such as `Commands` (so the
later system observes them, with an `ApplyDeferred` sync point inserted as usual), and
exclusive systems (which cannot overlap anything regardless).

Because the scheduler can only see accesses it tracks, dependencies expressed
through interior mutability on read-only accesses, global state, or other untracked
methods are **not** respected. Use `chain_weak` only when your systems don't rely
on such hidden ordering, otherwise stick with `chain`.

## Contextual theming

{{ heading_metadata(authors=["@viridia"] prs=[24969]) }}

Feathers now supports "contextual theming", meaning that the theme variables can change depending
on the parent entity. So widgets that are inside of a dialog box or subpanel can have different
colors than widgets that are on a regular panel or window background.

The design follows that of popular web toolkits like MUI, Radix, or Chakra. There's a new component,
`ThemeContext`, which lets you select which color scheme the widget's descendants should use;
currently the available schemes are `Base`, `Higher`, `Highest`, and `Floating`, which correspond
to the design plans for the Bevy scene editor.

The theme context is used in conjunction with a new kind of design token, named `SemanticToken`.
The lookup process for a color now requires two stages: the `ThemeToken` is converted into a
`SemanticToken`, and then the combination of `SemanticToken` and `ThemeContext` is used to look up
a color.

In addition to allowing context-specific color choices, this also makes it easier to design new
themes! Instead of having to tediously choose colors for a hundred different theme tokens,
the set of semantic tokens is much smaller, and the relationship between token and color is much
more intuitive.

## Val::Em and Val::Rem

{{ heading_metadata(authors=["@gagnus"] prs=[25231]) }}

Bevy UI now supports `em` and `rem` as sizing units. `em` is the current font size (represented by an `EmSize` component), `rem` is a
global "root" font size (represented by the existing `RemSize` resource).

`EmSize` is derived from `TextFont` when one is on the same entity; propagating it down the hierarchy is left to your app.

This is especially useful if you might want to vary your text size after authoring your UIs, for example as an accessibility feature or
just to improve your UI on different devices.

```rust
bsn! {
    Node { width: em(10) }
    Text("Hello")
    TextFont { font_size: FontSize::Rem(1.5) }
}
```

The default font-size is now `rem(1)` rather than `px(20)`. This is a no-op if you're not changing `RemSize` but it means your
text will scale by default when you do.

## New CPU UI clipping implementation supporting rotation

{{ heading_metadata(authors=["@Ickshonpe"] prs=[24148]) }}

Until now, Bevy ignored rotation and scaling when clipping UI node overflow, causing content to be distorted or clipped in the wrong place. In Bevy 0.20 overflow clipping now supports rotated clipping regions, using a simplified version of the Sutherland–Hodgman algorithm.Descendants are clipped against each of the clipping regions inherited from their ancestors, instead of against a single axis aligned rectangle. The user-facing overflow API is unchanged.

## Per-column change ticks

{{ heading_metadata(authors=["@pcwalton", "@SkiFire13"] prs=[25157, 25429]) }}

A new summary change tick is now stored per-column, representing the last time any component in that column was changed. This allows skipping the whole column in case no component was changed, as opposed to going through each component checking them individually.

## Fallback Fonts

{{ heading_metadata(authors=["@Ickshonpe"] prs=[24378]) }}

`FontSource` now supports font fallback lists. Use `FontSource::families("Arial, 'Noto Sans', sans-serif")` for CSS-style font family lists, or `FontSource::list([...])` to combine font handles, named families, CSS lists, and generic families.

## FixedNode

{{ heading_metadata(authors=["@Ickshonpe"] prs=[24323]) }}

`FixedNode` is a new marker component for Bevy UI.

A UI node entity with the `FixedNode` component is positioned relative to the target camera's viewport rather than its parent element. `FixedNode`s don't inherit their parent's layout, clipping or transform context.

In the Taffy layout (stored in `UiSurface`) there is nothing to distinguish `FixedNode`s and root nodes, so they are treated identically during updates.

## Elliptical Border Radius

{{ heading_metadata(authors=["@ickshonpe"] prs=[24779]) }}

Bevy UI can now draw Nodes with elliptical border geometry.

The fields of `BorderRadius` are now `CornerRadius`s to enable different radius to be set for each axis.

The `BorderRadius` constructor and update functions are no longer `const`, and their parameters take `Into<CornerRadius>`s instead of `Val`s. `CornerRadius` now implements `From<Val>` so most existing code using border radius shouldn't require changes:

```rust
let a = BorderRadius::all(CornerRadius::circular(vh(10.)));
let b = BorderRadius::all(vh(10.)); // a == b
let c = BorderRadius::top_right(CornerRadius::new(px(10.), px(20.)));
```

## Schedule randomization

{{ heading_metadata(authors=["@andriyDev"] prs=[25094]) }}

Before a schedule runs (and therefore, your systems), it first computes the order that systems run
in based on the ordering constraints (`.before()`, `.after()`, `.chain()`) of systems and system
sets. However, in addition to this, the schedule must also resolve **conflicts** - if system A and
system B both mutate component C, and there's no ordering between A and B, the schedule needs to
pick one to run first. So far, the rule has been that this is non-deterministic.

In practice though, schedules pick the order of these conflicting systems "deterministically, but
arbitrarily". Put simply, your systems might accidentally be in the right order, but making an
unrelated change to the graph might suddenly put it in the wrong order. This problem can be very
difficult to detect.

Introducing schedule randomization! This will randomize the order of systems while maintaining any
explicit system ordering constraints. Once the `debug` feature is enabled, `ScheduleBuildSettings`
will include a `shuffle_seed` field, that users can set to randomize their schedules. For example:

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .edit_schedule(Update, |schedule| {
        // Make sure to add the `rand` crate with `cargo add rand`.
        let rng_seed: u64 = rand::random();
        // Consider logging out the seed, so you can reproduce the error if you find a bug!
        info!("Randomizing Update schedule with seed={rng_seed}");
        schedule.set_build_settings(ScheduleBuildSettings {
            shuffle_seed: Some(rng_seed),
            ..Default::default()
        });
    })
    .run();
```

This can be used for "property testing", to verify that your systems satisfy some property despite
different orderings of systems.

There are some caveats however. Currently, when using `auto_insert_apply_deferred`, systems with
commands are always placed before the earliest sync point they can. This means that although your
systems may not have the correct ordering, they might "accidentally" have the correct ordering
because of which sync point it uses. We hope to fix this in the future.

In addition, the multi-threaded executor executes systems greedily: it looks for the first
unexecuted system whose dependencies are finished and that has no other conflicting systems running.
The result is that even if the shuffle results in the order `(A, B, C)`, `C` could run before `B` if
`A` and `B` conflict. **This can be desirable to test**, but consider using the single-threaded
executor to avoid this case.

## Add scrubbing / dragging to number_input widget

{{ heading_metadata(authors=["@viridia"] prs=[24636, 24701]) }}

The `FeathersNumberInput` widget has been substantially overhauled, with several new features.

Blender's [numeric input](https://docs.blender.org/manual/en/latest/interface/controls/buttons/fields.html)
is great, and we've borrowed its best elements. This includes support for multiple
editing modes — including "scrubbing" (click-and-drag) and direct keyboard
entry. The updated feathers widget is now much closer to feature parity with Blender.

The widget supports editing numbers of different data types: `f32`, `f64`, `i32` and `i64`.

The behavior of the widget can be configured through the use of several optional components:

- `HardLimit` specifies the minimum and maximum range for the value. If this component is absent,
  then the natural range of the data type is used.
- `SoftLimit` specifies the range that is accessible via dragging. Numbers that are entered by
  typing can exceed this limit.
- `NumberInputPrecision` is used to specify the number of decimal points of precision when dragging,
  so that you don't get a bunch of digits jumping around. This only quantizes the value when
  dragging, not when typing.
- `Step` is used to indicate the delta value when incrementing and decrementing.

When `SoftLimit` is present, the widget will look and feel like a slider: it will draw a slide
bar in the background, and the drag speed will be calculated such that changes in the bar's size
will be synchronized with movement of the mouse.

If `SoftLimit` is _not_ present, then the widget behaves more like a "scrubber", where there is
no slide bar, and drag speed is calculated based on a heuristic that takes into account precision,
step, and the current input value.

In either of this cases, a non-drag click event will activate "typing" mode, where a value can
be entered by typing digits.

Like all feathers widgets, this is a "controlled" widget, which means that the internal numeric
value is not automatically updated, but instead relies on the application's event handlers to
update the widget state in response to `ValueChange` events. Check out the `feathers_number_input`
example to see how to write such a handler trivially.

## Catching Panics

{{ heading_metadata(authors=["@SpecificProtagonist"] prs=[24240]) }}

For long-running programs, crashing can be unacceptable. If, for example, there is a bug in one of your image editor's tools, it's better for that tool to fail or to produce wrong results than to lose all your unsaved work.

Bevy's systems, commands and observers are able to return errors. You can either set an error handler case-by-case, or let the `FallbackErrorHandler` deal with it. But this used to only work for explicitly returned errors: Panics used to bring down the entire app.

In Bevy 0.xx, these panics now get turned into errors and passed to the fallback error handler. By default this re-panics, but now you can choose whether to log an error and continue, or whatever else you want.

## CompressedImageSaver Improvements

{{ heading_metadata(authors=["@JMS55", "@cwfitzgerald"] prs=[24223]) }}

Bevy's `CompressedImageSaver` asset processor has been significantly upgraded with a new compression backend powered by the [`ctt`](https://github.com/cwfitzgerald/ctt) library.

The new `compressed_image_saver` feature compresses textures into BCn formats (for desktop GPUs) or ASTC formats (for mobile GPUs), producing higher-quality output than the previous Basis Universal approach. The compressor automatically selects the best output format based on the input texture's channel count and type — for example, single-channel textures get BC4, HDR textures get BC6H, and standard RGBA textures get BC7.

Try out the new `compressed_image_saver` example to see it in action.

### Automatic Mipmap Generation

No more manually generating mipmaps! The new backend automatically produces a full mip chain during compression. This means less aliasing when textures are viewed at a distance and better GPU cache utilization — all for free, just by running your textures through the asset processor.

### ASTC for Mobile

To target mobile GPUs, set the `BEVY_COMPRESSED_IMAGE_SAVER_ASTC` environment variable with your desired block size (e.g. `4x4`, `6x6`, `8x8`). Larger blocks give smaller files at the cost of quality. All 14 ASTC block sizes are supported.

### Basis Universal is Still Available

The previous Basis Universal compression behavior has been moved to the `compressed_image_saver_universal` feature. This remains the best choice for cross-platform distribution (including WebGPU), since UASTC can be transcoded at load time to whatever format the target GPU supports.

## Bevy Error Context Messages

{{ heading_metadata(authors=["@cookie1170"] prs=[24528]) }}

Similar to the popular `anyhow` crate, `BevyError` now provides an ergonomic way to attach extra context to an error using the `context` method,
which also allows creating a `Result<T, BevyError>` from an `Option<T>`.

This makes it easier to trace back errors with human-readable messages without looking at verbose backtraces.

```rs
fn fallible() -> Result<(), BevyError> {
    // This produces the error message `Failed to parse number: invalid digit found in string`
    let parsed: usize = "I am not a number"
        .parse()
        .context("Failed to parse number")?;

    Ok(())
}
```

`with_context` may be used to produce the error string with a closure instead.

If multiple `context`s were used on the same `BevyError`, they're enumerated below:

```rs
fn fallible() -> Result<Package, BevyError> {
    let path = "package.json";
    let package = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {path}"))?;

    serde_json::parse(&package)?
}

fn uses_fallible() -> Result<(), BevyError> {
    let package = fallible().context("Failed to parse package.json")?;
    // Use `package`...
}
```

Will produce the following error if `package.json` is missing:

```rs
Failed to parse package.json

Caused by:
    Failed to read package.json
    No such file or directory (os error 2)
```

## InlineBox and InlineImage

{{ heading_metadata(authors=["@Ickshonpe"] prs=[25710]) }}

`InlineBox` is a new component added to `bevy_text` that allows space to be reserved within text layouts for custom content. Like `TextSpan`, an `InlineBox` entity is only valid when it's a descendant of a root `Text` or `Text2d` entity. `InlineBox` only reserves space, after layout `TextLayoutInfo::inline_boxes` contains the list of boxes and it's left to the user to draw its content. An inline box can be either `InFlow` or `OutOfFlow`. `InFlow` boxes takes up space and flows with the surrounding text. An `OutOfFlow` boxes is given a position as if it is zero-sized and it doesn't displace any text.

`InlineImage` is a new component added to `bevy_ui` that uses `InlineBox` to display images interspersed with text. It takes a color and an image asset handle, the size of its inline box is determined from the size of the image asset.

## What's Next?

No matter how many features we add, the flock will always demand *more*.
Game engines, unfortunately, are never *done*.

Returning by popular demand, let us peer deep into the mists of time,
and see what other features Bevy has in flight!
Like usual, many of these features are "essential components of a Bevy scene editor", even if they are not "the editor itself".
That allows us to ship useful bits and pieces incrementally,
and polish them while we put it all together.

- **X**: TODO

{{ support_bevy() }}

{{ contributors(version="0.20") }}

For those interested in a complete changelog, you can see the entire log (and linked pull requests) via the [relevant commit history](https://github.com/bevyengine/bevy/compare/v0.19.0...v0.20.0).

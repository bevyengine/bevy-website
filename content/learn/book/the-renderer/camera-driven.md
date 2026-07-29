+++
title = "Cameras"
insert_anchor_links = "right"
[extra]
weight = 3
+++

**Cameras** are the gate-keepers of Bevy's renderer.
If something isn't viewed by a [`Camera`], then the renderer won't try to display it.
You can think of a `Camera` as a specific view of what's happening in your game.
A camera could see over a character's shoulder, hover over top of the scene, or even be placed at eye level to show a first-person perspective.

Internally, cameras are used in several places throughout the rendering process.
As we saw in the [Pipelined Rendering page](/learn/book/the-renderer/render-pipelines), Bevy copies data from the Main `World` into a Render `World` before sending it off to the GPU for rendering.
Each `Camera` acts as filter for that data, letting the renderer know what data should be rendered and what data can be ignored based on what the `Camera` can see.

Cameras also dictate what render systems will run based on a [`CameraRenderGraph`] component.
This component holds a [`Schedule`], and systems placed in this `Schedule` will only run if a camera entity containing the specified `CameraRenderGraph` component exists in a scene.
Bevy provides the [`Core2d`] and [`Core3d`] schedules by default, which are enabled by adding either a [`Camera2d`] or [`Camera3d`] component to a camera entity.
Otherwise, you can create a custom render `Schedule` and provide it to a camera entity via `CameraRenderGraph`. 

[`Camera`]: https://docs.rs/bevy/latest/bevy/camera/struct.Camera.html
[`Schedule`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Schedule.html
[`CameraRenderGraph`]: https://docs.rs/bevy/latest/bevy/render/camera/struct.CameraRenderGraph.html
[`Core2d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/struct.Core2d.html
[`Core3d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/struct.Core3d.html
[`Camera2d`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Camera2d.html
[`Camera3d`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Camera3d.html

## The Camera Component

Bevy provides a [`Camera`] component that can be added to an entity.
Each `Camera` struct contains several fields for adjusting the camera's properties, like sub views, ordering, and even a toggle for enabling or disabling the camera.

```rust
// A basic, default camera entity.
commands.spawn(Camera::default());
```

However, a `Camera` component doesn't contain enough information on it's own.
Like we mentioned above, a [`CameraRenderGraph`] component also needs to be present in order for the camera to actually produce a rendered image.
If you forget to include a `CameraRenderGraph` component, your game will emit an `error` at runtime.

```rust
// Create a new camera entity that uses the `Core3D` render graph.
commands.spawn((
   Camera::default(),
   CameraRenderGraph::set(Core3d), 
));
```

Alternatively, if you know that you just want a basic camera, you can use the [`Camera3d`] (or [`Camera2d`] for 2D cameras) component which will do all of this for you.

```rust
// Create a new 2D camera.
commands.spawn(Camera2d::default());

// Create a new 3D camera.
commands.spawn(Camera3d::default());
```

Cameras also need to have a location in the `World`, which we can add with a [`Transform`] component.
If you don't specify this, Bevy will automatically insert a `Transform` component, placing the camera entity at `0, 0, 0` in the `World` with no rotations.

```rust
// Create a new 3D camera entity at location `0.0, 0.0, 5.0`.
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 0.0, 5.0),
));
```

[`Transform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html

### Camera Projections

Once you've set up the basic `Camera`, you might want to start adjusting how the rendered image is shaped.
A fundamental part of the rendered image's shape is the cameras [`Projection`] type.
A `Projection` describes how objects should be translated from the virtual scene into a flattened image.
Specifically, each projection is a 4x4 matrix that transforms points from world space (what the `Camera` is looking at) into screen space (the flattened image that the `Camera` renders out).

{% callout(type="info") %}
#### Frustum Culling

Remember how we said that each `Camera` helps filter the data that is copied into the Render `World`?
Bevy performs [frustum culling](https://en.wikipedia.org/wiki/Hidden-surface_determination#Viewing-frustum_culling) on the copied data to see what is actually visible to each `Camera`.
The "frustum" is derived from the location (`Transform`) and `Projection` of each `Camera`.

Data that is located within that frustum is rendered, while data that isn't is ignored.
This saves resources, since if something isn't being seen by a `Camera`, is there really a point in expending the resources to render it?

{% end %}

The [`Projection`] component is an enum with `Perspective`, `Orthographic`, or `Custom` variants.
Each variant is a wrapper containing a struct that houses the actual values each projection uses, [`PerspectiveProjection`], [`OrthographicProjection`], or [`CustomProjection`], respectively.
We'll only cover `Perspective` and `Orthographic` projections on this page, but you can find an example for a `CustomProjection` in the [Bevy Engine examples Camera folder](https://github.com/bevyengine/bevy/blob/main/examples/camera/custom_projection.rs).

```rust
// Create a new 3D camera with a Perspective projection.
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 0.0, 5.0),
    Projection::Perspective(
        PerspectiveProjection::default(),
    ),
))
```

[`PerspectiveProjection`] is the default view for 3D games, making distant objects smaller than closer objects when converted into the clip space.
You can alter the values in fields like `fov` (field of view), `near` (near clipping plane), and `far` (far clipping plane), among others, to control the projection's behavior.

```rust
// Create a new 3D camera with a custom Perspective projection.
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 0.0, 5.0),
    Projection::Perspective(
        PerspectiveProjection{
            fov: 70.0,
            near: 0.1,
            far: 1000.0,
            ..default()
        },
    ),
))
```

[`OrthographicProjection`] is the default for 2D games, where objects will stay a consistent size regardless of how far away from the camera they are.
Like with `PerspectiveProjection`, the `near` and `far` fields adjust the distances that objects will be rendered within.
However, because we're using `OrthographicProjection`, we don't have fields like `fov` to alter.
Instead, fields like `viewport_origin` (the center of the `RenderTarget`), `scale` (to control the size of objects in view), and `scaling_mode` (how the projection will scale) are used to control how the 2D image is shaped.

```rust
// Create a new 2D camera with a custom Orthographic projection.
commands.spawn((
    Camera2d::default(),
    Transform::from_xyz(0.0, 0.0, 5.0),
    Projection::Orthographic(
        OrthographicProjection{
            viewport_origin: Vec2(0.5, 0.5),
            scaling_mode: ScalingMode::Fixed {
                width: 200.0,
                height: 100.0,
            },
            scale: 1.0,
            near: 0.1,
            far: 1000.0,
            ..default()
        },
    ),
))
```

[`Projection`]: https://docs.rs/bevy/latest/bevy/prelude/enum.Projection.html
[`OrthographicProjection`]: https://docs.rs/bevy/latest/bevy/prelude/struct.OrthographicProjection.html
[`PerspectiveProjection`]: https://docs.rs/bevy/latest/bevy/prelude/struct.PerspectiveProjection.html
[`CustomProjection`]: https://docs.rs/bevy/latest/bevy/camera/struct.CustomProjection.html


## Using Multiple Cameras

At first, it might seem counterintuitive to use multiple cameras at the same time.
The player is only going to be looking at one thing at a time, so why use multiple cameras?
Well, using multiple cameras allows you to render out different views of your `World`.
It can be incredibly handy to switch between several views (or just display them at the same time) rather than needing to physically move the camera between locations.

Take local cooperative games for example.
Unless you're confining players to a small area that one camera can adequately cover, you might want to give each player their own camera.
Each player can then see their own view without being impeded by other players' views.

Or maybe you want the player to be able to see a live video feed of an area they aren't in or can't see.
You can place a camera looking at the area you want to display, and then render out that camera's view to an `Image` that you can show to the player.

These scenarios are possible because every `Camera` is independent.
Altering the properties or settings of one `Camera` does not affect other `Camera`s that might be in the `World`.
You can have one `Camera` using a `PerspectiveProjection` that represents the player's point of view, while another `Camera` uses `OrthographicProjection` to show a map or overlay image.

Using multiple `Camera`s isn't computationally free though.
Every `Camera` you place in your `World` will run an instance of the assigned `CameraRenderGraph`.
For example, this means that every render system you place in the `Core3d` schedule will run for every `Camera` that has `Core3d` in it's `CameraRenderGraph` component.
Try to think carefully about what render systems you actually need each `Camera` to use, and avoid repeating multiple complex systems between `Camera`s whenever possible.

### Render Targets

We use a `Camera` to get a specific view of our `World`, but where does the rendered image go once it's been captured?
The [`RenderTarget`] is the destination that the camera view is rendered to.
`RenderTarget` can send the camera view to a handful of destinations, most commonly either a [`Window`] (that the player looks at on their screen) or an [`Image`] asset (that can be saved and stored or displayed back to the player).
For example, the [`RenderTarget::Image`] variant holds an [`ImageRenderTarget`] struct, which allows you to write a camera's view into an [`Image`] asset.

However, it's likely you'll use the [`RenderTarget::Window`] variant the most, as this renders the camera's view onto a player's screen via a [`Window`].
This is the default `RenderTarget` that all `Camera`s will have unless you explicitly state otherwise.

The default behavior of `RenderTarget::Window` is to render each camera view at the full size of the `Window`.
We aren't always forced to use the entire window though.
`Camera` structs have a `viewport` field, containing an `Option<Viewport>` value.
A [`Viewport`] struct defines the actual area within a `RenderTarget` that the `Camera` can render to.
You can create a `Viewport` struct and assign it to a `Camera` to only render out that camera's view on a specific area of the game `Window`.

This is how split-screen games can easily be made in Bevy.
Using two cameras, we assign each a [`Viewport`] that takes up half of the total [`Window`] size.
Additional cameras halve the `Viewport` size again, with four cameras each getting a quarter of the total `Window` size.

Rather than just laying them side by side, cameras can also be layered on top of each other.
The `order` field on a `Camera` struct determines what order the rendered images will be placed in.
`Camera`s with lower `order` numbers will be rendered first and placed behind `Camera`s with higher `order` numbers.

[`RenderTarget`]: https://docs.rs/bevy/latest/bevy/camera/enum.RenderTarget.html
[`Window`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Window.html
[`Image`]: https://docs.rs/bevy/latest/bevy/image/struct.Image.html
[`RenderTarget::Image`]: https://docs.rs/bevy/latest/bevy/camera/enum.RenderTarget.html#variant.Image
[`ImageRenderTarget`]: https://docs.rs/bevy/latest/bevy/camera/struct.ImageRenderTarget.html
[`RenderTarget::Window`]: https://docs.rs/bevy/latest/bevy/camera/enum.RenderTarget.html#variant.Window
[`Viewport`]: https://docs.rs/bevy/latest/bevy/camera/struct.Viewport.html

### Render Layers

Adding multiple cameras to your game can bring a host of benefits, but it can also introduce some challenges.
Two cameras might be pointed in the same direction, or view the same area, but you might not want them to view the same things.

For cases like this, we can use **render layers** to separate the entities that each `Camera` can see.
Inserting a [`RenderLayers`] component with a specified layer number into a camera entity means that the `Camera` can see any entity that is on that render layer.
We can do the same for a non-camera entity, adding a `RenderLayers` component with a specific number to ensure that it'll only be seen by camera entities that have a `RenderLayers` component with a matching layer number.

```rust
// All environment entities are on layer 1.
const ENVIRONMENT_LAYER: usize = 1;

// Create a new entity with a `RenderLayers` component on layer 1.
fn spawn_environment(mut commands: Commands) {
    commands.spawn((
        Transform::default(),
        RenderLayers::layer(ENVIRONMENT_LAYER),
        ...
    ));
}

fn spawn_player(mut commands: Commands) {
    // Entities without a specified `RenderLayers` component will
    // be placed on layer 0.
    commands.spawn((
        Transform::default(),
        ...
    ));
}
```

A render layer is just a `bitmask` value, which means we aren't limited to only a single render layer.
You can add a `Camera` or other entity to multiple render layers by using the [`RenderLayers::from_layers()`] method and passing in a reference to an array of `usize` values.
`RenderLayers` also provides a number of methods that correspond to different bitwise operations, allowing you to compare and contrast the render layers of different entities.

```rust
fn merge_layers(
    mut layer_query: Query<&mut RenderLayers>,
) {
    // Add all entities with a `RenderLayers` component to layer 3.
    layer_query.iter_mut().for_each(|mut layers| {
        layers.with(3);
    });
}
```

A common use case for [`RenderLayers`] is creating first person camera setups that let players select their field-of-view.
These scenarios let players decide if they want to see all of their surroundings or whether they just want to focus on what's in front of them.
What shouldn't be affected is the tool or weapon they have in their hands.
This is where separate render layers come in.

You can place the tool / weapon on one render layer with a fixed field-of-view value.
Then, the environment around the player is on a different render layer that is affected by the player's chosen field-of-view value.
This ensures that the player experiences the game how they want without sacrificing the look or readability of the item they're using.

An example of a first person camera setup using [`RenderLayers`] is available in the [Bevy Engine examples Camera folder](https://github.com/bevyengine/bevy/blob/main/examples/camera/first_person_view_model.rs).

[`RenderLayers`]: https://docs.rs/bevy/latest/bevy/camera/visibility/struct.RenderLayers.html
[`RenderLayers::from_layers()`]: https://docs.rs/bevy/latest/bevy/camera/visibility/struct.RenderLayers.html#method.from_layers

### ClearColor

Whenever a `Camera` renders out sequential images or frames, there is a very brief moment when the previous frame is cleared away, but the next frame hasn't finished being written to the `Viewport`.
What is shown during this brief moment?

If we left the previous frame up, it would cause visual distortions and choppiness since parts of the next frame would be overlaid on top of the previous frame.
We could just remove the previous frame and let the `Viewport` stay blank?
It would work, but if the frame rate ever dropped, players might be stuck looking at a partially rendered screen.

Instead, Bevy clears the `Viewport` of the previous frame and displays a single, uniform color.
You can think of it like a "background" color that sits behind the rendered frames.
We call this the [`ClearColor`], a [`Resource`] that stores a [`Color`] value that all cameras will default to.

However, each `Camera` is able to have a unique `ClearColor` if desired.
The `clear_color` field holds a [`ClearColorConfig`] enum, which allows you to pick either the default [`ClearColor`] resource, a custom [`Color`] value, or `None` if you want the camera to just draw on top of whatever is already in the `Viewport`.

You might be wondering why you would ever choose `None` if we just said that not clearing the `Viewport` would cause visual distortions?
It's because clearing the `Viewport` of a `Camera` that is rendered on top of another `Camera` would (briefly) create a box of `ClearColor` that obstructs the background `Camera`.
It might not be an issue if the `Viewport` is updated fast enough, but if the frame rate were ever to drop, players would potentially see a section of their screen covered by `ClearColor`.
Whereas, if `ClearColorConfig` is set to `None`, then no color is used and the `Viewport` remains blank after clearing the previous screen.

[`ClearColor`]: https://docs.rs/bevy/latest/bevy/camera/struct.ClearColor.html
[`Resource`]: https://docs.rs/bevy/latest/bevy/prelude/trait.Resource.html
[`Color`]: https://docs.rs/bevy/latest/bevy/prelude/enum.Color.html
[`ClearColorConfig`]: https://docs.rs/bevy/latest/bevy/prelude/enum.ClearColorConfig.html

## UI in Cameras

`Camera` independence extends to UI elements as well.
Each camera can have its own UI root, which will render according to the camera's settings.
If there is only one `Camera` in the `World`, then no configuration is required.
Simply spawn your `Camera2d` or `Camera3d` and your UI elements will be displayed within the camera's [`Viewport`].

If you start using multiple cameras, you can use the [`UiTargetCamera`] component to indicate a specific camera to render the UI to.
`UiTargetCamera` is a wrapper around an `Entity` id value for a `Camera` entity.
Add it to the entity containing the UI elements and your UI will be rendered to the `Camera`s [`RenderTarget`] while also respecting the `Camera`s `Viewport` and scale.

If you don't specify a [`UiTargetCamera`], then UI `Node`s are rendered to the default `Camera`, which is marked by the [`IsDefaultUiCamera`] marker component.
You can specify the default `Camera` by inserting the [`IsDefaultUiCamera`] marker component into a camera entity.

[`UiTargetCamera`]: https://docs.rs/bevy/latest/bevy/prelude/struct.UiTargetCamera.html
[`IsDefaultUiCamera`]: https://docs.rs/bevy/latest/bevy/prelude/struct.IsDefaultUiCamera.html

+++
title = "Cameras"
insert_anchor_links = "right"
[extra]
weight = 3
+++

Making games requires a lot of different parts: assets to display, mechanics to interact with, even UI and text to provide information.
You can put a lot of work into hand-crafting these things in order to get them looking just the way you want them to.
Yet, you may wind up wondering: how are all of these things actually displayed to the person playing the game?

The simple answer is that the player is looking through a **camera** that displays what's happening inside your game.
The long answer is that the player looks at their screen which displays an image taken from a camera placed in your game.
A camera could see over a character's shoulder, hover over top of the game world, or even be placed at eye level to show a first-person perspective.
Basically, a camera shows a specific view of what's happening in your game.

Bevy uses a [`Camera`] component to add in the basic camera functionality.
Each `Camera` is a struct containing several fields for altering the camera's view properties, like sub views, ordering, and even a toggle for enabling or disabling the camera.
However, a `Camera` component doesn't contain enough information on it's own.
We have to go one step further and specify whether we want a 2D camera ([`Camera2d`]), or a 3D camera ([`Camera3d`]).
Doing this tells the renderer how to output the camera's view, since 2D games usually look a bit differently than 3D games do.

```rust
// Create a new entity with a 2D camera.
commands.spawn(Camera2d::default());

// Create a new entity with a 3D camera.
commands.spawn(Camera3d::default());
```

You can think of the `Camera` components like a window that looks into your game's `World`. 
Much like how looking out of a real-world window will only display what is outside that specific window, a Bevy `Camera` will only display what's in front of it.
Unlike a real-world window though, we can alter what is seen through a `Camera` without the player having to physically move their screen.

We can do this because every `Camera` is affected by three important components:

1. A **render target** ([`RenderTarget`]), which is the target (usually an area of the player's screen) that the `Camera` is viewed through.
2. A **camera projection** ([`Projection`]), controlling how the game's visuals are transformed from the game world into the rendered image.
3. A **scene location** ([`Transform`]), which describes how the `Camera` is oriented and where it's located in the game's `World`.

```rust
fn spawn_3d_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        RenderTarget::Window(WindowRef::Primary),
        Projection::Perspective,
        Transform::from_xyz(0.0, 0.0, 5.0),
    ));
}
```

For example, when a player moves their mouse or moves a joystick, the `Camera` entity will usually move in response.
The player's screen ([`RenderTarget`]) and the camera properties ([`Projection`]) are unlikely to have changed, but the rotation or position ([`Transform`]) of the `Camera` in the game world likely has changed.
This winds up "moving" the screen from the player's perspective.

If you don't set any of these components with explicit values when creating the camera entity, Bevy will supply them with default values.
[`Transform`] will default to a location of `0, 0, 0` and no rotations, while [`RenderTarget`] will output a window to your computer's primary screen.
The default value for [`Projection`] varies depending on the camera type. 
`Camera2d` defaults to [`Projection::Orthographic`], while `Camera3d` uses [`Projection::Perspective`] by default.

We'll go into more detail about each projection type and the different render targets further down the page.

[`Camera`]: https://docs.rs/bevy/latest/bevy/camera/struct.Camera.html
[`Camera2d`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Camera2d.html
[`Camera3d`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Camera3d.html
[`RenderTarget`]: https://docs.rs/bevy/latest/bevy/camera/enum.RenderTarget.html
[`Projection`]: https://docs.rs/bevy/latest/bevy/prelude/enum.Projection.html
[`Transform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html
[`Projection::Orthographic`]: https://docs.rs/bevy/latest/bevy/camera/enum.Projection.html#variant.Orthographic
[`Projection::Perspective`]: https://docs.rs/bevy/latest/bevy/camera/enum.Projection.html#variant.Perspective

## Camera Driven Rendering

Cameras are the gate-keepers of Bevy's renderer.
If something isn't viewed by a `Camera`, then the renderer won't try to display it.
That doesn't mean that it will be ignored though.
As we saw in the [Pipelined Rendering page](/learn/book/the-renderer/render-pipelines), Bevy creates the draw calls (the instructions for how to render the frame) in the "Render" step, which occurs after the data is copied into the Render `World` in the "Extract" step.
This is why the "Prepare & Queue" step occurs in-between "Extract" and "Render": we need each `Camera` to figure out what actually needs to be rendered.

Bevy performs [frustum culling](https://en.wikipedia.org/wiki/Hidden-surface_determination#Viewing-frustum_culling) on the supplied data to see what is actually visible to each `Camera`.
Draw calls are then dispatched for the entities that can be seen, while those that can't are skipped.
Every `Camera` is evaluated separately, meaning that one `Camera` isn't dependent on another. 
You can use multiple cameras in your game and not fear one influencing or disturbing the other.
If desired, you can fully turn a `Camera` off to prevent rendering what it sees, change individual `Camera` settings, or alter the order that each `Camera` is rendered in and only apply those changes to specific `Camera`s.

You can use this functionality to enable a large number of features, like enabling split-screen using two separate `Camera`s that each have a [`RenderTarget`] for one half of the display screen.
Or, you could create Portal-like objects by setting the `RenderTarget` of a `Camera` to a texture that is then displayed on an object in the world.

### 2D & 3D Cameras

Like we mentioned above, you have to specify whether you want a [`Camera2d`] or a [`Camera3d`] when creating a camera entity.
Adding this indicates that the camera entity should be included in the [rendering pipeline](/learn/book/the-renderer/render-pipelines) when a render `Schedule` runs.
Specifically, `Camera2d` includes a `Camera` in the [`Core2d`] render schedule, while `Camera3d` includes it in the [`Core3d`] schedule.
Forgetting to include one of these means that the renderer doesn't know to use that camera entity! 

Alternatively, you can specify a custom [`CameraRenderGraph`] if you have a custom render schedule label.

[`Core2d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/struct.Core2d.html
[`Core3d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/struct.Core3d.html

## Render Targets



## Projections

Since each `Camera` is independent of any others in a scene, it's important to know what type of image each `Camera` should display.
A fundamental part of this is a camera's [`Projection`].
A projection describes how objects in a game should be translated onto a screen.
Specifically, each projection is a 4x4 matrix that transforms points from view space (where the `Camera` is looking) into clip space (the screen, or portion of the screen, that is being viewed).
Objects that lie outside the bounds of the clip space are not rendered.

By default, a [`Projection`] in Bevy is an `enum` with either a `Perspective`, `Orthographic`, or `Custom` variant.
Each variant is a wrapper containing a struct that houses the actually values each projection uses, [`PerspectiveProjection`], [`OrthographicProjection`], or [`CustomProjection`], respectively.
`PerspectiveProjection` is most commonly used in 3D games, making distant objects smaller than closer objects when converted into the clip space.
`OrthographicProjection` is the default for 2D games, where objects will stay a consistent size regardless of how far back they are in a scene.

Both projection types have a number of fields that can alter the final look of the projection.
`near` and `far` are two fields common to both types, and describe the distances that objects won't be rendered at (objects beyond the `far` value won't be rendered, for example).
`PerspectiveProjection` contains fields such as `fov` (field of view) and `aspect_ratio` (width divided by height of the `RenderTarget` area).
Meanwhile, `OthographicProjection` contains fields like `viewport_origin` (the center of the `RenderTarget`) and `scale` (controls the size of objects in view).

You can also create a custom camera projection by implementing the [`CameraProjection`] trait on a struct and passing it into the [`Projection::custom()`] method.
`CameraProjection` requires implementing several methods, like [`CameraProjection::get_clip_from_view()`], [`CameraProjection::update()`], and [`CameraProjection::far()`] among others.
To see an example of this, we'd recommend checking out the [Custom Projection example](https://github.com/bevyengine/bevy/blob/main/examples/camera/custom_projection.rs) located in the Bevy Engine repository.

[`OrthographicProjection`]: https://docs.rs/bevy/latest/bevy/prelude/struct.PerspectiveProjection.html
[`PerspectiveProjection`]: https://docs.rs/bevy/latest/bevy/prelude/struct.PerspectiveProjection.html
[`CustomProjection`]: https://docs.rs/bevy/latest/bevy/camera/struct.CustomProjection.html
[`Projection::custom()]: https://docs.rs/bevy/latest/bevy/camera/enum.Projection.html#method.custom
[`CameraProjection::get_clip_from_view()`]: https://docs.rs/bevy/latest/bevy/camera/trait.CameraProjection.html#tymethod.get_clip_from_view
[`CameraProjection::update()`]: https://docs.rs/bevy/latest/bevy/camera/trait.CameraProjection.html#tymethod.update
[`CameraProjection::far()`]: https://docs.rs/bevy/latest/bevy/camera/trait.CameraProjection.html#tymethod.far

## Render Layers

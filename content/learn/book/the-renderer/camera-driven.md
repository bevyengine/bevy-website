+++
title = "Rendering To Cameras"
insert_anchor_links = "right"
[extra]
weight = 3
+++

Making games requires a lot of different parts: assets to display, mechanics to interact with, even UI and text to provide information.
You can put a lot of work into hand-crafting these things and get them to look just the way you want them.
Yet, you may wind up wondering: how are all of these things actually displayed to the person playing the game?

The simple answer is that the player is looking through a **Camera** that displays what's happening inside your game.
The long answer is that the player looks at their screen, which displays an image of your game that's taken from a camera placed in your game.
That camera shows a specific view of what's happening in your game.
We'll get into the intricacies of this later on, but for now all you have to know is that the easiest way to show the user something is to use a [`Camera`].

You can think of a [`Camera`] like a window that looks into your game's `World`. 
Much like how looking out of a real-world window will only display what is outside that specific window, a Bevy `Camera` will only display what's in front of it.
Unlike a real-world window though, we can alter what is seen through a `Camera` without the player having to physically move their screen.

We can do this because every `Camera` is affected by three important components:

1. A **render target** ([`RenderTarget`]), which is the target (usually an area of the player's screen) that the `Camera` is viewed through.
2. A **camera projection** ([`Projection`]), controlling how the game's visuals are transformed into the series of pixels on a player's screen.
3. A **scene location** ([`Transform`]), which describes how the `Camera` is oriented and where it's located in the game's `World`.

For example, when a player moves their mouse or presses a joystick in a direction, a `Camera` will usually move in response.
The screen (or area of the screen) that the player is looking at will stay the same, and most likely the camera projection will also stay the same.
However, the actual rotation (or position) of the `Camera` would have changed, which would display new objects that weren't able to be seen before.

Another example are scopes and zoom-ins.
When the player activates a scope, their screen (the render target) stays the same and they haven't actually moved (the scene location).
What is likely to have changed (depending on the setup) are the values affecting the `Camera` projection, enabling objects that were further away to now be shown closer from the player's perspective.

[`Camera`]: https://docs.rs/bevy/latest/bevy/camera/struct.Camera.html
[`RenderTarget`]: https://docs.rs/bevy/latest/bevy/camera/enum.RenderTarget.html
[`Projection`]: https://docs.rs/bevy/latest/bevy/prelude/enum.Projection.html
[`Transform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html

## Camera Driven Rendering

Now that we know what a `Camera` does in the context of a game, we can start to explore how it connects back to the rest of Bevy's rendering setup.

If you've read the [Pipelined Rendering page](/learn/book/the-renderer/render-pipelines), you'll know that the "Render" step is where Bevy actually creates the commands to display what is happening in your game.
You'll also know that we copy all the rendering-relevant Main `World` data to the Render `World` in the "Extract" step.
It's in the "Prepare & Queue" step that occurs in-between "Extract" and "Render" that a `Camera` factors in.

Bevy copies over Main `World` data that it's told can be rendered or used in rendering, like loaded assets or marked components.
It's unlikely that every piece of data is actually used every frame though.
To figure out what data is needed, Bevy evaluates what each `Camera` can actually see by performing frustum culling on each object.
Draw calls are then dispatched for the objects that can be seen, while those that can't are skipped.

This process repeats for every `Camera` that you place in a scene, which makes each `Camera` independent from the others.
You can use multiple cameras in your game and not fear one influencing or disturbing the other.
If desired, you can even fully turn a `Camera` on or off, change individual settings, or alter the order that each `Camera` is rendered in.

You can use this functionality to enable a large number of features, like enabling split-screen using two separate `Camera`s that each have a [`RenderTarget`] for one half of the display screen.
Or, you could create Portal-like objects by setting the `RenderTarget` of a `Camera` to a texture that is then displayed on an object in the world.

## Camera Projections

Since each `Camera` is independent of any others in a scene, it's important to know what type of image each `Camera` should display.
A fundamental part of this is a camera's [`Projection`].
A projection describes how objects in a game should be translated onto a screen.
Specifically, each projection is a 4x4 matrix that transforms points from view space (where the `Camera` is looking) into clip space (the screen, or portion of the screen, that is being viewed).
Objects that lie outside the bounds of the clip space are not rendered.

By default, a [`Projection`] in Bevy is an `enum` with either a `Perspective` ([`PerspectiveProjection`]), `Orthographic` ([`OrthographicProjection`]), or `Custom` ([`CustomProjection`]) variant.
`PerspectiveProjection` is most commonly used in 3d games, making distant objects smaller than closer objects when converted into the clip space.
`OrthographicProjection` is the default for 2d games, where objects will stay a consistent size regardless of how far back they are in a scene.

Both projection types have a number of fields that can alter the final look of the projection.
`near` and `far` are two fields common to both types, and describe the distances that objects won't be rendered at (objects beyond the `far` value won't be rendered, for example).
`PerspectiveProjection` contains fields such as `fov` (field of view) and `aspect_ratio` (width divided by height of the `RenderTarget` area).
Meanwhile, `OthographicProjection` contains fields like `viewport_origin` (the center of the `RenderTarget`) and `scale` (controls the size of objects in view).

You can also create a custom camera projection by implementing the [`CameraProjection`] trait on a struct and passing it into the [`Projection::custom()`] method.
`CameraProjection` requires implementing several methods, like [`CameraProjection::get_clip_from_view()`], [`CameraProjection::update()`], and [`CameraProjection::far()`], among others.
To see an example of this, we'd recommend checking out the [Custom Projection example](https://github.com/bevyengine/bevy/blob/main/examples/camera/custom_projection.rs) located in the Bevy Engine repository.

### 2D & 3D Cameras



## Camera Render Layers

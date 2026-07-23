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
What is likely to have changed (depending on the setup) are the values affecting the `Camera`'s projection, enabling objects that were further away to now be shown closer from the player's perspective.

[`Camera`]: https://docs.rs/bevy/latest/bevy/camera/struct.Camera.html
[`RenderTarget`]: https://docs.rs/bevy/latest/bevy/camera/enum.RenderTarget.html
[`Projection`]: https://docs.rs/bevy/latest/bevy/prelude/enum.Projection.html
[`Transform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html

## Camera Driven Rendering



## Camera Projections



### 2D & 3D Cameras



## Camera Render Layers

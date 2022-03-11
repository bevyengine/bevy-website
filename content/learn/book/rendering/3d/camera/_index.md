+++
title = "camera"
weight = 6
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

The first thing need you will need if you want to render anything is a camera.

At it's core a camera is simply a 2d projection of the 3d scene.

TODO: insert image showing a 3d scene projected on a 2d plane

TODO: Talk about view frustum <https://en.wikipedia.org/wiki/Viewing_frustum>
TODO: Talk about frustum culling

In bevy just like most things in the engine a `Camera` is just a component. To simplify spawning a `Camera` with all the required component we can use a `PerspectiveCameraBundle`. This bundle will spawn the camera and every other components required by bevy to render an image. If you prefer an orthographic projection you can use the `OrthographicCameraBundle`.

In the example at the beginning of the chapter we spawned a camera like this:

```rust
commands.spawn_bundle(PerspectiveCameraBundle {
    // This line spawns the camera at (-2.0, 2.5, 5.0) and rotates it
    // so that it looks at Vec3::ZERO, in other words it looks at the origin.
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
});
```

TODO: Cover the PerspectiveProjection options

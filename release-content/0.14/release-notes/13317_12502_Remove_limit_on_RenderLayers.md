<!-- #12502 Remove limit on RenderLayers. -->
<!-- https://github.com/bevyengine/bevy/pull/13317 -->

In Bevy you are able to assign entities spawned to different layers.

You can make 3D models, meshes, lights, etc. all have their own if you wish.

Then you can tell cameras which layers to render, and only objects which are part of those layers get rendered to that camera's target.

```rust
// This camera shows everything by default- membership in layer 0 is implicit
commands.spawn(Camera3dBundle::default());

const DEBUG_EXTRAS: usize = 100;

// This camera shows the same as above but also entities that are part of the `DEBUG_EXTRAS` layer
commands.spawn((
    Camera3dBundle::default(),
    RenderLayers::default().with(DEBUG_EXTRAS),
));
```

But this is old news, you've already been able to do this! What's new?

There is no longer any practical limit to how many layers you can define!
Before Bevy 0.14 the membership was defined by a bitmask which had limited slots available.


```rust
let so_many_layers = RenderLayers::from_layers(&[0, DEBUG_EXTRAS, ..]); // To your heart's content
```

<!-- Retained `Gizmo`s -->
<!-- https://github.com/bevyengine/bevy/pull/15473 -->
In previous versions of Bevy, gizmos were always rendered in an "immediate mode" style: they were rendered for a single frame before disappearing. This is great for prototyping but also has a performance cost.

With retained gizmos, you can now spawn gizmos that persist, enabling higher performance! For a
static set of lines, we've measured a ~65-80x improvement in performance!

As an example, here's how to spawn a sphere that persists:

```rust
fn setup(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>
) {
    let mut gizmo = GizmoAsset::default();

    // A sphere made out of one million lines!
    gizmo
        .sphere(default(), 1., CRIMSON)
        .resolution(1_000_000 / 3);

    commands.spawn(Gizmo {
        handle: gizmo_assets.add(gizmo),
        ..default()
    });
}
```

The immediate mode `Gizmos` API is still there if you want it though. It is still a great choice for easy debugging.

<!-- Retained `Gizmo`s -->
<!-- https://github.com/bevyengine/bevy/pull/15473 -->

In previous versions of Bevy, gizmos were limited to rendering for a single frame. This meant each
frame, you would need to recreate the gizmo. This is great for prototyping but can have an effect on
performance.

With retained gizmos, you can now spawn gizmos that persist, enabling higher performance! For a
static set of lines, we've measured ~65-80x improvement in performance! This does not remove the old
API though - if you were using `Gizmos` before, you can continue to use it to draw gizmos each
frame.

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

Previous versions of Bevy supported drawing line gizmos:

```rust
fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos.line_2d(Vec2::ZERO, Vec2::splat(-80.), RED);
}
```

However the only way to customize gizmos was to change their color, which may be limiting for some use cases. Additionally, the meeting points of two lines in a line strip, their *joints*, had little gaps.

As of Bevy 0.14, you can change the style of the lines and their joints for each gizmo config group:

```rust
fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos.line_2d(Vec2::ZERO, Vec2::splat(-80.), RED);
}

fn setup(mut config_store: ResMut<GizmoConfigStore>) {
    // Get the config for you gizmo config group
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    // Set the line style and joints for this config group
    config.line_style = GizmoLineStyle::Dotted;
    config.line_joints = GizmoLineJoint::Bevel;
}
```

The new line styles can be used in both 2D and 3D and respect the `line_perspective` option of their config groups.

Available line styles are:

- `GizmoLineStyle::Dotted`: draws a dotted line with each dot being a square
- `GizmoLineStyle::Solid`: draws a solid line - this is the default behavior and the only one available before Bevy 0.14

![new gizmos line styles](gizmos_line_styles.jpg)

Similarly, the new line joints offer a variety of options:

- `GizmoLineJoint::Miter`, which extends both lines until they meet at a common miter point,
- `GizmoLineJoint::Round(resolution)`, which will approximate an arc filling the gap between the two lines. The `resolution` determines the amount of triangles used to approximate the geometry of the arc.
- `GizmoLineJoint::Bevel`, which connects the ends of the two joining lines with a straight segment, and
- `GizmoLineJoint::None`, which uses no joints and leaves small gaps - this is the default behavior and the only one available before Bevy 0.14.

![new gizmos line joints](gizmos_line_joints.jpg)

You can check out the [2D gizmos example](https://github.com/bevyengine/bevy/tree/v0.14.0/examples/gizmos/2d_gizmos.rs), which demonstrates the use of line styles and joints!

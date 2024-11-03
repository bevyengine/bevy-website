- In initialization of `OrthographicProjection`, change `..default()` to `..OrthographicProjection::default_2d()` or `..OrthographicProjection::default_3d()`

Example:

```diff
--- a/examples/3d/orthographic.rs
+++ b/examples/3d/orthographic.rs
@@ -20,7 +20,7 @@ fn setup(
         projection: OrthographicProjection {
             scale: 3.0,
             scaling_mode: ScalingMode::FixedVertical(2.0),
-            ..default()
+            ..OrthographicProjection::default_3d()
         }
         .into(),
         transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
```

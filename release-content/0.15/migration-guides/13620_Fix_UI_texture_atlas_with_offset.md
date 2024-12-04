```diff
let ui_node = ExtractedUiNode {
                    stack_index,
                    transform,
                    color,
                    rect,
                    image,
-                   atlas_size: Some(atlas_size * scale_factor),      
+                   atlas_scaling: Some(Vec2::splat(scale_factor)),
                    clip,
                    flip_x,
                    flip_y,
                    camera_entity,
                    border,
                    border_radius,
                    node_type,
                },
```

```diff
let computed_slices = ComputedTextureSlices {
    slices,
-    image_size,
}
```

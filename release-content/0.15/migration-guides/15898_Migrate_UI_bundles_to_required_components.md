- Replace all uses of `NodeBundle` with `Node`. e.g.

```diff
     commands
-        .spawn(NodeBundle {
-            style: Style {
+        .spawn((
+            Node::default(),
+            Style {
                 width: Val::Percent(100.),
                 align_items: AlignItems::Center,
                 justify_content: JustifyContent::Center,
                 ..default()
             },
-            ..default()
-        })
+        ))
```

- Replace all uses of `ButtonBundle` with `Button`. e.g.

```diff
                     .spawn((
-                        ButtonBundle {
-                            style: Style {
-                                width: Val::Px(w),
-                                height: Val::Px(h),
-                                // horizontally center child text
-                                justify_content: JustifyContent::Center,
-                                // vertically center child text
-                                align_items: AlignItems::Center,
-                                margin: UiRect::all(Val::Px(20.0)),
-                                ..default()
-                            },
-                            image: image.clone().into(),
+                        Button,
+                        Style {
+                            width: Val::Px(w),
+                            height: Val::Px(h),
+                            // horizontally center child text
+                            justify_content: JustifyContent::Center,
+                            // vertically center child text
+                            align_items: AlignItems::Center,
+                            margin: UiRect::all(Val::Px(20.0)),
                             ..default()
                         },
+                        UiImage::from(image.clone()),
                         ImageScaleMode::Sliced(slicer.clone()),
                     ))
```

- Replace all uses of `ImageBundle` with `UiImage`. e.g.

```diff
-    commands.spawn(ImageBundle {
-        image: UiImage {
+    commands.spawn((
+        UiImage {
             texture: metering_mask,
             ..default()
         },
-        style: Style {
+        Style {
             width: Val::Percent(100.0),
             height: Val::Percent(100.0),
             ..default()
         },
-        ..default()
-    });
+    ));
```

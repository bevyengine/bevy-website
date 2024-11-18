`NodeBundle` has been replaced with `Node` (and its associated required components).

Simultaneously, the fields and behavior of `Style` have been moved to `Node`, and the largely internal values previously stored there are now found on `ComputedNode`.

To migrate, first move any fields set on `Style` into `Node` and replace all `Style` component usage with `Node`.

Before:

```rust
commands.spawn((
    Node::default(),
    Style {
        width:  Val::Px(100.),
        ..default()
    },
));
```

After:

```rust
commands.spawn(Node {
    width:  Val::Px(100.),
    ..default()
});
```

For any usage of the “computed node properties” that used to live on `Node`, use `ComputedNode` instead:

Before:

```rust
fn system(nodes: Query<&Node>) {
    for node in &nodes {
        let computed_size = node.size();
    }
}
```

After:

```rust
fn system(computed_nodes: Query<&ComputedNode>) {
    for computed_node in &computed_nodes {
        let computed_size = computed_node.size();
    }
}
```

Then, replace all uses of `NodeBundle` with `Node`. e.g.

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

```diffMove any fields set on `Style` into `Node` and replace all `Style` component usage with `Node`.

Before:

```rust
commands.spawn((
    Node::default(),
    Style {
        width:  Val::Px(100.),
        ..default()
    },
));
```

After:

```rust
commands.spawn(Node {
    width:  Val::Px(100.),
    ..default()
});
```

For any usage of the “computed node properties” that used to live on `Node`, use `ComputedNode` instead:

Before:

```rust
fn system(nodes: Query<&Node>) {
    for node in &nodes {
        let computed_size = node.size();
    }
}
```

After:

```rust
fn system(computed_nodes: Query<&ComputedNode>) {
    for computed_node in &computed_nodes {
        let computed_size = computed_node.size();
    }
}
```
      ButtonBundle {
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

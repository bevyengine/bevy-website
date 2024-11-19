`NodeBundle` has been replaced with `Node` (and its associated required components).
Simultaneously, the fields and behavior of `Style` have been moved to `Node`, and the largely internal values previously stored there are now found on `ComputedNode`.

It will be easiest to migrate if you replace `Node` with `ComputedNode` first, then `Style` with `Node`, and finally `NodeBundle` with `Node`.

## `Node` -> `ComputedNode`

For any usage of the “computed node properties” that used to live on `Node`, use `ComputedNode` instead.
This is a trivial find-and-replace rename.

If you were ever explicitly adding `Node` (now `ComputedNode`) to your UI bundles, you can remove this, as it is now required by `Node` (previously `Style`).

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

## `Style` -> `Node`

All of the values of `Style` now live on `Node`. This is a find-and-replace rename.

Before:

```rust
Style {
    width:  Val::Px(100.),
    ..default()
}
```

After:

```rust
Node {
    width:  Val::Px(100.),
    ..default()
}
```

## `NodeBundle` -> `Node`

Finally, replace all uses of `NodeBundle` with `Node`.
All other components in `NodeBundle` are now added implicitly via required components.
Adding them to your bundles manually will overwrite the default values.

Before:

```rust
     commands
        .spawn(NodeBundle {
            style: Style {
                 width: Val::Percent(100.),
                 align_items: AlignItems::Center,
                 justify_content: JustifyContent::Center,
                 ..default()
             },
            ..default()
        });
```

After:

```rust
     commands
        .spawn(Node {
                 width: Val::Percent(100.),
                 align_items: AlignItems::Center,
                 justify_content: JustifyContent::Center,
                 ..default()
             });
```

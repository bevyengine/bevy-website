Move any fields set on `Style` into `Node` and replace all `Style` component usage with `Node`.

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

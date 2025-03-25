Bevy UI now supports multiple shadows per node. A new struct `ShadowStyle` is used to set the style for each shadow. And the `BoxShadow` component is changed to a tuple struct wrapping a vector containing a list of `ShadowStyle`s. To spawn a node with a single shadow you can use the `new` constructor function:

```rust
commands.spawn((
    Node::default(),
    BoxShadow::new(
        Color::BLACK.with_alpha(0.8),
        Val::Percent(offset.x),
        Val::Percent(offset.y),
        Val::Percent(spread),
        Val::Px(blur),
    )
));
```

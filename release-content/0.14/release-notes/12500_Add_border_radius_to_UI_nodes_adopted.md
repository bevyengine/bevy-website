Border radius for UI nodes has been a long-requested feature for Bevy. Now it's supported!

To apply border radius to a UI node, there is a new component [`BorderRadius`](https://dev-docs.bevyengine.org/bevy/prelude/struct.BorderRadius.html). The [`NodeBundle`](https://dev-docs.bevyengine.org/bevy/prelude/struct.NodeBundle.html) and [`ButtonBundle`](https://dev-docs.bevyengine.org/bevy/prelude/struct.ButtonBundle.html) bundles have a new field in place for this called `border_radius`. For example:

```rs
commands.spawn(NodeBundle {
    style: Style {
        width: Val::Px(50.0),
        height: Val::Px(50.0),
        // We need a border to round a border, after all!
        border: UiRect::all(Val::Px(5.0)),
        ..default()
    },
    border_color: BorderColor(Color::BLACK),
    // Apply the radius to all corners. 
    // Optionally, you could use `BorderRadius::all`.
    border_radius: BorderRadius {
        top_left: Val::Px(50.0),
        top_right: Val::Px(50.0),
        bottom_right: Val::Px(50.0),
        bottom_left: Val::Px(50.0),
    },
    ..default()
});
```

There's a [new example](https://github.com/bevyengine/bevy/blob/main/examples/ui/rounded_borders.rs) showcasing this new API, a screenshot of which can be seen below:

![`rounded_borders` example](./rounded_borders.png)

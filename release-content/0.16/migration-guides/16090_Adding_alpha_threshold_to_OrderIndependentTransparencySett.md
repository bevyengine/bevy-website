If you previously explicitly initialized OrderIndependentTransparencySettings with your own `layer_count`, you will now have to add either a `..default()` statement or an explicit `alpha_threshold` value:

```rust
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        OrderIndependentTransparencySettings {
            layer_count: 16,
            ..default()
        },
    ));
}
```

The new `Monitor` component simplifies the process of working with multi-monitor setups by providing easy access to monitor properties such as resolution, refresh rate, position, and scaling factor. This feature is especially useful for developers who need to spawn windows on specific displays, gather monitor details, or adjust their application based on available hardware. This is especially useful for creative setups like multi-projector installations or LED video walls, where precise control over display environments is critical.

`Monitor` can be queried for and used to spawn or resize existing Windows:

```rust
fn spawn_windows(
    mut commands: Commands,
    monitors: Query<(Entity, &Monitor)>,
) {
    for (entity, monitor) in monitors_added.iter() {
        commands
            .spawn((
                Window {
                    mode: WindowMode::Fullscreen(MonitorSelection::Entity(entity)),
                    position: WindowPosition::Centered(MonitorSelection::Entity(entity)),
                    ..default()
                },
            ));
    }
}
```

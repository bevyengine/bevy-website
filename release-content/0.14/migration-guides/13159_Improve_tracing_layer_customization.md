Bevy uses `tracing` to handle logging and spans through `LogPlugin`. This could be customized with the `update_subscriber` field, but it was highly restrictive. This has since been amended, replacing the `update_subscriber` field with the more flexible `custom_layer`. which returns a `Layer`.

```rust
// Before
fn update_subscriber(_app: &mut App, subscriber: BoxedSubscriber) -> BoxedSubscriber {
    Box::new(subscriber.with(CustomLayer))
}

App::new()
    .add_plugins(LogPlugin {
        update_subscriber: Some(update_subscriber),
        ..default()
    })
    .run();

// After
use bevy::log::tracing_subscriber;

fn custom_layer(_app: &mut App) -> Option<BoxedLayer> {
    // You can provide a single layer:
    return Some(CustomLayer.boxed());

    // Or you can provide multiple layers, since `Vec<Layer>` also implements `Layer`:
    Some(Box::new(vec![
        tracing_subscriber::fmt::layer()
            .with_file(true)
            .boxed(),
        CustomLayer.boxed(),
    ]))
}

App::new()
    .add_plugins(LogPlugin {
        custom_layer,
        ..default()
    })
    .run();
```

The `BoxedSubscriber` type alias has also been removed, it was replaced by the `BoxedLayer` type alias.

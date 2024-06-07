The `LogPlugin`’s `update_subscriber` field has been replaced with the more flexible `custom_layer`.

```rust
// in 0.13
fn update_subscriber(_: &mut App, subscriber: BoxedSubscriber) -> BoxedSubscriber {
    Box::new(subscriber.with(CustomLayer))
}

LogPlugin {
    update_subscriber: Some(update_subscriber),
    ..default()
}

// in 0.14
fn custom_layer(_app: &mut App) -> Option<BoxedLayer> {
    // You can provide multiple layers like this, since Vec<Layer> is also a layer:
    Some(Box::new(vec![
        bevy::log::tracing_subscriber::fmt::layer()
            .with_file(true)
            .boxed(),
        CustomLayer.boxed(),
    ]))
}
LogPlugin {
    custom_layer,
    ..default()
}
```

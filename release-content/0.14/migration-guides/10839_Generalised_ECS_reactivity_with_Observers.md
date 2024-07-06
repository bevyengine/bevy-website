In 0.14, ECS observers were introduced: mechanisms for immediately responding to events in the world. As part of this change, the `Event` trait was extended to require `Component`. `#[derive(Event)]` now automatically implements `Component` for the annotated type, which can break types that also `#[derive(Component)]`.

```rust
// 0.13
#[derive(Event, Component)]
struct MyEvent;

// 0.14
// `Component` is still implemented by the `Event` derive.
#[derive(Event)]
struct MyEvent;
```

For more information, see the [release notes](/news/bevy-0-14/#ecs-hooks-and-observers) on hooks and observers.

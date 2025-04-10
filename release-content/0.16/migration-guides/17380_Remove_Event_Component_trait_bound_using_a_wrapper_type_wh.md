In 0.15 the `Event` trait required the `Component` trait. This bound has been removed, as it was deemed confusing for users (events aren't typically attached to entities or queried in systems).

If you require an event to implement `Component` (which usually isn't the case), you may manually derive it and update your trait bounds.

```rust
// 0.15
#[derive(Event)]
struct MyEvent;

fn handle_event_component<T: Event>(event_component: T) {
    // Access some `Component`-specific property of the event.
    let storage_type = T::STORAGE_TYPE;
}

// 0.16
#[derive(Event, Component)]
struct MyEvent;

fn handle_event_component<T: Event + Component>(event_component: T) {
    // Access some `Component`-specific property of the event.
    let storage_type = T::STORAGE_TYPE;
}
```

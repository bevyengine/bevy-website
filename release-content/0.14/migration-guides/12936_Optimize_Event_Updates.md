`Events::update()` has been optimized to be `O(1)` for the amount of events registered. In doing so, a few systems and run conditions have been changed.

Events are registered to a `World` using `EventRegistry` instead of the `Events` resource:

```rust
// 0.13
world.insert_resource(Events::<MyEvent>::default());

// 0.14
EventRegistry::register_event::<MyEvent>(&mut world);
```

A few systems and run conditions have been changed as well:

- `event_update_system` no longer uses generics and now has different arguments.
- `signal_event_update_system` now has different arguments.
- `reset_event_update_signal_system` has been removed.
- `event_update_condition` now has different arguments.

While not related to events, the `virtual_time_system` has been changed as well. It has been converted from a system to a regular function, and now takes `&T` and `&mut T` instead of `Res<T>` and `ResMut<T>`.

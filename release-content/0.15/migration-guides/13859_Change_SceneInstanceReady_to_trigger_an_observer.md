If you have a system which reads `SceneInstanceReady` events, it must be rewritten as an observer or entity observer.

```rust
// 0.14
fn ready_system(ready_events: EventReader<'_, '_, SceneInstanceReady>) {
    // ...
}

// 0.15
commands.observe(|trigger: Trigger<SceneInstanceReady>| {
    // ...
});
commands.entity(entity).observe(|trigger: Trigger<SceneInstanceReady>| {
    // ...
});
```

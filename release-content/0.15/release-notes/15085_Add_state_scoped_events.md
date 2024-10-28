<!-- Add state scoped events -->
<!-- https://github.com/bevyengine/bevy/pull/15085 -->

State scoped events will be automatically cleared when exiting a state. Useful when you want to guarantee clean state transitions.

Normal way to add an event.
```rust
fn setup(app: &mut App) {
    app.add_event::<MyGameEvent>();
}
```

**NEW**: Add a state-scoped event.
```rust
fn setup(app: &mut App) {
    app.add_state_scoped_event::<MyGameEvent>(GameState::Play);
}
```

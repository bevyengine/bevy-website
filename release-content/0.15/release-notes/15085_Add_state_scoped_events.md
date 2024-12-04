<!-- Add state scoped events -->
<!-- https://github.com/bevyengine/bevy/pull/15085 -->

State scoped events will be automatically cleared when exiting a state (similar to [StateScoped entities](https://bevyengine.org/news/bevy-0-14/#state-scoped-entities)). This is useful when you want to guarantee clean state transitions.

Normally, you would configure your event via:
```rust
fn setup(app: &mut App) {
    app.add_event::<MyGameEvent>();
}
```

If you want the events to be cleared when you exit a specific state, change this to:
```rust
fn setup(app: &mut App) {
    app.add_state_scoped_event::<MyGameEvent>(GameState::Play);
}
```

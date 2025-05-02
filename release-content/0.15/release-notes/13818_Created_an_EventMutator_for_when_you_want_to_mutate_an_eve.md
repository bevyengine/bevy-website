<!-- Created an EventMutator for when you want to mutate an event before reading -->
<!-- https://github.com/bevyengine/bevy/pull/13818 -->

<!-- TODO -->

When working with complex event-driven logic, you may find that you want to conditionally modify events without changing their type or re-emitting them.
While this has always been possible, it was quite onerous:

```rust
// We need to manually track which events this system has read
// using a system-local `EventCursor`, previously called `ManualEventReader`.
fn mutate_events(mut events: ResMut<Events<MyEvent>>, mut local_cursor: Local<EventCursor<MyEvent>>){    
    for event in local_cursor.read_mut(&mut *events){
        event.some_mutation();
    }
}
```

Now, you can simply use the new [`EventMutator`] system param, which keeps track of this bookkeeping for you.

```rust
fn mutate_events(mut event_mutator: EventMutator<MyEvent>>){    
    for event in event_mutator.read(){
        event.some_mutation();
    }
}
```

[`EventMutator`]: https://docs.rs/bevy/0.15/bevy/ecs/event/struct.EventMutator.html
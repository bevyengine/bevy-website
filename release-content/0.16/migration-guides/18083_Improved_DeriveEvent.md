In **Bevy 0.16** you can now use `#[derive(Event)]` for more specialized implementations. 

```rust
// 0.15
struct MyEvent;

impl Event for MyEvent {
    const AUTO_PROPAGATE: bool = true;
    type Traversal = &'static ChildOf
}

// 0.16
#[derive(Event)]
#[event(traversal = &'static ChildOf, auto_propagate)]
struct MyEvent;
```
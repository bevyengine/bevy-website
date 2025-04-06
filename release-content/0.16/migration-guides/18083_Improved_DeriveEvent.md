
In Bevy 0.16 you can now use `#[derive(Event)]` for more specialized implementations. 

```rust
use bevy_ecs::prelude::*;

struct MyEvent;

// before
impl Event for MyEvent {
    const AUTO_PROPAGATE: bool = true;
    type Traversal = &'static ChildOf
}

// after
#[derive(Event)]
#[event(traversal = &'static ChildOf, auto_propagate)]
struct MyEvent;
```
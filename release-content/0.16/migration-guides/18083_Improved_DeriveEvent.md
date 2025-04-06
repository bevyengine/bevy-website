
In Bevy 0.16 you can now use `#[derive(Event)]` for more specialized implementations. 

```rust

// before
use bevy_ecs::prelude::*;

struct MyEvent;

impl Event for MyEvent {
    const AUTO_PROPAGATE: bool = true;
    type Traversal = &'static ChildOf
}

// after
use bevy_ecs::prelude::*;

#[derive(Event)]
#[event(traversal = &'static ChildOf, auto_propagate)]
struct MyEvent;
```
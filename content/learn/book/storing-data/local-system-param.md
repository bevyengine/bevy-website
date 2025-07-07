+++
title = "System-Local Data Storage"
insert_anchor_links = "right"
[extra]
weight = 99
status = 'hidden'
+++

There's one more place where the ECS can store data: on the systems themselves.
With the help of the [`Local<T>`] system parameter, you can cache data from one run of a system to the next.

In most cases, a [resource] is going to be more flexible, as you can read and mutate it from other systems.
However, from time to time, you might want to:

- store some system-specific state about the last value
- maintain a cache of intermediate results to improve efficiency of computation
- avoid allocating a new large data structure each frame

Bevy itself uses `Local` system params in two prominent places:

1. As part of the [`EventReader`] abstraction, keeping track of which events each system has read.
2. In run conditions like [`on_timer`], to track how much time has run.

## Starting values

`Local` system params are always initialized with a default value.
This value is set by either the [`FromWorld`] trait or the [`Default`] trait.
The [`FromWorld`] trait allows you to access arbitrary data from the world,
allowing more complex initialization.

If you can't figure out what a good default value might be, remember that [`Option<T>`]
implements `Default`, even if `T` does not!

```rust,hide_lines=1
# use bevy::prelude::*;

struct NoGoodDefaultValue(u8);

fn increment_local_system_data(mut local: Local<Option<NoGoodDefaultValue>>){
    if local.is_none() {
        *local = Some(NoGoodDefaultValue(0));
    }
    
    local.as_mut().unwrap().0 += 1;
}
```

[`Local<T>`]: https://docs.rs/bevy/0.16.0/bevy/ecs/system/struct.Local.html
[resource]: [./resources.md]
[`EventReader`]: https://docs.rs/bevy/latest/bevy/ecs/event/struct.EventReader.html
[`on_timer`]: https://docs.rs/bevy/latest/bevy/time/common_conditions/fn.on_timer.html

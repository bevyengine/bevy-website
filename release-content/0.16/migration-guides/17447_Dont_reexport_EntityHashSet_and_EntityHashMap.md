`EntityHashSet` and `EntityHashMap` are no longer re-exported in `bevy::ecs::entity`, although they are still accessible through the prelude. If you were directly importing `EntityHashSet` and `EntityHashMap`, please access them from `bevy::ecs::entity::hash_set` and `bevy::ecs::entity::hash_map`.

```rust
// 0.15
use bevy::ecs::entity::{EntityHashSet, EntityHashMap};

// 0.16
use bevy::ecs::entity::{hash_set::EntityHashSet, hash_map::EntityHashMap};
```

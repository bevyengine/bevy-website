If you need to access the underlying `Entity` for a one-shot system's `SystemId`, use the new `SystemId::entity()` method.

```rust
// 0.13
let system_id = world.register_system(my_system);
let entity = Entity::from(system_id);

// 0.14
let system_id = world.register_system(my_system);
let entity = system_id.entity();
```

```rust
let system_id = world.register_system(my_sys);

// old
let entity = Entity::from(system_id);

// new
let entity = system_id.entity();
```

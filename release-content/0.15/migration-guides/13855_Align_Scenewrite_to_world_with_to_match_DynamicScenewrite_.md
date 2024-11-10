`Scene::write_to_world_with` no longer returns an `InstanceInfo`. 

Before

```rust
scene.write_to_world_with(world, &registry)
```

After

```rust
let mut entity_map = EntityHashMap::default();
scene.write_to_world_with(world, &mut entity_map, &registry)
```

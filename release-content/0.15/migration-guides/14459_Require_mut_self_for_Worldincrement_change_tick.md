The method `World::increment_change_tick` now requires `&mut self` instead of `&self`. If you need to call this method but do not have mutable access to the world, consider using `world.as_unsafe_world_cell_readonly().increment_change_tick()`, which does the same thing, but is less efficient than the method on `World` due to requiring atomic synchronization.

```rust
fn my_system(world: &World) {
    // Before
    world.increment_change_tick();

    // After
    world.as_unsafe_world_cell_readonly().increment_change_tick();
}
```

The `EntityCommands::apply()` method now takes a `EntityWorldMut`, which is an optimized version of the previous `Entity` and `&mut World` pair. `EntityWorldMut` has several existing methods for working with entities, although you may use `EntityWorldMut::id()` to access the `Entity` and `EntityWorldMut::world_scope()` to access the `&mut World`.

```rust
struct MyCommand;

fn print_entity(In(entity): In<Entity>) {
    info!("Entity: {entity}");
}

// 0.15
impl EntityCommand for MyCommand {
    fn apply(self, entity: Entity, world: &mut World) {
        world
            .run_system_cached_with(print_entity, entity)
            .unwrap();
    }
}

// 0.16
impl EntityCommand for MyCommand {
    fn apply(self, entity_world: EntityWorldMut) {
        let entity = entity_world.id();

        entity_world.world_scope(move |world: &mut World| {
            world.run_system_cached_with(print_entity, entity).unwrap();
        });
    }
}
```

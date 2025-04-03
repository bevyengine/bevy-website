The `EntityCommands::apply` method now takes an `EntityWorldMut` argument, instead of a `Entity` and a `&mut World` argument.
This was done to improve our error handling and better encapsulate the effect of `EntityCommands`, which are focused on mutating a single entity during exclusive world access.

To access the entity affected, use `EntityWorldMut::id`. Before:

```rust
struct Foo;

impl EntityCommand for Foo {
    fn apply(self, entity: Entity, world: &mut World) {
        world
            .run_system_cached_with(print_entity, entity)
            .unwrap();
    }
}

fn print_entity(In(entity): In<Entity>) {
    info!("entity: {entity}");
}
```

After:

```rust
struct Foo;

impl EntityCommand for Foo {
    fn apply(self, entity_world: EntityWorldMut) {
        let entity = entity_world.id();
        entity_world
            .into_world_mut()
            .run_system_cached_with(print_entity, entity)
            .unwrap();
    }
}

fn print_entity(In(entity): In<Entity>) {
    info!("entity: {entity}");
}
```

While `EntityWorldMut` has most of the same methods as `&mut World`, you can transform it into `&mut World` by calling `EntityWorldMut::into_world_mut`.

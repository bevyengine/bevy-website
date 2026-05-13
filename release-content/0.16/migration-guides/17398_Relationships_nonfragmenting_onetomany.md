Entity relationships are now built-in to the ECS, providing significant performance and user-experience improvements. There are several changes you may need in order to update your existing code.

First, when adding children to an entity with `EntityCommands::with_children()`, the method now passes a `ChildSpawnerCommands` type to the closure instead of a `ChildBuilder`. `ChildSpawnerCommands` is slightly different from `ChildBuilder`, but is still able to accomplish the same things as before.

```rust
// 0.15
commands.spawn_empty().with_children(|builder: &mut ChildBuilder<'_>| {
    // Spawn a child of the parent entity;
    builder.spawn(MyComponent(255));

    // Get the `Entity` ID of the parent.
    let parent = builder.parent_entity();

    // Queue a new `Command` to be executed.
    builder.enqueue_command(MyCommand::new(parent));
});

// 0.16
commands.spawn_empty().with_children(|spawner: &mut ChildSpawnerCommands<'_>| {
    spawner.spawn(MyComponent(255));

    // `parent_entity()` is now `target_entity()`.
    let parent = spawner.target_entity();

    // You can now access the `Commands` struct directly, which you can then use to queue commands.
    spawner.commands().queue(my_command(parent));
});
```

Furthermore, the new relationship system encourages working with the relationship components (`ChildOf`, `Children`) directly. For example, setting the parent of an entity is as simple as inserting a `ChildOf` component:

```rust
// 0.15
commands.spawn_empty().set_parent(parent);

// 0.16
commands.spawn_empty().insert(ChildOf(parent));
```

Replacing the children of a parent now requires removing the `Children` component and re-adding children individually:

```rust
// 0.15
commands.entity(parent).replace_children(&[child1, child2]);

// 0.16
commands.entity(parent)
    .remove::<Children>()
    .add_children(&[child1, child2]);
```

Despawning has also been changed to remove the complexities of `despawn_recursive()` and `despawn_descendants()` from `EntityCommands`:

|Action|0.15|0.16|
|-|-|-|
|Despawn parent and children|`despawn_recursive()`|`despawn()`|
|Despawn children|`despawn_descendants()`|`despawn_related::<Children>()`|
|Despawn parent|`despawn()`|`remove::<Children>()`, then `despawn()`|

```rust
// 0.15
commands.entity(parent).despawn_recursive();
commands.entity(parent).despawn_descendants();
commands.entity(parent).despawn();

// 0.16
commands.entity(parent).despawn();
commands.entity(parent).despawn_related::<Children>();
commands.entity(parent).remove::<Children>().despawn();
```

Because relationships are now part of `bevy_ecs` itself, all methods from the previous `HierarchyQueryExt` extension trait are
now inherent methods on `Query`.
While these have mostly been migrated unchanged, `parent` is now `related` and `children` now `relationship_sources`,
as these methods work for any relationship, not just parent-child ones.

The following functions have been deprecated:

- `Commands::insert_or_spawn_batch()`
- `World::insert_or_spawn_batch()`
- `World::insert_or_spawn_batch_with_caller()`
- `Entities::alloc_at()`

These methods, when used incorrectly, can cause major performance problems and are generally viewed as anti-patterns and foot guns. These are planned to be removed altogether in 0.17.

Instead of the above functions, consider doing one of the following:

1. Use the new `Disabled` component. Instead of despawning entities, simply disable them until you need them again. You can even use `Commands::try_insert_batch()` and `EntityCommands::remove()` to adjust what components an entity has.
2. Instead of despawning and respawning entities with the same `Entity` ID, simply use `spawn_batch()` and update the IDs to the new values.

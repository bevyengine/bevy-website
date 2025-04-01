The following functions have been deprecated:

- `Commands::insert_or_spawn_batch`
- `World::insert_or_spawn_batch`
- `World::insert_or_spawn_batch_with_caller`

These functions, when used incorrectly, can cause major performance problems and are generally viewed as anti-patterns and foot guns. These are planned to be removed altogether in 0.17.

Instead of these functions consider doing one of the following:

Option A) Instead of despawing entities and re-spawning them at a particular id, insert the new `Disabled` component without despawning the entity, and use `try_insert_batch` or `insert_batch` and remove `Disabled` instead of re-spawning it.

Option B) Instead of giving special meaning to an entity id, simply use `spawn_batch` and ensure entity references are valid when despawning.

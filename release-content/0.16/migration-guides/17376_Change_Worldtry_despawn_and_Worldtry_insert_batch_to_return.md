`World::try_despawn()` now returns a `Result` rather than a `bool`. Additionally, `World::try_insert_batch()` and `World::try_insert_batch_if_new()` now return a `Result` instead of silently failing.

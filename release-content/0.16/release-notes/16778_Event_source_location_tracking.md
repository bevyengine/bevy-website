<!-- Better source location tracking -->

Having a unified data model allows introspection and debugging tools to work for the entire engine:
For example, last release's `track_change_detection` feature flag lets you
automatically track by which line of source code any component (or resource) was inserted/mutated.

Now it also tracks which code
- triggered a hook or observer: `HookContext.caller`, `Trigger::caller()`
- sent an event: `EventId.caller`
- spawned or despawned an entity (until the entity index is reused): `EntityRef::spawned_by()`, `Entities::entity_get_spawned_or_despawned_by()`

And as a side effect, this leads to nicer error messages in some cases:
`Entity 0v1 was despawned by src/main.rs:10:11`.
Having outgrown its old name, the feature flag is now called `track_location`.

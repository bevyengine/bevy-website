<!-- Better source location tracking -->

Bevy's unified data model allows introspection and debugging tools to work for the entire engine:
For example, last release's `track_change_detection` feature flag lets you
[automatically track](/news/bevy-0-15/#change-detection-source-location-tracking) which which line of source code inserted or mutated any component or resource.

Now it also tracks which code
- triggered a hook or observer: `HookContext.caller`, `Trigger::caller()`
- sent an event: `EventId.caller`
- spawned or despawned an entity (until the entity index is reused): `EntityRef::spawned_by()`, `Entities::entity_get_spawned_or_despawned_by()`

And as a side effect, this leads to nicer error messages in some cases, such as this nicely improved despawn message:

`Entity 0v1 was despawned by src/main.rs:10:11`

Having outgrown its old name, the feature flag is now called `track_location`.

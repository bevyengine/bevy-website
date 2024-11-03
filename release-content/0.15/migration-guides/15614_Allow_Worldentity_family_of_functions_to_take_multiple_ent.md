- `World::get_entity` now returns `Result<_, Entity>` instead of `Option<_>`.
  - Use `world.get_entity(..).ok()` to return to the previous behavior.

- `World::get_entity_mut` and `DeferredWorld::get_entity_mut` now return `Result<_, EntityFetchError>` instead of `Option<_>`.
  - Use `world.get_entity_mut(..).ok()` to return to the previous behavior.

- Type inference for `World::entity`, `World::entity_mut`, `World::get_entity`, `World::get_entity_mut`, `DeferredWorld::entity_mut`, and `DeferredWorld::get_entity_mut` has changed, and might now require the input argument’s type to be explicitly written when inside closures.
- The following functions have been deprecated, and should be replaced as such:
  - `World::many_entities` -> `World::entity::<[Entity; N]>`
  - `World::many_entities_mut` -> `World::entity_mut::<[Entity; N]>`
  - `World::get_many_entities` -> `World::get_entity::<[Entity; N]>`
  - `World::get_many_entities_dynamic` -> `World::get_entity::<&[Entity]>`
  - `World::get_many_entities_mut` -> `World::get_entity_mut::<[Entity; N]>`
    - The equivalent return type has changed from `Result<_, QueryEntityError>` to `Result<_, EntityFetchError>`

  - `World::get_many_entities_dynamic_mut` -> `World::get_entity_mut::<&[Entity]>`
    - The equivalent return type has changed from `Result<_, QueryEntityError>` to `Result<_, EntityFetchError>`

  - `World::get_many_entities_from_set_mut` -> `World::get_entity_mut::<&EntityHashSet>`
    - The equivalent return type has changed from `Result<Vec<EntityMut>, QueryEntityError>` to `Result<EntityHashMap<EntityMut>, EntityFetchError>`. If necessary, you can still convert the `EntityHashMap` into a `Vec`.

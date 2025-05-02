The return types of several `World` and `UnsafeWorldCell` methods have been modified to return a `Result<T, EntityDoesNotExist>`.

- `World::inspect_entity()` now returns `Result<impl Iterator<Item = &ComponentInfo>, EntityDoesNotExistError>` instead of `impl Iterator<Item = &ComponentInfo>`. As such, this method no longer panics if the entity does not exist.
- `World::get_entity()` now returns `EntityDoesNotExistError` as an error instead of `Entity`. You can still access the entity's ID through `EntityDoesNotExistErrorentity::entity`, however.
- `UnsafeWorldCell::get_entity()` now returns `Result<UnsafeEntityCell, EntityDoesNotExistError>` instead of `Option<UnsafeEntityCell>`, giving you access to the entity's ID and other details on the error.

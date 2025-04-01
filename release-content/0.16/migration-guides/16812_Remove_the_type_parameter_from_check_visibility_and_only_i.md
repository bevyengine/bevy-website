`check_visibility` no longer takes a `QueryFilter`, and there’s no need to add it manually to your app schedule anymore for custom rendering items. Instead, entities with custom renderable components should add the appropriate type IDs to `VisibilityClass`. See `custom_phase_item` for an example.

Similarly, all methods on `VisibleEntities` (such as `get` and `iter`) no longer take a generic parameter, and instead must be passed a `TypeId` corresponding to the component used in the `VisibilityClass` of the entity.

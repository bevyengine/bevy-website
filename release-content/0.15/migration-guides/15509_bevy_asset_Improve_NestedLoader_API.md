Code which uses `bevy_asset`’s `LoadContext::loader` / `NestedLoader` will see some naming changes:

- `untyped` is replaced by `with_unknown_type`
- `with_asset_type` is replaced by `with_static_type`
- `with_asset_type_id` is replaced by `with_dynamic_type`
- `direct` is replaced by `immediate` (the opposite of “immediate” is “deferred”)

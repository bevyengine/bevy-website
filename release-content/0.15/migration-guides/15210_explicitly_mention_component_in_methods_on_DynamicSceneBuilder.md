`DynamicSceneBuilder::allow_all` and `deny_all` now set resource accesses, not just components. To return to the previous behavior, use the new `allow_all_components` or `deny_all_components` methods.

The following methods for `DynamicSceneBuilder` have been renamed:

- `with_filter` -> `with_component_filter`
- `allow` -> `allow_component`
- `deny` -> `deny_component`

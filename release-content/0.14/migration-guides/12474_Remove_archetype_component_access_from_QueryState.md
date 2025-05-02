`QueryState` no longer stores an `Access<ArchetypeComponentId>`, you must now pass it as an argument to each method that uses it. To account for this change:

- `QueryState::archetype_component_access` has been removed. You can work around this by accessing the surrounding `SystemState`s instead.
- `QueryState::new_archetype` and `QueryState::update_archetype_component_access` now require an `&mut Access<ArchetypeComponentId>` as a parameter.

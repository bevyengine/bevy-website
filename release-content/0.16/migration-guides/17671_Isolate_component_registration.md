In order to decouple `Storages` from `Components`, the following methods no longer take a `&mut Storages` argument:

- `Components::register_component()`
- `Components::register_component_with_descriptor()`
- `Bundle::register_required_components()`
- `Component::register_required_components()`

With this change, note that `SparseSets` will no longer be created when components are registered. Instead, they will only be constructed when those components are spawned.

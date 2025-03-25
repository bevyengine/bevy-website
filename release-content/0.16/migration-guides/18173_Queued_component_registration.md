Component registration can now be queued with only `&World`. To facilitate this, a few APIs needed to be moved around.

The following functions have moved from `Components` to `ComponentsRegistrator`:

- `register_component`
- `register_component_with_descriptor`
- `register_resource_with_descriptor`
- `register_non_send`
- `register_resource`
- `register_required_components_manual`

Accordingly, functions in `Bundle` and `Component` now take `ComponentsRegistrator` instead of `Components`.
You can obtain `ComponentsRegistrator` from the new `World::components_registrator`.
You can obtain `ComponentsQueuedRegistrator` from the new `World::components_queue`, and use it to stage component registration if desired.

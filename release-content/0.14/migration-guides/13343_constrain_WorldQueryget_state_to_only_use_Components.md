A few methods of `WorldQuery` and `QueryState` were unsound because they were passed an `&World`. They are now restricted to just take an `&Components`. The affected methods are:

- `WorldQuery::get_state()`
- `QueryState::transmute()`
- `QueryState::transmute_filtered()`
- `QueryState::join()`
- `QueryState::join_filtered()`

To access `Components` from a `World`, call `World::components()`.

If you manually implemented `WorldQuery`, you need to update `get_state()` to only use the information provided by `Components`.

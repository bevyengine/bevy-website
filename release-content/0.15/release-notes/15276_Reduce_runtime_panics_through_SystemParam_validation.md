**Bevy 0.15** comes with the much needed reduction of runtime panics when desired system parameters aren't available.
The new feature **fallible system parameters** allows parameters to safely fail fetching, which in turn will prevent systems from running.
Now, this does not affect majority of parameters, which are always available.

From the pre-existing `SystemParam`'s 
`Res<R>`, `ResMut<R>`, `NonSend<S>`, `NonSendMut<S>`

System parameters that build on top of other system parameters work on an **AND** basis.
`DynSystemParam` and `ParamSet` will fail if their respective internal params fail,
while tuples of params will fail if any of the internal params fail.

Based on this mechanic, a new set of parameters was crated:
- `Single<D, F>` - Works like `Query<D, F>::single`, fails if query contains 0 or more than 1 match,
- `Option<Single<D, F>>` - Works like `Query<D, F>::single`, fails if query contains more than 1 match,
- `Populated<D, F>` - Works like a `Query<D, F>`, fails if query contains no matches.

`fallible_params` example

example

Fallible params are still in development and their behavior in compound systems (like `system_a.pipe(system_b)`) is a subject to change.

If system fails to run because of missing parameters, the default behavior will be to emit a warning.
The warning will show up exactly once, on the first skip.
Warnings can be disabled on functional systems as following:
```rust
app.ass_systems(my_system.never_param_warn())
```
Currently warnings can either be emitted once or never, but we're looking into expanding this mechanic in subsequent updates!

<!-- Reduce runtime panics through `SystemParam` validation -->
<!-- https://github.com/bevyengine/bevy/pull/15276 -->

<!-- `QuerySingle` family of system params -->
<!-- https://github.com/bevyengine/bevy/pull/15476 -->

<!-- `Populated` (query) system param -->
<!-- https://github.com/bevyengine/bevy/pull/15488 -->


In Bevy 0.14 and prior, the following code would panic:
```rust
#[derive(Resource)]
struct MyResource;

fn my_system(my_resource: Res<MyResource>) {}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
    app.add_systems(my_system);
    // Panic here: `my_system` cannot fetch `MyResource`, because it was never added.
    app.run();
}
```
but in Bevy 0.15, `my_system` simply won't be executed.

This works for all system-based features:
- Systems & Observers - will be skipped,
- Run conditions - will be skipped and return `false`.

Compound systems, like `system_a.pipe(system_b)`, are currently skipped if any required data is missing.

Pre-existing parameters which now benefit from this feature are: `Res` and `ResMut` as well as their siblings `NonSend` and `NonSendMut`.
Parameters that build on top of other parameters: tuples, `DynSystemParam` and `ParamSet` are considered present if and only if all of their system parameters are present.

Additionally, few new system params were introduced to simplify existing code:
- `Single<D, F>` - Works like `Query<D, F>::single`, fails if query contains 0 or more than 1 match,
- `Option<Single<D, F>>` - Works like `Query<D, F>::single`, fails if query contains more than 1 match,
- `Populated<D, F>` - Works like a `Query<D, F>`, fails if query contains no matches.

## Warnings

Fallible system params come with a primitive warning mechanic.
Currently, systems can behave in one of two ways:
- (default) warn exactly once,
- never warn.

The default can be changed as following:
```rust
// For systems
app.add_systems(my_system.never_param_warn());
// For observers
app.add_observer(my_observer.never_param_warn());
// For run conditions
app.add_systems(my_system.run_if(my_condition.never_param_warn()));
```

Let us know what other warning strategies you'd like!

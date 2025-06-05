<!-- SystemParamBuilder - Support dynamic system parameters -->
<!-- https://github.com/bevyengine/bevy/pull/14817 -->

Bevy 0.14 introduced [the `SystemBuilder` type](https://bevy.org/news/bevy-0-14/#systembuilder) to allow systems to be created with dynamic queries.
In Bevy 0.15, this has been extended to many more types of system parameters!

The `SystemBuilder` type has been replaced with a `SystemParamBuilder<P>` trait to make it easier to compose builders.
Aggregates of parameters, including [tuples, `ParamSet`](https://github.com/bevyengine/bevy/pull/14050), [`Vec<T>`](https://github.com/bevyengine/bevy/pull/14821), and [custom parameters using `#[derive(SystemParam)]`](https://github.com/bevyengine/bevy/pull/14818), can now be used in dynamic systems.
For example, a `ParamSet<Vec<Query<FilteredEntityMut>>>` can be used to pass a variable number of dynamic queries that may conflict.

New [`FilteredResources` and `FilteredResourcesMut`](https://github.com/bevyengine/bevy/pull/15189) types can access a set of resources configured at runtime, similar to how the existing `FilteredEntityRef` and `FilteredEntityMut` access a set of components on one entity.

Finally, a new [`DynSystemParam`](https://github.com/bevyengine/bevy/pull/14817) type allows systems to use parameters of dynamic type and then downcast them.
This is especially useful for implementing part of a system with trait objects, where each trait implementation can use a different system parameter type.

Taken together, these can be used to build a system that runs a script defined at runtime, where the script needs a variable number of query and resource parameters.
Or, they can be used to build systems out of parts assembled at runtime!

```rust
fn buildable_system(
    query_a: Query<&A>,
    query_b: Query<&B>,
    queries_with_locals: Vec<(Query<FilteredEntityMut>, Local<usize>)>,
    mut dynamic_params: ParamSet<Vec<DynSystemParam>>,
    resources: FilteredResourcesMut,
) {
    // Parameters in a `ParamSet<Vec>` are accessed by index.
    let mut dyn_param_0: DynSystemParam = dynamic_params.get_mut(0);
    // Parameters in a `DynSystemParam` are accessed by downcasting to the original type.
    let param: Local<&str> = dyn_param_0.downcast_mut::<Local<&str>>().unwrap();
    // `FilteredResources` and `FilteredResourcesMut` have methods to get resources by type or by ID.
    let res: Ref<R> = resources.get::<R>().unwrap();
}

let param_builder = (
    // Parameters that don't need configuration can be built using `ParamBuilder` or its factory methods.
    ParamBuilder,
    ParamBuilder::query(),
    // A `Vec` of parameters can be built using a `Vec` of builders.
    vec![
        // A tuple of parameters can be built using a tuple of builders.
        (
            // Queries are built with a callback that supplies a `QueryBuilder` to configure the query.
            QueryParamBuilder::new(|builder| { builder.data::<&A>(); }),
            // Locals are built by passing the initial value for the local.
            LocalBuilder(123),
        ),
    ],
    // A `ParamSet` can be built for either a tuple or a `Vec`.
    ParamSetBuilder(vec![
        // A `DynSystemParam` is built using a builder for any type, and can be downcast to that type.
        DynParamBuilder::new(LocalBuilder("hello")),
        DynParamBuilder::new(ParamBuilder::resource::<R>()),
        // The type may be any system parameter, even a tuple or a `Vec`!
        DynParamBuilder::new((ParamBuilder::query::<&A>(), ParamBuilder::query::<&B>())),
    ]),
    // `FilteredResources` and `FilteredResourcesMut` are built with a callback
    // that supplies a builder to configure the resource access.
    FilteredResourcesMutParamBuilder::new(|builder| { builder.add_read::<R>(); }),
);

let system = param_builder
    .build_state(&mut world)
    .build_system(buildable_system);

// The built system is just like any other system, and can be added to a schedule.
schedule.add_systems(system);
```

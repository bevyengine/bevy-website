The API for `SystemBuilder` has changed.  Instead of constructing a builder with a world and then adding params, you first create a tuple of param builders and then supply the world.

```rust
// Before
let system = SystemBuilder::<()>::new(&mut world)
    .local::<u64>()
    .builder::<Local<u64>>(|x| *x = 10)
    .builder::<Query<&A>>(|builder| { builder.with::<B>(); })
    .build(system);

// After
let system = (
    ParamBuilder,
    LocalBuilder(10),
    QueryParamBuilder::new(|builder| { builder.with::<B>(); }),
)
    .build_state(&mut world)
    .build_system(system);
```

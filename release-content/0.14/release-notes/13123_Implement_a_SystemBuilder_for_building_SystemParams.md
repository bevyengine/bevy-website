Bevy users *love* systems, so we made a builder for their systems so they can build systems from within systems.
At runtime, using dynamically-defined component and resource types!

While you can use [`SystemBuilder`](https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.SystemBuilder.html) as an ergonomic alternative to the [`SystemState`](https://docs.rs/bevy/0.14/bevy/ecs/system/struct.SystemState.html) API for splitting the [`World`](https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.World.html) into disjoint borrows, its true values lies in its dynamic usage.

You can choose to create a different system based on runtime branches or, more intriguingly, the queries and so on can use runtime-defined component IDs.
This is another vital step towards creating an ergonomic and safe API to work with [dynamic queries](https://bevyengine.org/news/bevy-0-13/#dynamic-queries),
laying the groundwork for the devs who want to integrate scripting languages or bake in sophisticated modding support for their game.

```rust
// Start by creating builder from the world
let system = SystemBuilder::<()>::new(&mut world)
    // Various helper methods exist to add `SystemParam`.
    .resource::<R>()
    .query::<&A>()
    // Alternatively use `.param::<T>()` for any other `SystemParam` types.
    .param::<MyParam>()
    // Finish it all up with a call `.build`
    .build(my_system);
// The parameters the builder is initialized with will appear first in the arguments.
let system = SystemBuilder::<(Res<R>, Query<&A>)>::new(&mut world)
    .param::<MyParam>()
    .build(my_system);
// Parameters like `Query` that implement `BuildableSystemParam` can use
// `.builder::<T>()` to build in place.
let system = SystemBuilder::<()>::new(&mut world)
    .resource::<R>()
    // This turns our query into a `Query<&A, With<B>>`
    .builder::<Query<&A>>(|builder| { builder.with::<B>(); })
    .param::<MyParam>()
    .build(my_system);
world.run_system_once(system);
```

+++
title = "Systems"
insert_anchor_links = "right"
[extra]
weight = 0
status = 'hidden'
+++

**Systems** are the primary way that Bevy programs interact with the [`World`].
When you need to perform regularly scheduled logic, a [`System`] will usually be the best tool for the job.
As the third pillar of the ECS model, systems give us the power to operate over a large number of similar entities in an efficient way.
Since components of a given type are laid out beside each other in memory, we get good [cache locality] when operating on them in a batched fashion.

Systems in Bevy are any Rust function where every argument implements the [`SystemParam`].
Some common parameters you'll use and see in Bevy are [`Query`], [`Res`], [`ResMut`], [`MessageReader`], [`MessageWriter`], [`Local`] and [`Commands`].
Each of these is covered in their own chapters throughout the book, but we'll be using them here to help explain the concepts that directly tie in to systems.

```rust
// A simple system accessing the Commands, Query, and Resource parameters.
fn bevy_system(
    // Our System's System Parameters.
    mut commands: Commands,
    query: Query<&Component>,
    mut res: ResMut<Resource>
) {
    // Using our parameters in a System.
    for item in query.iter() {
        let new_entity = commands.spawn(BundleOfComponents).id();
        
        res.entity_field.push(new_entity);
    }
}
```

Bevy provides these and other parameters by default, but you can also create your own by implementing the [`SystemParam`] trait on a custom struct.
Note that the fields on a custom struct must also implement the `SystemParam`  trait, along a couple of other restrictions as well (check the `SystemParam` docs.rs page for more details).

```rust
#[derive(SystemParam)]
pub struct CustomParameter{
    field_1: Res<'w, SomeResource>,
    field_2: Query<'w, 's, &'static SomeComponent>,
}
```

[cache locality]: https://en.wikipedia.org/wiki/Locality_of_reference

[`World`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html
[`System`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.System.html
[`SystemParam`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html
[`Query`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Query.html
[`Res`]: https://docs.rs/bevy/latest/bevy/ecs/change_detection/struct.Res.html
[`ResMut`]: https://docs.rs/bevy/latest/bevy/ecs/change_detection/struct.ResMut.html
[`MessageReader`]: https://docs.rs/bevy/latest/bevy/ecs/message/struct.MessageReader.html
[`MessageWriter`]: https://docs.rs/bevy/latest/bevy/ecs/message/struct.MessageWriter.html
[`Local`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Local.html
[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Commands.html

## Accessing Data In Systems

One of the major benefits of Bevy's system abstraction is that it easily and efficiently ["splits the borrow"] of a [`World`],
allowing users to access non-overlapping parts of the world at the same time within each system,
and with the help of schedule executors like [`MultiThreadedExecutor`],
across multiple systems in parallel at the same time.

The requested data is automatically fetched from the [`World`] when the system is run,
locking out access to the underlying data to avoid violating the rules of the borrow checker.
When working within a system, or when running multiple systems at once, the prime directive of Rust applies:
access can be mutable or multiple: but never both at once.

In addition to the data retrieved from the `World`,
each system also has its own cached state.
This system state is used for performance optimizations (e.g. for queries), but can also be used to keep
track of private internal state or as scratch space.
See the section on [local system state] for more details.

["splits the borrow"]: https://doc.rust-lang.org/nomicon/borrow-splitting.html

[local system state]: ../storing-data/local-system-param

[`MultiThreadedExecutor`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.MultiThreadedExecutor.html

## Running Systems In Schedules

Normally, systems are inserted into a [`Schedule`] via [`App::add_system`].
Each of the standard schedules are evaluated once per frame, and systems within a single schedule run in parallel unless they are explicitly ordered relative to each other.

There's a great deal of complexity and nuance here: please see the [Game Loop chapter] for a more complete treatment of the topic.

[Game Loop chapter]: ../the-game-loop

[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html
[`App::add_system`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.add_systems

## One-shot Systems

Systems can also be run on demand, via a "one-shot" pattern.
This is an extremely flexible tool, allowing you to execute arbitrary logic on the world in an ergonomic way whenever you please.
One-shot systems are particularly useful for testing, handling callbacks in UI, creating scripted events or when architecting turn-based games.

As discussed above, systems have their own state.
When working with one-shot systems, entities are spawned to store this information.
These [`Entity`] identifier for these systems are stored in a [`SystemId`],
which are returned by [`World::register_system`]
and can then be used to run specific systems via [`World::run_system`].

In many cases though, simply passing in the function via
[`World::run_system_cached`] is more convenient,
causing it to be automatically cached and retrieved on the basis of its [`TypeId`].
However, this approach can be harder to abstract, and limits you to one copy of each system.
Any internal state (such as [locals] or [change detection] information) will be shared.

For convenience, [`Commands`] has a set of equivalent methods,
allowing you to queue up systems from within other systems.
This can recurse; use this power wisely!

Note that an entire [`Schedule`] can be run on demand in much the same way,
which can be valuable when trying to evaluate complex blocks of logic in response to specific triggers,
or at a rate other than "once per frame".

[locals]: ../storing-data/local-system-param
[change detection]: ../control-flow/change-detection

[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`SystemdId`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.SystemId.html
[`World::register_system`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html#method.register_system
[`World::run_system`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html#method.run_system
[`World::run_system_cached`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html#method.run_system_cached
[`TypeId`]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html

## Exclusive Systems

In addition to ordinary [`SystemParam`], `&mut World` can be used as the first argument of a system to create an **exclusive system**.
Exclusive systems:

- Can access any data in the world.
- Cannot be run in parallel with any other system.
- Do not use [`Commands`], including to spawn or despawn entities or to insert or remove components.

Exclusive systems can be useful for spawning large numbers of entities at once, as there is no additional overhead.
They are also extremely useful for unusually complex game logic or control flow, as any data, and can be used to immediately run one-shot systems or schedules.

That said, exclusive systems are harder to schedule:
both because they prevent any other system from running at the same time,
and because their [scheduling order is ambiguous with] any other systems in the same schedule.


## System Input and Output

Systems can optionally take input, and produce output, via the [`In`] and [`Out`] generics on the various system traits
such as [`System`].

The application of this is simplest to understand in the context of one-shot systems.
If we call [`World::run_system_once_with`] we can pass in an input value, and return an output value,
just like any other function.
The output type is always inferred, but when using system input the first parameter must be [`In<T>`],
where `T` is any type of input that you want to pass in.

System inputs and outputs can also be used in scheduled systems, via **system piping**.
Calling `App:add_systems(Update, first_system.pipe(second_system))` passes the output of the first system into the input of the first system,
and the combined system is treated as a single system from the perspective of the scheduler.
This can be repeated indefintely, but branching is not supported.

System piping is mostly useful for composing fragments of logic in a modular, reusable way.

System output is also used when returning errors from systems, as explained in the [Handling Errors] section of this chapter.

[Handling Errors]: ../control-flow/handling-errors

[`In`]: https://docs.rs/bevy/latest/bevy/prelude/trait.System.html#associatedtype.In
[`Out`]: https://docs.rs/bevy/latest/bevy/prelude/trait.System.html#associatedtype.Out
[`World::run_system_once_with`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html#method.run_system_once_with
[`In<T>`]: https://docs.rs/bevy/latest/bevy/prelude/struct.In.html

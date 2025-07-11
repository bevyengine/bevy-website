+++
title = "Systems"
insert_anchor_links = "right"
[extra]
weight = 0
status = 'hidden'
+++

Systems are the primary way that Bevy programs interact with the [`World`].
Systems are great for regularly scheduled logic,
especially when you want to operate over a large number of similar entities in an efficient way.

That high-performance access pattern is why ECS was originally invented:
because components of a given type are laid out beside each other in memory,
we get good [cache locality] when operating on them in a batched fashion.

[cache locality]: https://en.wikipedia.org/wiki/Locality_of_reference

## Accessing data in systems

One of the major benefits of Bevy's system abstraction is that it easily and efficiently ["splits the borrow"] of a [`World`],
allowing users to access non-overlapping parts of the world at the same time within each system,
and with the help of schedule executors like [`MultiThreadedExecutor`],
across multiple systems in parallel at the same time.

Systems in Bevy are any Rust function where every argument implements the [`SystemParam`].
The full list can be seen in the API docs linked (and you can create your own!),
but the most common are [`Query`], [`Res`], [`ResMut`], [`EventReader`], [`EventWriter`], [`Local`] and [`Commands`].

The requested data is automatically fetched from the [`World`] when the system is run,
locking out access to the underlying data to avoid violating the rules of the borrow checker.
When working within a system, or when running multiple systems at once, the prime directive of Rust applies:
access can be mutable or multiple: but never both at once.

In addition to the data retrieved from the `World`,
each system also has its own cached state.
This system state is used for performance optimizations (e.g. for queries), but can also be used to keep
track of private internal state or as scratch space.
See the section on [`Local`] for more details.

["splits the borrow"]: https://doc.rust-lang.org/nomicon/borrow-splitting.html

## Running systems in schedules

Normally, systems are inserted into a [`Schedule`] via [`App::add_system`].
Each of the standard schedules are evaluated once per frame, and systems within a single schedule run in parallel unless they are explicitly ordered relative to each other.

There's a great deal of complexity and nuance here: please see the [Game Loop chapter] for a more complete treatment of the topic.

## One-shot systems

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

Note that entire [`Schedule`]s can be run on demand in much the same way,
which can be valuable when trying to evaluate complex blocks of logic in response to specific triggers,
or at a rate other than "once per frame".

## Exclusive systems

In addition to ordinary [`SystemParam`], [`&mut World`] can be used as the first argument of a system to create an **exclusive system**.
Exclusive systems:

- can access any data in the world
- cannot be run in parallel with any other system
- do not use [commands], including to spawn or despawn entities or to insert or remove components

Exclusive systems can be useful for spawning large numbers of entities at once,
as there is no additional overhead
They are also extremely useful for unusually complex game logic or control flow,
as any data, and can be used to immediately run one-shot systems or schedules.

That said, exclusive systems are harder to schedule:
both because they prevent any other system from running at the same time,
and because their [scheduling order is ambiguous with] any other systems in the same schedule.

## System input and output

Systems can optionally take input, and produce output, via the `In` and `Out` generics on the various system traits
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

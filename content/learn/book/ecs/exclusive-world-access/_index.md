+++
title = "Exclusive `World` access"
weight = 8
template = "book-section.html"
page_template = "book-section.html"
+++

In various places, Bevy code works directly with the `World` data, gaining exclusive, blocking access to it and allowing for arbitrary mutations.
This is very flexible, and can be essential, but prevents any other systems from running simultaneously and can often be harder to reason about and maintain.
As a result, you should use only exclusive `World` when you have to.
You might be working with the `World` if:

- you're running an exclusive system to access data in unusually broad ways (such as for saving the game or handling networking)
- you're writing a custom command, to execute logic at the end of the system
- you're initializing a resource using the `FromWorld` trait
- you're working with a `NonSend` resource that cannot be sent across threads
- you're setting up tests to be run in a headless fashion
- you're using `bevy_ecs` as a standalone crate

## Basic usage

Generally speaking, the [API](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html) of working with the `World` mirrors those elsewhere that you might be familiar with.

Like with `Commands`, you can call `spawn`, `spawn_batch` and `despawn` to add and remove entities, and `
Resources are simply accessed with [`get_resource::<R>](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.get_resource) and the mutable equivalent [`get_resource_mut::<R>`](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.get_resource)

Like with queries, you can call [`get::<C>`](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.get) and [`get_mut::<C>`](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.get_mut) to access component data of a particular sort on a given entity. 
If you want access to *all* of the data on an entity, use [`get_entity`](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.get_entity) and [`get_entity_mut](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.get_entity_mut) (or their faster but riskier siblings [entity](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.entity) and [`entity_mut`]), along with various [`EntityRef`](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.EntityRef.html) methods.

You can create new queries using [`query`](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.query) and [`query_filtered`](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.query_filtered), using the former when you only have one type parameter and the latter when you want to use the second filtering type parameter of standard queries as well.

TODO: add new methods for using system parameters with `World` access

## Accessing multiple parts of the `World` simultaneously

When working with non-trivial exclusive `World` logic, you're likely to run into cases where you need mutable access to more than one part of the `World` at once.
This tends to make the compiler quite unhappy, but you can use [`World::cell`](https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.bundles) and `World::resource_scope`(https://docs.rs/bevy/0.5.0/bevy/ecs/world/struct.World.html#method.resource_scope) to allow for carefully shared mutable access.

TODO: explain how `cell` and `resource_scope` works.
TODO: add `WorldCell` example

## Exclusive systems

Exclusive systems are systems that operate on `&mut World`.
Unlike ordinary systems, which can be executed in parallel in arbitrary orders, exclusive systems can run either:

1. Immediately before the start of a stage, using `my_exclusive_system().exclusive_system().at_start()`.
2. Immediately after the end of a stage, before commands are applied, using `my_exclusive_system().exclusive_system().before_commands()`.
3. Immediately after the end of a stage, after commands are applied, using `my_exclusive_system().exclusive_system().after_commands()`.

TODO: add exclusive system example
```rust

```

## Custom commands

Commands execute arbitrary logic at the end of the stage, queued up by ordinary systems.
You can extend the [`Command`](https://docs.rs/bevy/0.5.0/bevy/ecs/system/trait.Command.html) trait to create your own commands, performing tasks with far-reaching consequences without requiring access to that data in your originating systems.

Here's an example of how you might do so:

TODO: add custom commands example
```rust

```

Due to the delayed effect of commands, and their relatively poor performance (they can only be executed one at a time in sequence), you should only use custom commands for tasks that truly need their world-altering powers.
In many cases, an event plus an event-handling system will be faster, more ergonomic and easier to debug.

## `NonSend` resources

Non-send resources are resources that lack the `Send + Sync` trait bounds: they cannot be sent safely across threads.
`NonSend` resources will typically be quite advanced: used for things like networking or interfacing with external libraries.
They are accessed using `NonSend<R>` and `NonSendMut<R>` in a way that is directly analogous to `Res` and `ResMut`.
The only difference is that they can only be accessed when we have exclusive `World` access, as accessing them can only be done on the main thread.

Here's a simple example showing how to use an exclusive system to modify a `NonSend` resource.

TODO: add `NonSend` example
```rust

```

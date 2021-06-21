+++
title = "Manipulating entities with commands"
weight = 4
template = "book-section.html"
page_template = "book-section.html"
+++

**Commands** are powerful, unrestricted tools for modifying the `World` of your Bevy game.
Queue them up, and you can reshape the world in arbitrary ways when they're evaluated!

Commands are fundamentally designed to perform work that cannot be safely done in parallel, and are used to change the world in ways that touch large amounts of data at once (requiring exclusive access to the archetypes and other metadata of the `World`).
While you can check out the full list of options by reading the API docs for [`Commands`](https://docs.rs/bevy/0.5.0/bevy/ecs/system/struct.Commands.html) and [`EntityCommands`](https://docs.rs/bevy/0.5.0/bevy/ecs/system/struct.EntityCommands.html), commands are commonly used to:

- Spawn and despawn entities with `spawn`, `spawn_bundle` and ``commands.entity(my_entity).despawn`
- Add and remove resources dynamically with `init_resource`, `insert_resouce` and `remove_resource`
- Add and remove components to entities with `commands.entity(my_entity).insert` and `commands.entity(my_entity).remove`

Due to the limitations on the parallel execution of the operations performed using commands, **commands do not take effect immediately.**
This is, by and large, their defining characteristic: they must wait until the next **hard sync point** (where single function have access to the entire world in a sequential fashion) in order to be resolved as they require exclusive mutable access to the world.
Unless you are already operating in an exclusive way on the `World`, commands are typically going to be processed at the end of the current stage.

As a result, **avoid using commands unnecessarily**.
For example, you *could* use the overwriting behavior of component insertion to mutate components in place.
However, this is unclear, slow and takes delayed effect, making it strictly worse than just requesting the appropriate data and mutating it in an ordinary fashion.

## Manipulating entities with commands

Most of the time, you'll be using commands to modify entities.
Let's take a look at the details of that, beginning with the various ways to spawn and despawn entities.

```rust

```

Now, let's take a quick look at modifying the components of entities with commands.

```rust

```

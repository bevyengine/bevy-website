+++
title = "Commands queue up work"
weight = 5
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

**Commands** are powerful, unrestricted tools for deferring complex or heavily-blocking work.
They are commonly used to:

- Spawn and despawn entities
- Add and remove components to entities
- Work across entity boundaries in hierarchal ways
- Add and remove resources dynamically

Commands are fundamentally designed to perform work that cannot be safely done in parallel, and are used to change the world in ways that touch large amounts of data at once (requiring exclusive access to the [archetypes](../../game-logic/system-ordering/_index.md) and other metadata of the [`World`].
The [`Command`] trait used by each method on [`Commands`] takes in `&mut World` as the only argument to its [`write`] method, allowing commands to modify the entire world in arbitrary, sequential ways.

As a result, **commands do not take effect immediately.**
Instead, they must wait until the next **hard sync point** (where each step executed has  [exclusive access](../exclusive-world-access/_index.md) to the entire world in a sequential fashion) in order to be resolved as they require exclusive mutable access to the world.
In other word: commands are processed at the end of each stage.

You should avoid using commands unnecessarily.
For example, you *could* use the overwriting behavior of component insertion to mutate components in place.
However, this is unclear, relatively expensive and takes delayed effect: it is strictly worse than just requesting the appropriate data and mutating it.
If you wish to defer work but do not need to spawn entities, add/remove components or add/remove resources, you likely can (and should) just use [events](../../game-logic/events/_index.md) instead.

[`Command`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.Command.html
[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html

## Usage

As discussed in [entities have components](../entities-components/_index.md), commands are used by adding a [`Commands`]-type system parameter, then appending various tasks to its end through the provided methods.
For a basic overview of how they're used, please refer to that page.

For more detailed information on the available methods, check [`Commands`] and [`EntityCommands`], which operate on a single entity.

[`EntityCommands`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.EntityCommands.html

### Custom commands

You can extend the [`Command`] trait to create your own commands, performing tasks with far-reaching consequences without requiring access to that data in your originating systems.

Let's walk through the steps needed to create a new custom command:

```rust
// Usually, this setup will be handled automatically
// by your Bevy app, but you can perform it manually
let world = World::new();
let command_queue = CommandQueue::default()
let commands = Commands::new(command_queue, world);

// First, we need to create a struct that stores any data needed by the command
struct StoreMessage {
    // This functionality is not very useful,
    // and is included for teaching purposes
    message: String,
}

// Next, we need to implement the `Command` trait for that struct
// which describes how the command changes the world
impl Command for StoreMessage {
    // This method allows us to read from and modify
    // *any* part of the world that we want
    fn write(self, _world: &mut World) {
        // Here though, we're simply passing on the message stored
        info!(self.message);
    }
}

// We can add our new command directly to our Commands struct
// by inserting an instance of this new type
commands.add(StoreMessage{message: "Added directly!"});

// For better ergnonomics, we can instead choose to create an extension trait,
// which allows us to add new methods to commands
trait EntityCounting {
    fn store_message(&mut self, message: String);
}

// We implement our new extension trait for the `Commands` type,
// giving us access to a new method
impl<'a> EntityCounting for Commands<'a> {
    fn store_message(&mut self, message: String) {
        self.add(StoreMessage { String })
    }
}

// Now it's just another method!
commands.store_message("Now with trait extensions!");

// Finally, let's apply all of our commands to verify that this worked as we hoped
command_queue.apply(world);
```

Due to the delayed effect of commands, their excessive power, and their relatively poor performance (they can only be executed one at a time in sequence), you should only use custom commands for tasks that truly need their world-altering powers.
In many cases, an event-emitting system combined with an event-handling system will be faster, more ergonomic and easier to debug.

For more details on how to work with `&mut World` to perform logic in custom commands, see the page on [exclusive world access](../exclusive-world-access/_index.md).

### Manually flushing commands

Ordinarily, commands are only applied at the end of each stage.
This is because they require exclusive mutable access to the [`World`].

However, if you already have exclusive access to the [`World`], you can use [`SystemState::<Commands>::apply()`] to immediately run and clear any [`Commands`] that may have accumulated.

```rust
use bevy::prelude::*;
use bevy::ecs::system::SystemState;

let world = World::new();

let mut system_state = SystemState::<Commands>::new(&mut world);

// Manually adding a command to the list to verify that this works
let mut commands = system_state.get_mut();
commands.spawn();

// Applies all accumulated commands to the world, causing them to take immediate effect
system_state.apply(&mut world);
```

Generally speaking, this isn't useful for the average game: you can't get exclusive world access any faster than commands naturally apply.
However, this technique can be incredibly useful for advanced control flows that are willing to sacrifice some parallelism in order to immediately (or repeatedly) process commands.

[`SystemState::<Commands>::apply()`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.SystemState#method.apply

## Internal mechanics

Due to some internal special-casing to reduce blocking, it can be challenging to grasp how commands are actually implemented.
This section discusses a few of the details that can matter as an advanced end user.

### System parallelism and commands

Ordinarily, mutable access to data in Bevy requires exclusive access: preventing more than one system that use this data from running at the same time.

However, [`Commands`] are special-cased, due to their prevalence in a way that takes advantage of their append-only nature.

A fresh [`Commands`] struct is dispatched to each system and then each instance processed later according to the execution order of the originating systems.

### Application order

When combining the effects of multiple commands, it can be important to be aware of the exact order in which your commands are executed can be vitally important.
If one system is spawning an entity and then passing off its [`Entity`] identifier to another system, the entity must be spawned before the second system attempts to add components to it!

Thankfully, while their effect is delayed, their application order follows a few simple rules:

1. Commands are always applied one at a time.
2. All commands created by a given system are applied sequentially, in the order in which the methods were called on the [`Commands`] object in that system.
3. If a system is explicitly specified (using [`.before`] or [`.after`]) to occur before another system, its commands will always be applied first.
4. If two systems do not have an [explicit ordering](../../game-logic/system-ordering/_index.md) between them (including any transitive ordering from e.g. A before B before C), the order in which their commands are applied is unspecified, and may vary between runs of the app.

[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html

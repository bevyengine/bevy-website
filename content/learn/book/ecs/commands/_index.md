+++
title = "Commands queue up work"
weight = 5
template = "book-section.html"
page_template = "book-section.html"
+++

**Commands** are powerful, unrestricted tools for modifying the `World` of your Bevy game.
Queue them up, and you can reshape the world in arbitrary ways when they're evaluated!

Commands are fundamentally designed to perform work that cannot be safely done in parallel, and are used to change the world in ways that touch large amounts of data at once (requiring exclusive access to the archetypes and other metadata of the `World`).
While you can check out the full list of options by reading the API docs for [`Commands`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html) and [`EntityCommands`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.EntityCommands.html), commands are commonly used to:

- Spawn and despawn entities with `spawn`, `spawn_bundle` and `commands.entity(my_entity).despawn`
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

When you want to spawn large numbers of entities at once in an efficient way, use [`spawn_batch`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html#method.spawn_batch):

```rust
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera.system())
        .add_startup_system(spawn_lines.system())
        .run()
}

fn new_line(i: u8, material_handle: Handle<ColorMaterial>) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite::new(Vec2::new(10.0, 200.0)),
        transform: Transform::from_xyz(i as f32 * 50.0, 0.0, 1.0),
        material: material_handle,
        ..Default::default()
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn spawn_lines(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material_handle = materials.add(Color::PINK.into());

    // spawn_batch accepts any object which can be turned into an iterator
    // which returns a Bundle in each item
    // and creates one entity for each item in that iterator
    commands.spawn_batch((1..9).map(move |i| new_line(i, material_handle.clone())));
}
```

## Manually flushing commands

Ordinarily, commands are only applied at the end of each stage.
This is because they require exclusive mutable access to the `World`.

However, if you already have exclusive access to the `World`, you can use [`SystemState::<Commands>::apply()`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.SystemState#method.apply) to immediately run and clear any `Commands` that may have accumulated.

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

## Custom commands

You can extend the [`Command`](https://docs.rs/bevy/latest/bevy/ecs/system/trait.Command.html) trait to create your own commands, performing tasks with far-reaching consequences without requiring access to that data in your originating systems.

Let's walk through the steps needed to create a new custom command:

```rust
// First, we need to create a struct that stores any data needed by the command
struct CountEntities {
    // This functionality is not very useful,
    // and is included for teaching purposes
    message: String,
}

// Next, we need to implement the `Command` trait for that struct
// which describes how the command changes the world
impl Command for CountEntities {
    fn write(self, world: &mut World) {
        info!(
            "The world has {} entities in it. BTW, \"{}\"!",
            world.entities().len(),
            self.message.to_string(),
        );
    }
}
// Then, we create an extension trait, which allows us to add new methods to commands
trait EntityCounting {
    fn count_entities(&mut self, message: String);
}

// Finally, we implement that trait for the `Commands` type,
// adding our new command type to the command queue
impl<'a> EntityCounting for Commands<'a> {
    // We can now call `commands.count_entities(offset)`
    // on any instance of `Commands` when we have
    // the `EntityCounting` trait in scope
    fn count_entities(&mut self, message: String) {
        self.add(CountEntities { String })
    }
}
```

Due to the delayed effect of commands, and their relatively poor performance (they can only be executed one at a time in sequence), you should only use custom commands for tasks that truly need their world-altering powers.
In many cases, an event plus an event-handling system will be faster, more ergonomic and easier to debug.

For more details on how to perform logic in custom commands, see the page on [exclusive world access](../exclusive-world-access/_index.md).

## Application order of commands

When combining the effects of multiple commands, it can be important to be aware of the exact order in which your commands are executed can be vitally important.
If one system is spawning an entity and then passing off its `Entity` identifier to another system, that needs to occur before the second system attempts to add components to it!

Thankfully, while their effect is delayed, their application order follows a few simple rules:

1. Commands are always applied one at a time.
2. All commands created by a given system are applied sequentially, in the order in which the methods were called on the `Commands` object in that system.
3. If a system is explicitly specified (using `.before` or `.after`) to occur before another system, its commands will always be applied first.
4. If two systems do not have an explicit ordering between them (including any transitive ordering from e.g. A before B before C), the order in which their commands are applied is unspecified, and may vary between runs of the app.

For more information on how to control system ordering, please read the [System Ordering](../../game-logic/system-ordering/_index.md) page in the next chapter.

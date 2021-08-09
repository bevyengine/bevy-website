+++
title = "Exclusive world access"
weight = 7
template = "book-section.html"
page_template = "book-section.html"
+++

In various places, Bevy code works directly with the {{rust_type(type="struct" crate="bevy_ecs" name="World")}} data, gaining exclusive, blocking access to it and allowing for arbitrary mutations.
This is very flexible, and can be essential for advanced use cases, but prevents any other systems from running simultaneously and can often be harder to reason about and maintain.
As a result, you should use only exclusive {{rust_type(type="struct" crate="bevy_ecs" name="World")}} when you have to.
You might be working with the {{rust_type(type="struct" crate="bevy_ecs" name="World")}} if:

- you're running an exclusive system to access data in unusually broad ways (such as for saving the game or handling networking)
- you're writing a custom command, to execute logic at the end of the system
- you're initializing a resource using the {{rust_type(type="struct" crate="bevy" mod = "ecs/world" name="FromWorld" no_mod = "true")}} trait
- you're working with {{rust_type(type="struct" crate="bevy" mod = "ecs/world" name="NonSend" no_mod = "true")}} values that cannot be sent across threads
- you're setting up tests to be run in a headless fashion
- you're using {{rust_mod(crate="bevy" mod="ecs")}} as a standalone crate

## Basic usage

Generally speaking, the API of working directly with the {{rust_type(type="struct" crate="bevy_ecs" name="World")}} mirrors those elsewhere that you might be familiar with.

Like with {{rust_type(type="struct" crate="bevy" mod = "ecs/system" name="Commands" no_mod = "true")}}, you can call {rust_type(type="struct" crate="bevy_ecs" name="World" method = "spawn")}}, {rust_type(type="struct" crate="bevy_ecs" name="World" method = "spawn_batch")}} and {rust_type(type="struct" crate="bevy_ecs" name="World" method = "despawn")}} to add and remove entities, adding components to them with {{rust_type(type="struct" crate="bevy_ecs" name="World" method = "insert")}} and {{rust_type(type="struct" crate="bevy_ecs" name="World" method = "insert_bundle")}}.
Resources are simply accessed with {rust_type(type="struct" crate="bevy_ecs" name="World" method = "get_resource")}} and the mutable equivalent {rust_type(type="struct" crate="bevy_ecs" name="World" method = "get_resource_mut")}}.

Like with queries, you can call {rust_type(type="struct" crate="bevy_ecs" name="World" method = "get")}} and {rust_type(type="struct" crate="bevy_ecs" name="World" method = "get_mut")}} to access component data of a particular sort on a given entity.
If you want access to *all* of the data on an entity, use {rust_type(type="struct" crate="bevy_ecs" name="World" method = "get_entity")}} and {rust_type(type="struct" crate="bevy_ecs" name="World" method = "get_entity_mut")}}, along with various {rust_type(type="struct" crate="bevy_ecs" name="World" method = "EntityRef)}} methods to operate on the returned object.

You can create new queries using {rust_type(type="struct" crate="bevy_ecs" name="World" method = "query")}} and {rust_type(type="struct" crate="bevy_ecs" name="World" method = "query_filtered")}}.
Use the former when you only have one type parameter and the latter when you want to use the second filtering type parameter of standard queries as well.

## Exclusive systems

Exclusive systems can operate on any data at once: no pesky carefully scoped data access needed.
This comes at a great cost: no other work can be done at the same time.

You *must* use `world: &mut World` as the only parameter in exclusive systems: all other data can be accessed using the methods on {{rust_type(type="struct" crate="bevy_ecs" name="World")}}.
Let the schedule know that they're exclusive by using {rust_type(type="trait" crate="bevy_ecs" mod = "system" name="IntoExclusiveSystem" method = "exclusive_system" no_mod = "true")}} on the system function, then use {rust_type(type="struct" crate="bevy_app" name="App" method = "add_system")}} or any of its relatives like usual.

Unlike ordinary systems, which can be executed in parallel in arbitrary orders, exclusive systems can run either:

1. Immediately before the start of a stage, using `my_exclusive_system().exclusive_system().at_start()`.
2. Immediately after the end of a stage, before commands are applied, using `my_exclusive_system().exclusive_system().before_commands()`.
3. Immediately after the end of a stage, after commands are applied, using `my_exclusive_system().exclusive_system().at_end()`.

`at_start` is the default behavior of exclusive systems and can be omitted.
Like other systems, exclusive systems obey standard ordering constraints, but only with respect to other exclusive systems running at the same time in the same stage.

Here's an example of where you might want the far-reaching power of exclusive systems:

```rust
// In this example, we're using a custom ActionStack to queue up
// free-form actions to enable complex, flexible logic in a turn-based game

/// Allows components that implement this trait to store arbitrary world-altering logic
pub trait Action: Send + Sync {
    fn perform(&mut self, world: &mut World);
}

/// Stores a vec of actions to be executed in first-in-last-out order
pub struct ActionStack(Vec<Box<dyn Action>>);

/// Takes the top action from the stack and runs it,
/// repeating until the action queue is empty
pub fn perform_next_action(world: &mut World) {
    loop {
        // This must be within the loop to satisfy the borrow checker
        // as our action could modify this resource
        let mut action_stack = world.get_resource_mut::<ActionStack>().unwrap();
        match action_stack.0.pop() {
            Some(mut action) => {
                action.perform(world);
            }
            None => break,
        }
    }
}
```

## Accessing multiple parts of the world simultaneously

When working with non-trivial exclusive world logic, you're likely to run into cases where you need mutable access to more than one part of the {{rust_type(type="struct" crate="bevy_ecs" name="World")}} at once.
This tends to make the compiler quite unhappy, but by carefully partitioning our data access we can ensure that our code doesn't violate Rust's memory safety rules.
No `unsafe` needed!

Right now, there are two main tools to do so:

1. {rust_type(type="struct" crate="bevy_ecs" name="World" method = "cell")}}: Like the [concept of the same name from the Rust standard library](https://doc.rust-lang.org/std/cell/), this enables interior mutability by disabling Rust's compile-time checks for aliased mutability and replacing them run-time checks.
2. {rust_type(type="struct" crate="bevy_ecs" name="World" method = "resource_scope")}}: temporarily removes the requested resource from the world, returning it at the end of your function (or when the created scope is manually dropped). This allows you to freely have multiple mutable references to distinct resources active at once.

## Accessing system parameters

Occasionally, you may find yourself reaching for convenient system parameters (like {{rust_type(type="struct" crate="bevy_app" name="EventReader")}} and {{rust_type(type="struct" crate="bevy_app" name="EventWriter")}}) while you have exclusive world access.
We can call these directly, using the same syntax as we use in systems, using the {{rust_type(type="struct" crate="bevy" mod="ecs/system" name="SystemState")}}type.

```rust
use bevy::prelude::*;
use bevy::ecs::SystemState;
use bevy::app::AppExit;

let mut world = World::new();

// Extract all of the relevant parameters from the world, as if it were a system
let mut system_state: SystemState<(Query<&Transform, With<Camera>>, Commands, EventWriter<AppExit>>)> =
            SystemState::new(&mut world);

// Use .get() if you only need read-access for less restrictive ownership constraints
let (camera_transform_query, mut commands, mut app_exit_event_writer) = system_state.get_mut(&world);

// You can then work with the extracted system parameters however you wish
app_exit_event_writer.send(AppExit);
```

## Manually running systems

If you'd like to use the familiar and expressive system syntax when working with {{rust_type(type="struct" crate="bevy_ecs" name="World")}}, you can manually run systems to immediately execute them one at a time.

```rust
struct Name(String);
let world = World::new();
world.insert_resource(Name{"Bevian".to_string()})

fn hello_system(name: Res<Name>){
    println!("Hello {}!", name);
}

// Setting up the system correctly
// This only needs to be done once per system per world
hello_system.system().initialize(world);
// Actually running the system
hello_system.system().run(world);
// Optionally, immediately apply any buffers (like Commands) created by the system to the World
hello_system.system().apply_buffers(world);
```

This will generally be less performant than using a standard parallel system stage,
but can offer greater simplicity and control, particularly when prototyping or for non-standard control flows.

It can also be useful for re-using or organizing logic used in other areas where you need exclusive world access, by wrapping it in a safe and well-contained function that is run in a single line.

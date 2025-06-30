+++
title = "The E, the C, and the S"
insert_anchor_links = "right"
[extra]
weight = 3
status = 'hidden'
+++

The core concept in Bevy is the [ECS architecture](https://en.wikipedia.org/wiki/Entity_component_system), which stands for **Entity, Component, System**. It is a way of organizing the data of a program, and controlling how that data is accessed and updated. ECS has been utilized in a number of commercial game engines, and has been increasing in popularity in the last several decades.

There are two main mental models for how to think about ECS:
- The **object-like model:** similar to game objects you may be familiar with from other engines.
- The **database model:** similar to an in-memory SQL database or spreadsheet.

We'll reference both conceptual models throughout this chapter.

So, what does each letter mean?

## The E: Entities

**Entities are objects** in our game world.
This might include:
- The player
- Each enemy
- Props in the game scene
- The camera
- The skybox

An Entity, by itself, is just an identifier; it does not store any data within it. In order to be useful, it needs to be associated with one or more [components](#the-c-components).

In the "in-memory database" model, entities are the row keys in our database, with each entity getting its own row and unique identifier.

While entities are conceptually similar to Objects in object-oriented engines, they are distinctly different bececause they **do not store any behavior**.
This is handled by [systems](#the-s-systems).

**Note on terminology**: Sometimes, using the word "entity" on its own can be ambiguous. Does it mean the row/id/primary key or does it mean the game object/thing it represents with all its data? In Bevy, entity ids are modeled in the `Entity` type. As a result, `Entity` typically refers to the id, and a lowercase "entity" typically refers to the game object.

## The C: Components

A **component** is a modular piece of data that can be reused across entities in the world.
In Bevy, components are "just Rust structs" (or enums).

```rs
/// The location of a player, creature, or object in our game
#[derive(Component)]
struct Location {
    x: f32,
    y: f32,
    z: f32,
}

/// The color of an object in our game
#[derive(Component)]
enum Color {
    Red,
    Green,
    Blue,
    Heliotrope,
}

/// A "marker" component for entities which represent a player
///
/// Since this contains no data, this is more like a tag
#[derive(Component)]
struct Player;
```

Any number and combination of components can be added to an entity, and each entity gets its own value for that component.
In the database model, components are like the columns of our database (although not every entity will have every component).

Spawning entities with components is done like so:
```rs
fn spawn_entities(mut commands: Commands) {
    // Spawn an entity with all our components
    commands.spawn((Location::zero(), Color::Red, Player));
    // Spawn an entity with only one component
    commands.spawn(Color::Heliotrope);
}
```

Entities are usually spawned using [Commands](../intro/the-next-three-letters#commands), which will be covered in the next section.

## The S: Systems

Systems interact with and update the data in the ECS.
By default, each system is run each frame, in a loop (specifically, in a [Schedule](../../the-game-loop/schedules)).
In Bevy, systems are "just Rust functions".
These can fetch data from the ECS, make updates, call external APIs, and anything else that a function can do.

```rs
// No derive macro needed!
fn my_system(entities: Query<&mut Location>) {
    for location in entities.iter_mut() {
        location.x += 1;
    }
}
```

Entities and their components are usually accessed via [Queries](../intro/the-next-three-letters#queries), which will be covered in the next section.

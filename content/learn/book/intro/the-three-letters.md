+++
title = "The E, the C, and the S"
insert_anchor_links = "right"
[extra]
weight = 3
status = 'hidden'
+++

The core concept in Bevy is the ECS architecture, which stands for **Entity, Component, System**.
ECS is a method of structuring the data of a program, and how that data is accessed and updated.
Another way to think of ECS is as an in-memory database.
We'll go over both conceptual models throughout this chapter.

So, what does each letter mean?

## The E: Entities

**Entities are single objects** in our game world.
This includes:
- The player
- Each enemy
- Each object
- The camera
- The skybox

Entities store data in a modular fashion using [components](#the-c-components) (see below).
In general, an entities do not store anything themselves, they are just "a pile of components".
In the "in-memory database" model, entities are the rows in our database, with each entity getting its own row and ID.

Note: While entities are similar to Objects in object-oriented engines, they are distinctly different bececause they **do not store any behavior**.
(This is handled by [systems](#the-s-systems), below.)

## The C: Components

A **component** is a modular piece of data that can be reused across entities in the world.
In Bevy, components are "just Rust structs" (or enums).

```rs
/// The location of a player, creature, or object in our game
#[derive(Component)]
struct Location {
    x: f32,
    y: f32,
    z: f32
}

/// The color of an object in our game
#[derive(Component)]
enum Color {
    Red,
    Green,
    Blue,
    Heliotrope
}
```

Any number of components can be added to an entity, and each entity gets its own value for that component.
In the database model, components are like the columns of our database (although not every entity will have every component).

Spawning entities with components is done like so:
```rs
fn spawn_entities(commands: &mut Commands) {
    // Spawn an entity with a both our components
    commands.spawn((Location::zero(), Color::Red));
    // Spawn an entity with only one component
    commands.spawn(Color::Heliotrope);
}
```
(Entity spawning is done using [Commands](../intro/the-next-three-letters#commands), which will be covered in the next section.)

## The S: Systems

Systems interact with and update the data in the ECS.
By default, each system is run each frame, in a loop (specifically, in a [Schedule](todo-link-to-schedule-chapter)).
In Bevy, systems are "just rust functions".
These can fetch data from the ECS, make updates, call external APIs, and anything else that a function can do.

```rs
// No derive macro needed!
fn my_system(entities: Query<&mut Location>) {
    for location in entities.iter_mut() {
        location.x += 1;
    }
}
```
(Most ineractions with entities and their components is done using [Queries](../intro/the-next-three-letters#queries), which will be covered in the next section.)

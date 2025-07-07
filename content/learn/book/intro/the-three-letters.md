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
- The player's inventory, buffs, or active enchantments
- Each enemy
- Props in the game scene
- The camera
- The skybox
- Particle effect clouds

An Entity, by itself, is just an identifier; it does not store any data within it. In order to be useful, it needs to be associated with one or more [components](#the-c-components).

In the "in-memory database" model, entities are the row keys in our database, with each entity getting its own row and unique identifier.

While entities are conceptually similar to Objects in object-oriented engines, they are distinctly different bececause they **do not store any behavior**.
This is handled by [systems](#the-s-systems).

{% callout(type="note") %}
**Note on terminology**: Sometimes, using the word "entity" on its own can be ambiguous. Does it mean the row/id/primary key or does it mean the game object/thing it represents with all its data? In Bevy, entity ids are modeled in the `Entity` type. As a result, `Entity` typically refers to the id, and a lowercase "entity" typically refers to the game object.
{% end %}

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

{% callout(type="info") %}
Bevy systems use a technique called [dependency injection](https://en.wikipedia.org/wiki/Dependency_injection) to access data about the Bevy world. By declaring your function parameters wrapped in special types like [Query](../intro/the-next-three-letters#queries) or [Res](../intro/the-next-three-letters#resources), the data for those parameters will be filled in for you automagically - without you having to actually call the system.

Another cool feature of Bevy systems is automatic parallelism: by inspecting the function parameter types, Bevy can automatically determine if it's safe to run two systems concurrently. For example, if you have a system which regenerates character health by modifying a `Health` component, and a different system that manages the characters' mana pool (say, via a `Mana` component), then Bevy knows that these two data sets are _disjoint_ and can be updated at the same time. This is particularly important for optimal utilization of multiple CPU cores.
{% end %}

Systems usually access Entities and their components via [Queries](../intro/the-next-three-letters#queries), which will be covered in the next section.

## Why ECS?

At this point, you may be wondering: why bother with all of this machinery and these new concepts?
What's wrong with a good-old-fashioned game loop?
Aren't game object models simpler?

We won't deny it: these approaches work, and people have and can build great games with them.
But we think that by focusing on ECS as the heart of an engine (rather than a tacked on feature),
you can:

- write fast, scalable code by default
  - most operations in games are of the form "look at each of these objects and do the same thing to them"
  - because of better [data locality], ECS architectures are much faster at iterating during these operations
  - no more speculative rewrites of whole subsystems: gradually optimize the hot loops
- have engine code that looks like library code that looks like game code
  - weird behavior? Check the source!
  - this makes [contributing](/learn/contribute) fixes and features to Bevy much easier
  - and it helps support a thriving, heavily interoperable [ecosystem of third-party libraries](https://bevy.org/assets/)
- build consistent, universal abstractions on a common base of data structures
  - shared data structures mean that improvements and bug fixes trickle down automatically
  - use the same powerful patterns for [control flow](../control-flow/) everywhere
  - structure your application using a uniform, flexible [modular architecture](../modular-architecture)
  - debug and inspect every part of your game using the same [dev tools](../development-practices)

Learning to take advantage of everything a modern ECS has to offer will take time:
if you want to be able to tackle any data modelling problem that games have to throw at you,
you need a lot more than just entities, components, and systems.
Even if you're a veteran game programmer, there will be a learning curve
as you explore new approaches and master new tools.

But start simple, and add in new patterns as you encounter the problems they're solving.
With a bit of persistence, you'll be flying in no time!

[data locality]: https://en.wikipedia.org/wiki/Locality_of_reference

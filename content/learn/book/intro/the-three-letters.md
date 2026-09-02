+++
title = "The E, the C, and the S"
insert_anchor_links = "right"
[extra]
weight = 3
status = 'hidden'
+++

Bevy's architecture centers around its [ECS](https://en.wikipedia.org/wiki/Entity_component_system): E for **Entity**, C for **Component** and S for **System**. ECS is a high-performance way of organizing the data of a program, and controlling how that data is accessed and updated.
ECS has been utilized in a number of commercial game engines, and has been increasing in popularity over the last couple of decades.
Bevy however is relatively unique in how widely it uses these patterns: ECS in Bevy is used _everywhere_, not just for performance-critical code.

## The E: Entities

**Entities are objects** in our game world.
This includes things like players, enemies, inventories, buffs, trees, buildings, cameras, skyboxes, particle effects, etc.

An entity, by itself, is just an identifier; it does not store any data within it. In order to be useful, it needs to be associated with one or more [components](#the-c-components). [`Entity`] is the "entity identifier" type.

"Empty" entities can be spawned like this:

```rust
let entity: Entity = commands.spawn_empty().id();
```

While entities are conceptually similar to "objects" in object-oriented programming, they are distinctly different in that they are _composable_. You can add new data and behaviors to them using [components](#the-c-components) and [systems](#the-s-systems).

{% callout(type="info") %}
**Note on terminology**: Sometimes, using the word "entity" on its own can be ambiguous. It can mean the identifier (the `Entity` type) or it can mean the whole entity "object", with all of its stored components and behaviors. By convention, `Entity` typically refers to the id, and a lowercase "entity" typically refers to the "whole" game object.
{% end %}

## The C: Components

A **component** is a modular piece of data that can be reused across entities in the world.
In Bevy, components are just normal Rust types that implement the [`Component`] trait:

```rs
/// The grid-based location of a player, creature, or object in our game.
#[derive(Component)]
struct Location {
    x: u32,
    y: u32,
}

/// The color of an object in our game.
#[derive(Component)]
enum Color {
    Red,
    Green,
    Blue,
}

/// A "marker" component for entities which represents a player. This contains no data. 
/// It behaves like a "tag" in the ECS / stores no data on the entity.
#[derive(Component)]
struct Player;
```

Any number and combination of components can be added to an entity, and each entity gets its own value for that component.

Spawning entities with components is done like so:

```rs
fn spawn_entities(mut commands: Commands) {
    // Spawn an entity with only one component
    commands.spawn(Player);
    // Spawn an entity with multiple components
    commands.spawn((Player, Color::Red, Location { x: 0, y: 0 }));
}
```

Entities are usually spawned using [Commands](@/learn/book/control-flow/commands.md), which queue up work to be done later. They can also be spawned immediately using a [`World`] (more on this distinction later!).

## The S: Systems

Systems interact with and update the data in the ECS.
Each system is run every frame by default, and repeats in a loop (specifically, in a [Schedule](@/learn/book/the-game-loop/schedules.md)).
In Bevy, systems are normal Rust functions.
These can fetch data from the ECS, make updates, call external APIs, and anything else that a function can do.

```rs
fn my_system(mut entities: Query<&mut Location>) {
    for location in entities.iter_mut() {
        location.x += 1;
    }
}
```

{% callout(type="info") %}
Bevy systems use a technique called [dependency injection](https://en.wikipedia.org/wiki/Dependency_injection) to access ECS data. Function parameters like [Query](@/learn/book/intro/the-next-three-letters.md#queries) and [Res](@/learn/book/intro/the-next-three-letters.md#resources) will have their data filled in for you automatically!
{% end %}

Systems usually access entities and their components via [Queries](@/learn/book/intro/the-next-three-letters.md#queries), which will be covered in the next chapter.

Bevy systems are generally run in parallel automatically. By inspecting the function parameter types, Bevy can automatically determine if it's safe to run two systems concurrently. This is particularly important for optimal utilization of multiple CPU cores.

For example, if you have a system which regenerates character health by modifying a `Health` component, and a different system that manages the characters' mana pool (say, via a `Mana` component), then Bevy knows that these two data sets are _disjoint_ and can be updated at the same time. 

## The Database Analogy

ECS is very similar to an "in-memory database", both from an implementation perspective and a logical model perspective. Entities are like the "rows" of a database. The [`Entity`] id is like a "primary key". Components are like "columns". We can run queries on the data in these "tables". If you have experience with databases, you may find this analogy helpful. But you definitely don't need database experience to get the most out of Bevy!

## Why ECS?

At this point, you may be wondering: why bother with all of this machinery and these new concepts?
What's wrong with a good-old-fashioned game loop?
Aren't game object models simpler?

We won't deny it: these approaches work, and people can and have built great games with them.
But by focusing on ECS as the heart of an engine (rather than a tacked on feature), we get significant benefits:

- **Write fast, scalable code by default**:
  - Most operations in games take the shape of "look at each of these objects and do the same thing to them". Because of better [data locality], ECS architectures are much faster at iterating during these operations.
  - Non-ECS-by-default engines either don't receive these benefits at all, or require refactoring your data to use special-cased "fast path" APIs. In Bevy, everything is fast by default using a single, unified API.
- **Engine code, game code, and library code all look the same**:
  - Because Bevy uses the same ECS API pretty much everywhere (including the engine itself), there is no line between "engine developer" and "app developer". Want to know how something in the engine works? Check the source! It will use the same patterns you're used to in your app code.
  - This makes [contributing](@/learn/contribute/_index.md) fixes and features to Bevy much easier.
  - It enables a thriving, heavily interoperable [ecosystem of third-party libraries](https://bevy.org/assets/). The only difference between a third party plugin and a built-in engine feature is what repository it lives in!
- **A consistent data model**
  - Structure your application using a uniform, flexible [modular architecture](@/learn/book/modular-architecture/_index.md).
  - Debug and inspect every part of your game (including engine internals) using the same [dev tools](@/learn/book/development-practices/_index.md).
  - Shared data structures mean that improvements and bug fixes trickle down automatically.
  - Use the same powerful patterns for [control flow](@/learn/book/control-flow/_index.md) everywhere.

Learning to take advantage of _everything_ a modern ECS has to offer will take time, but Bevy's ECS is actually extremely approachable!
Start simple, and only add in new concepts and patterns when there is a problem to solve. The basics are straightforward, and if you are coming from a "traditional" engine, you can start by using the general patterns and structures you are used to.

[data locality]: https://en.wikipedia.org/wiki/Locality_of_reference

[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`Component`]: https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html

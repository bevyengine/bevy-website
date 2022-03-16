+++
title = "Entities, components and systems"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

Bevy takes an ECS-first approach to organizing data and orchestrating game logic.
ECS stands for ["Entity-Component-System"](https://en.wikipedia.org/wiki/Entity_component_system), and is a high-performance, modular [paradigm](https://ajmmertens.medium.com/ecs-from-tool-to-paradigm-350587cdf216) for organizing and manipulating data.
In a nutshell: data is stored as components which belong to entities, which is extracted and then modified by systems.

Each **component** store a piece of data for a single entity in a strongly-typed fashion.
You may have a `Life` component that stores how much life an entity has, a `Team` enum component that defines which team the entity belongs to, or even a dataless `Player` marker component that serves as a hint that this entity represents the player.
This allows us to build up entities in a composable fashion, sharing data types (and behavior) across different kinds of game objects.

```rust
use bevy::prelude::*;

// This derive macro automatically implements the `Component` trait for our type
#[derive(Component)]
struct Life {
    current: u8,
    max: u8,
}

// Enums can be used as components:
// each entity with this component belongs to exactly one of the Red, Blue or Green team
#[derive(Component)]
enum Team {
    Red,
    Blue,
    Green,
}

// Components don't have to store data
// Marker components like this are very helpful to toggle behavior
// when combined with query filters
#[derive(Component)]
struct Player;

// This system that spawns a new player each time it is run
fn spawn_player(commands: Commands){
    // Each component is added seperately to the entity;
    // there's no limitation on which components any given entity can have
    commands.spawn().insert(Life {current: 10, max: 10}).insert(Team::Blue).insert(Player);
}
```

Generally, components are kept quite small, which increases the flexibility of the design and reduces the amount of unneeded data that is fetched.
To make this design easier to work with, related components are commonly grouped together in **component bundles** to allow users to easily build entities with complex behavior.

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Strength(u8);

#[derive(Component)]
struct Dexterity(u8);

#[derive(Component)]
struct Agility(u8);

// This component bundle groups several related components together
// allowing them to be inserted in a single statement
#[derive(Bundle)]
struct AttributeBundle {
    // Each of the types in the bundle
    // will be inserted as a seperate component in our entity
    strength: Strength,
    dexterity: Dexterity,
    intelligence: Intelligence,
}

// In Rust, impl blocks allow you to define methods for types
impl AttributeBundle {
    // Defining "builder methods" for bundle types is a common pattern,
    // and allows you to simplify construction and 
    fn new(strength: u8, dexterity: u8, intelligence: u8) -> AttributeBundle {
        AttributeBundle {
            strength,
            dexterity,
            intelligence
        }
    }
}

fn spawn_player_with_attributes(mut commands: Commands){
    // This starts the same as the previous method
    commands.spawn()
        .insert(Life {current: 10, max: 10})
        .insert(Team::Blue)
        .insert(Player)
        // But here, we're inserting several components at once
        // Our player is much stronger than they are smart!
        .insert_bundle(AttributeBundle::new(9, 4, 3));

    // We can make a nemesis by spawning another entity!
    commands.spawn()
        .insert(Life {current: 10, max: 10})s
        .insert(Team::Red)
        .insert(Player)
        // Inserting the components one at a time leads to exactly the same result
        // But as you can see, is often more verbose (and error-prone!)
        .insert(Strength(9))
        .insert(Dexterity(4))
        .insert(Intelligence(3));
}
```

**Entities** are simply collections of components, and the [`Entity`] type is simply a unique identifier for that particular type: something like a name, URL or row number in a database.

```rust
use bevy::prelude::*;

fn spawn_player_in_a_convoluted_way(mut commands: Commands){
    // We can store which Entity we're working with
    let player_entity = commands.spawn()
        .insert(Life {current: 10, max: 10})
        .insert(Team::Blue)
        .insert(Player)
        // This .id() method grabs the `Entity` identifier
        // of the entity we're working with
        .id();

    // Inspect the identifier used
    dbg!(player_entity);

    // And then operate on a specific entity using the `Commands::entity` method
    commands.entity(player_entity).insert_bundle(AttributeBundle::new(9, 4, 3));
}
```

In order to read, manipulate and act on the data we've created, we must use systems.
**Systems** are Rust functions that request specific data from the [`World`] by defining which data they need in their **system parameters**.
Most commonly, systems use a [`Query`] to select component data from all entities with a matching collection of components.

## The game loop

Once systems are added to our app, the **runner** takes this information and automatically runs our systems, typically once during each pass of the **game loop** according to the rules defined in the **schedule**.

Bevy's default execution strategy runs systems in parallel.
Because the **function signature** of each of our systems fully define the data it can access, we can ensure that only one system can change a piece of data at once (although any number can read from a piece of data at the same time).

Systems within the same **stage** are allowed to run in parallel with each other (as long as their data access does not conflict), and are assigned to a thread to perform work as soon as one is free.

```rust
fn main(){
    let app = App::new()
        .add_plugins(MinimalPlugins)
        // Startup systems run exactly once, when our app is first initialized
        .add_startup_system(todo!())
        // Regular systems are run each time the system loops
        .add_system(todo!())
        .add_system(todo!());

    for i in 1..10 {
        // This runs our app's schedule a single time
        // When writing real games, you'll typically want to use App::run to loop the schedule indefinitely,
        // but this method is very valuable for testing and teaching purposes!
        app.update();
    }
}
```

## Resources

Some data isn't reasonably stored as components on a particular entity: it may represent a piece of global state or configuration.
In these cases, we can turn to **resources**: simple, unique global stores of data.

These are accessed from the [`World`] via their type using the [`Res`] (for read-only access) or [`ResMut`] (for read-write access) system parameters.

## Working with the `World`

When we need to access data in complex, cross-cutting ways that are not cleanly modelled by our systems' function signatures, we can defer the work until we have exclusive access to the entire [`World`'s] data: executing **commands** generated in earlier systems at the end of each stage (to do things like spawn entities or insert components) or performing complex logic (like saving the entire game) in our own **exclusive systems**.

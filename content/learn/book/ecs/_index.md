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

**Systems** can be used to read, manipulate and otherwise act on the data we've created, bringing our game to life.
In Bevy, all behavior is ultimately powered by systems, and systems are constructed as ordinary Rust functions.
The data that each system can access is defined by its **system parameters**: function parameters with types that implement the [`SystemParam`] trait.
[`Commands`], shown in the examples above, is a system parameter, but there are many more, each requesting a different kind of data from the [`World`].

```rust
#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
struct Life {
    current: u8,
    max: u8,
}

#[derive(Component)]
struct LifeRegen(u8);

// The `Query` system parameter allows us to fetch component data from our entities
// 
// The first type parameter defines the data that should be returned,
// while the second type parameter "filters" the entities that have the matching components
fn print_player_life(query: Query<&Life, With<Player>>){
    // Queries return all matching entities
    // by iterating over them we can perform the same logic on each entity
    for life in query.iter(){
        dbg!(life);
    }
}

// We can request multiple components in our queries by wrapping them in a (A, B) tuple
// and mutate the components by requesting &mut A rather than &A
fn regenerate_life(mut query: Query<(&mut Life, &LifeRegen)>){
    // We can use "destructuring" to unpack our query items into the corresponding types
    for (mut life, life_regen) in query.iter_mut(){
        // .0 means "the first field of a tuple struct"
        life.current += life_regen.0;
        
        // We shouldn't let life regeneration heal our units above full!
        if life.current > life.max {
            life.current = life.max;
        }
    }
}
```

Once systems are added to our app, the **runner** takes this information and automatically runs our systems, typically once during each pass of the **game loop** according to the rules defined in the **schedule**.

Bevy's default execution strategy runs systems in parallel.
Because the **function signature** of each of our systems fully define the data it can access, we can ensure that only one system can change a piece of data at once (although any number can read from a piece of data at the same time).

Systems within the same **stage** are allowed to run in parallel with each other (as long as their data access does not conflict), and are assigned to a thread to perform work as soon as one is free.

```rust
fn main(){
    let app = App::new()
        .add_plugins(MinimalPlugins)
        // Startup systems run exactly once, when the schedule is first run
        .add_startup_system(spawn_damaged_player)
        // Regular systems are run each time the schedule loops
        .add_system(regenerate_life)
        // We can use .before and .after to enforce a consistent ordering of our systems
        .add_system(print_player_life.after(regenerate_life));

    for i in 1..10 {
        // This runs our app's schedule a single time
        // When writing real games, you'll typically want to use App::run to loop the schedule indefinitely,
        // but this method is very valuable for testing and teaching purposes!
        app.update();
    }
}
```

While there's much more to learn about Bevy's ECS, this basic overview should give you the vocabulary you need to start exploring the rest of this chapter.
Don't worry if some concepts are too abstract or impractical for you at this point:
this book is intended to be skimmed on the first read.
Refer back to it later for more detailed explanations as you start building your own awesome projects in Bevy!

[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
[`SystemParam`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html

+++
title = "Entities, components and systems"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

Bevy is fundamentally powered by its ECS (Entity Component System): almost all data is stored as components on entities, and all logic is executed by its systems.

As we mentioned in the last chapter, all of our data is stored in a [`World`](https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html) on our [`App`](https://docs.rs/bevy/latest/bevy/app/struct.App.html).
We can think of our **entity-component data storage** as a giant database:

* each row is an **entity**, representing an object (perhaps a player, tile, or UI button) in our game
* each column is a type of **component**, storing data of a particular type (perhaps the sprite, team or life of a player entity) in an efficient way
* each cell is a component of a particular entity, which has a concrete value we can look up and change
* we access data from this database using **queries**, which fetch entities with the specified components
* the primary key of this database is the [`Entity`](https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html) identifier, which can be used to look up specific entities using [`Query::get(my_entity)`](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Query.html#method.get)

Of course, this database is not very well-normalized: not all entities will have every component!
We can use this fact to specialize behavior between entities: systems only perform work on entities with the correct combination of components.
You don't want to apply gravity to entities without a position in your world, and you're only interested in using the UI layout algorithm to control the layout of UI entities!

When we want to go beyond this tabular data storage, we can use **resources**: global singletons which store data in monolithic blobs.
You might use resources to store one-off bits of state like the game's score, use it to interface with other libraries, or store secondary data structures like indexes to augment your use of entity-component data.

In order to actually manipulate all of this data in interesting ways, we must use systems.
**Systems** are Rust functions which request specific data from the world, as declared in their system parameters (function parameters): generally resources and entities that have a particular combination of components using queries.
Systems are Rust functions that request data from the `World` (such as resources or entities with particular components) in order to perform tasks.
All of the rules and behaviours of our game are governed by systems.

Once the systems are added to our app the **runner** takes in this information and automatically runs our systems: typically once during each pass of the **game loop** according to the rules defined in their **schedule**.
Bevy's default execution strategy runs systems in parallel by default, without the need for any manual setup.
Because the **function signature** of each of our systems fully define the data it can access, we can ensure that only one system can change a piece of data at once (although any number can read from a piece of data at the same time).
If no shared mutable access exists, systems within the same **stage** are allowed to run in parallel freely, assigned to a free thread as soon as it is free.

When we need to access data in complex, cross-cutting ways that are not cleanly modelled by our systems' function signatures, we can defer the work until we have exclusive access to the entire world's data: executing **commands** generated in earlier systems at the end of each stage or performing complex logic (like saving the entire game) in our own **exclusive systems**.
You will first encounter this when spawning and despawning entities: we have no way of knowing precisely which other components our entities might have, and so we are forced to wait until we can ensure that we can safely write to *all* component data at once.

## ECS by example

Before we dive into the details of each of these features, let's take a quick look at a simple game that you can run and play.
Unsurprisingly, the different parts of the ECS tend to be closely linked: components are not very useful without a way to spawn entities and systems that run our logic are very dull if we can't discuss the data they can access.
The details of each part are more easily grasped if you have a basic sense of the whole.

```rust
use bevy::core::FixedTimestep;
use bevy::prelude::*;

// In this game, two combatants are fighting a duel
// Dealing damage to each other until one combatant's life total reaches 0
fn main() {
    App::new()
        // Resources store global state
        .insert_resource(Victory::Undetermined)
        .add_plugins(DefaultPlugins)
        // Startup systems run exactly once, before ordinary systems run
        // and are typically used for initialization
        .add_startup_system(spawn_combatants)
        // We only want to run this system every few seconds to allow the reader to follow along
        .add_system(
            combat
                .label("combat")
                .with_run_criteria(FixedTimestep::step(3.0)),
        )
        // We want to ensure that we report life totals before damage is dealt
        .add_system(report_life.before("combat"))
        // We want to ensure that we check victory after damage is dealt
        .add_system(check_victory.after("combat"))
        .run();
}

// This resource stores who has won our duel
#[derive(PartialEq, Clone, Copy)]
enum Victory {
    Undetermined,
    Concluded(Side),
    Draw,
}

// This resource stores the Entity id's of our two combatants
struct CombatantEntities {
    player_entity: Entity,
    enemy_entity: Entity,
}

// These components define the data stored on our entities
struct Life(i8);
struct Attack(i8);
struct Defense(i8);

#[derive(PartialEq, Clone, Copy)]
enum Side {
    Player,
    Enemy,
}
struct Name(String);

// Bundles are used to gather groups of components
// allowing convenient access
#[derive(Bundle)]
struct CombatBundle {
    // Each field of this bundle corresponds to a type of component
    // that will be inserted into the final entity
    // The field names are only used when instantiating the bundle;
    // only the types are retained in our ECS storage
    life: Life,
    attack: Attack,
    defense: Defense,
    side: Side,
}

// This is a startup system that we add to our app
// It runs only once, before everything else has occurred
fn spawn_combatants(mut commands: Commands) {
    // Spawning the player entity
    let player_entity = commands
        // Creating a base entity
        .spawn()
        // Adding a single component to that entity
        .insert(Name("Gallant".to_string()))
        // Adding a collection of components to that entity
        .insert_bundle(CombatBundle {
            life: Life(10),
            attack: Attack(5),
            defense: Defense(2),
            side: Side::Player,
        })
        // .id() just causes the expression to return the Entity id that was just spawned;
        // if you don't need to store that information you should omit it.
        .id();

    // Spawning the enemy entity
    let enemy_entity = commands
        .spawn_bundle(CombatBundle {
            life: Life(8),
            attack: Attack(6),
            defense: Defense(1),
            side: Side::Enemy,
        })
        .insert(Name("Gallant".to_string()))
        .id();

    // We're recording the Entity id's of our combatants
    // So we can easily access them later
    commands.insert_resource(CombatantEntities {
        player_entity,
        enemy_entity,
    })
}

// The query in this system only returns entities whose life total has changed since it last ran
fn report_life(query: Query<(&Life, &Name), Changed<Life>>) {
    for (life, name) in query.iter() {
        // .0 refers to the first (and only) field of our tuple structs
        println!("{} is at {} life!", life.0, name.0);
    }
}

fn combat(
    mut query: Query<(&mut Life, &Attack, &Defense)>,
    combatant_entities: Res<CombatantEntities>,
    victory: Res<Victory>,
) {
    // We only want combat to continue if victory has not yet been declared
    if *victory == Victory::Undetermined {
        // FIXME: does not compile due to borrow checker not understanding that player_entity != enemy_entity
        let player = query.get_mut(combatant_entities.player_entity).unwrap();
        let enemy = query.get_mut(combatant_entities.enemy_entity).unwrap();

        // This pattern matching destructures our (Mut<Life>, &Attack, &Defense) tuple into three new variables
        let (mut p_life, p_attack, p_defense) = player;
        let (mut e_life, e_attack, e_defense) = enemy;

        // The attacks are made simultaneously
        // FIXME: &* is ugly
        *p_life = damage_calculation(&*p_life, e_attack, p_defense);
        *e_life = damage_calculation(&*e_life, p_attack, e_defense);
    }
}

fn damage_calculation(life: &Life, attack: &Attack, defense: &Defense) -> Life {
    // Attacks never deal negative damage
    let damage_dealt = (attack.0 - defense.0).max(0);
    // Life totals cannot drop below 0
    let new_life_total = (life.0 - damage_dealt).max(0);
    Life(new_life_total)
}

fn check_victory(query: Query<(&Life, &Side), Changed<Life>>, mut victory: ResMut<Victory>) {
    for (life, &side) in query.iter() {
        if life.0 == 0 {
            *victory = match *victory {
                Victory::Undetermined => Victory::Concluded(side),
                Victory::Concluded(old_victor) => {
                    if old_victor != side {
                        Victory::Draw
                    } else {
                        Victory::Concluded(old_victor)
                    }
                }
                Victory::Draw => Victory::Draw,
            }
        }
    }
}
```

If you'd like to see more tiny working games like this, check out our [game examples](https://github.com/bevyengine/bevy/tree/latest/examples/game) on the Bevy GitHub repository.

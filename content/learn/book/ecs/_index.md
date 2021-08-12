+++
title = "Entities, components and systems"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

Bevy is fundamentally powered by its ECS (Entity Component System): almost all data is stored as components on entities, and all logic is executed by its systems.

As we [mentioned in the last chapter](../welcome/app/_index.md), all of our data is stored in a {{rust_type(type="struct" crate="bevy_ecs" name="World")}} on our {{rust_type(type="struct" crate="bevy" name="App")}}).
We can think of our **entity-component data storage** as a giant in-memory database:

* each row is an **entity**, representing an object (perhaps a player, tile, or UI button) in our game
* each column is a type of **component**, storing data of a particular type (perhaps the sprite, team or life of a player entity) in an [efficient way](https://github.com/bevyengine/bevy/pull/1525) that keeps data of the same type tightly packed together
* each cell is a component of a particular entity, which has a concrete value we can look up and change
* we access data from this database using **queries**, which fetch entities with the specified components
* the primary key of this database is the {{rust_type(type="struct" crate="bevy_ecs" name="Entity")}} identifier, which can be used to look up specific entities using {{rust_type(type="struct" crate="bevy_ecs" name="Query" method = "get")}}

Of course, this database is [very ragged](https://www.transdatasolutions.com/what-is-ragged-data/): not all entities will have every component!
We can use this fact to specialize behavior between entities: systems only perform work on entities with the correct combination of components.
You don't want to apply gravity to entities without a position in your world, and you're only interested in using the UI layout algorithm to control the layout of UI entities!

When we want to go beyond this tabular data storage, we can use **resources**: global singletons which store data in monolithic blobs.
You might use resources to interface with other libraries, store unique bits of state like the game's score, or store secondary data structures like indexes to augment your use of entity-component data.

In order to manipulate and act on this data, we must use systems.
**Systems** are Rust functions that request specific data, such as resources and entities, from the {{rust_type(type="struct" crate="bevy_ecs" name="World")}}. They define a query in their parameters (arguments) that selects data with a particular combination of components.
All of the rules and behaviours of our game are governed by systems.

Once the systems are added to our app the **runner** takes in this information and automatically runs our systems: typically once during each pass of the **game loop** according to the rules defined in their **schedule**.
Bevy's default execution strategy runs systems in parallel by default, without the need for any manual setup.
Because the **function signature** of each of our systems fully define the data it can access, we can ensure that only one system can change a piece of data at once (although any number can read from a piece of data at the same time).
Systems within the same **stage** are allowed to run in parallel with each other (as long as their data access does not conflict), and are assigned to a free thread as soon as one is free.

When we need to access data in complex, cross-cutting ways that are not cleanly modelled by our systems' function signatures, we can defer the work until we have exclusive access to the entire world's data: executing **commands** generated in earlier systems at the end of each stage or performing complex logic (like saving the entire game) in our own **exclusive systems**.
You will first encounter this when spawning and despawning entities: we have no way of knowing precisely which other components our entities might have, and so we are forced to wait until we can ensure that we can safely write to *all* component data at once.

## ECS by example

Before we dive into the details of each of these features, let's take a quick look at a simple game that you can run and play.
Unsurprisingly, the different parts of the ECS tend to be closely linked: components are not very useful without a way to spawn entities and systems that run our logic are very dull if we can't discuss the data they can access.
The details of each part are more easily grasped if you have a basic sense of the whole.

```rust
use bevy::app::AppExit;
use bevy::log::LogPlugin;
use bevy::prelude::*;

// This component defines our entity's life total.
#[derive(Component)]
struct Life(f32);

// This component is used to mark if our entity is currently airborne.
#[derive(Component)]
struct Falling {
    // The higher the initial height of falling, the higher the damage.
    height: f32,
}

fn main() {
    App::new()
        // These plugins create the basic framework
        .add_plugins(MinimalPlugins)
        // This allows us to report player health using `info!`
        .add_plugin(LogPlugin)
        // Because we've added this system as a startup system,
        // it runs exactly once before any ordinary system
        .add_startup_system(spawn_player_system)
        // Ordinary systems run once per frame (or pass of the game loop).
        .add_system(gravity_system.label("gravity"))
        // We need to make sure we report fall damage after gravity
        // Otherwise it won't have been calculated yet
        .add_system(fall_damage_system.after("gravity"))
        .run();
}

// This system spawns the player at a fairly high elevation.
fn spawn_player_system(mut commands: Commands) {
    const INITIAL_HEIGHT: f32 = 15.0;

    // Entities must be spawned in a delayed fashion with commands.
    commands
        .spawn()
        // We can add components to entities that we are spawning with the .insert()
        .insert(Life(20.0))
        // Transform is the standard position component in Bevy,
        // controlling the translation, rotation and scale of entities
        .insert(Transform::from_translation(Vec3::new(
            0.0,
            INITIAL_HEIGHT,
            0.0,
        )))
        .insert(Falling {
            height: INITIAL_HEIGHT,
        });

    // This expression creates a second entity, with a slightly different set of components
    commands
        .spawn()
        // We can customize the starting values of our components
        // by changing the data stored in the structs we pass in
        .insert(Life(30.0))
        // This player begins on the ground
        // So we're not inserting the Falling component
        .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)));
}

// This system pulls down the entity towards the ground (at y = 0), at a constant velocity,
// only while it's falling.
// The With<Falling> filter ensures that only entities with the `Falling` component are affected
fn gravity_system(mut query: Query<&mut Transform, With<Falling>>) {
    const FALL_RATE: f32 = 1.0;

    // Performing the same operation on each entity returned by the query
    // using a loop is a very common pattern
    for mut transform in query.iter_mut() {
        transform.translation.y = (transform.translation.y - FALL_RATE).max(0.0);
    }
}

// This system deals damage to falling entities based on the height from which it fell
fn fall_damage_system(
    mut commands: Commands,
    // By adding `Entity` to our query, we can extract
    // the unique identifier of the entity we're iterating over
    mut query: Query<(Entity, &mut Life, &Falling, &mut Transform)>,
    mut exit_events: EventWriter<AppExit>,
) {
    // Each of the components in our query must be present
    // on an entity for it to be returned in our query.
    // This system will loop over the first entity spawned, but not the second.
    for (entity, mut life, falling, mut transform) in query.iter_mut() {
        // Our entity has touched the ground
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            // We're using the `Entity` information from our query
            // to ensure we're removing the `Falling` component from the correct entity
            commands.entity(entity).remove::<Falling>();

            // Falling from small heights shouldn't hurt players at all
            let damage = (falling.height - 3.0).max(0.0);
            // .0 accesses the first field of our Life(f32) tuple struct
            life.0 = (life.0 - damage).max(0.0);
            info!("Damage: {}", damage);
            // End the game as soon as the first entity has collided with the ground
            exit_events.send(AppExit);
        }
    }
}
```

If you'd like to see more tiny but cohesive examples like this, check out our [game examples](https://github.com/bevyengine/bevy/tree/latest/examples/game) on the Bevy GitHub repository.

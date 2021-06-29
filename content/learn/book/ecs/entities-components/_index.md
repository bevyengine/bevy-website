+++
title = "Entities have components"
weight = 1
template = "book-section.html"
page_template = "book-section.html"
+++

As we discussed in the introduction to this chapter, **entities** represent objects in your game world, whose data is stored in the form of components.

The very first thing we're going to want to do is define the components that we'd like to use.
To do so, we simply create Rust types (ensuring that they are `Send + Sync + 'static` so they can be sent across the threads safely) and pick a descriptive name for them.
On this page, we're going to create a simple little combat system, so lets start by defining some basic components.

```rust
// These are dataless "unit structs", which hold no data of their own
// In Bevy, these are useful for distinguishing similar entities or toggling behavior
// and are called "marker components"
struct Player;
struct Enemy;

// These simple components wrap an i8 in a tuple struct
struct Life(u8);
struct Attack(u8);
struct Defense(u8);

// We can store arbitrary data in our components, as long as it has a 'static lifetime
struct Name(String);
```

`Life`, `Attack` and `Defense` will almost always be added to our entities at the same time, so let's create a **bundle** (collection of components) to make them easier to work with.

```rust
#[derive(Bundle)]
struct CombatBundle {
	// Each field of this bundle corresponds to a type of component 
	// that will be inserted into the final entity
	// The field names are only used when instantiating the bundle;
    // only the types are retained in our ECS storage
    life: Life,
    attack: Attack,
    defense: Defense,
}
```

Now, let's get to work by spawning a player entity and an enemy entity for them to fight!

```rust
// This is a startup system that we add to our app
// It runs only once, before everything else has occurred
fn spawn_combatants(mut commands: Commands) {
    // Spawning the player
    commands
        .spawn_bundle(CombatBundle {
            life: Life(10),
            attack: Attack(5),
            defense: Defense(2),
        })
        .insert(Player);
    // Spawning the enemy
    commands
        .spawn_bundle(CombatBundle {
            life: Life(10),
            attack: Attack(5),
            defense: Defense(1),
        })
        .insert(Enemy);
}
```

**Commands** are used to perform tasks that require non-local access to the data in the ECS: things like spawning or despawning entities or adding or removing components.
You'll learn about the details in the [commands section of this chapter](../commands/_index.md).
In this case, these commands will create two entities in our `World`. The first will have the `Life`, `Attack`, `Defense` and `Player` components, while the second will have the `Enemy` component instead of the `Player` component.

Now that we have entities with some components in the game, lets give them some behavior using **systems**.

```rust
// This resource stores who is going to make the next attack
// Attacks alternate back and forth
enum NextAttack {
    Player,
    Enemy,
}

// Makes the next attack when Space is pressed
fn next_attack(
    // We need mutable access to Life, but not the Attack or Defense component
    mut player_query: Query<(&mut Life, &Attack, &Defense), (With<Player>, Without<Enemy>)>,
    // We're using the With<Enemy> and Without<Player> query filters to limit the entities returned
    mut enemy_query: Query<(&mut Life, &Attack, &Defense), (With<Enemy>, Without<Player>)>,
    mut next_attack: ResMut<NextAttack>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // Only do work when Space was pressed
    if keyboard_input.just_pressed(KeyCode::Space) {
        // We know that exactly one entity should be returned by each of these
        let (mut player_life, player_attack, player_defense) = player_query.single_mut().unwrap();
        let (mut enemy_life, enemy_attack, enemy_defense) = enemy_query.single_mut().unwrap();

        // Make the next attack by mutating the appropriate life value
        let damage = match *next_attack {
            NextAttack::Player => make_attack(player_attack, enemy_defense, &mut enemy_life),
            NextAttack::Enemy => make_attack(enemy_attack, player_defense, &mut player_life),
        };

        println!("An attack landed for {} damage!", damage);

        // The other side gets the next attack
        *next_attack = match *next_attack {
            NextAttack::Player => NextAttack::Enemy,
            NextAttack::Enemy => NextAttack::Player,
        }
    }
}

// Damage = attack - defense
// Damage dealt and life totals can never go below 0
fn make_attack(attack: &Attack, defense: &Defense, life: &mut Life) -> i8 {
    let damage = (attack.0 - defense.0).max(0);
    life.0 = (life.0 - damage).max(0);
    damage
}

// Reports the value of each health bar in the terminal after an attack has been made
fn report_health(query: Query<(&Name, &Life), Changed<Life>>) {
    // This query will only contain entities whose Life component
    // has been added or mutably accessed since the last time this system ran
    // due to the Changed<Life> filter
    for (name, life) in query.iter() {
        println!("{} has {} life left.", name.0, life.0);
    }
}
```

Finally, let's put this all together into a final app that we can run by adding our systems and resources to our app and sticking it in `main()`.

```rust
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // The player gets the first attack!
        .insert_resource(NextAttack::Player)
		// Startup systems run exactly once before ordinary systems
        .add_startup_system(spawn_combatants.system())
        .add_system(next_attack.system())
        .add_system(report_health.system())
        .run();
}

// These are dataless "unit structs", which hold no data of their own
// In Bevy, these are useful for distinguishing similar entities or toggling behavior
// and are called "marker components"
struct Player;
struct Enemy;

// These simple components wrap an i8 in a tuple struct
struct Life(u8);
struct Attack(u8);
struct Defense(u8);

// We can store arbitrary data in our components, as long as it has a 'static lifetime
struct Name(String);

#[derive(Bundle)]
struct CombatBundle {
	// Each field of this bundle corresponds to a type of component 
	// that will be inserted into the final entity
	// The field names don't matter: only the types are retained in our ECS storage
    life: Life,
    attack: Attack,
    defense: Defense,
}

// This is a startup system that we add to our app
// It runs only once, before everything else has occurred
fn spawn_combatants(mut commands: Commands) {
    // Spawning the player
    commands
        .spawn_bundle(CombatBundle {
            life: Life(10),
            attack: Attack(5),
            defense: Defense(2),
        })
        .insert(Name("Gallant".to_string()))
        .insert(Player);
    // Spawning the enemy
    commands
        .spawn_bundle(CombatBundle {
            life: Life(10),
            attack: Attack(5),
            defense: Defense(1),
        })
        .insert(Name("Goofus".to_string()))
        .insert(Enemy);
}

// This resource stores who is going to make the next attack
// Attacks alternate back and forth
enum NextAttack {
    Player,
    Enemy,
}

// Makes the next attack when Space is pressed
fn next_attack(
    // We need mutable access to Life, but not the Attack or Defense component
    mut player_query: Query<(&mut Life, &Attack, &Defense), (With<Player>, Without<Enemy>)>,
    // We're using the With<Enemy> and Without<Player> query filters to limit the entities returned
    mut enemy_query: Query<(&mut Life, &Attack, &Defense), (With<Enemy>, Without<Player>)>,
    mut next_attack: ResMut<NextAttack>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // Only do work when Space was pressed
    if keyboard_input.just_pressed(KeyCode::Space) {
        // We know that exactly one entity should be returned by each of these
        let (mut player_life, player_attack, player_defense) = player_query.single_mut().unwrap();
        let (mut enemy_life, enemy_attack, enemy_defense) = enemy_query.single_mut().unwrap();

        // Make the next attack by mutating the appropriate life value
        let damage = match *next_attack {
            NextAttack::Player => make_attack(player_attack, enemy_defense, &mut enemy_life),
            NextAttack::Enemy => make_attack(enemy_attack, player_defense, &mut player_life),
        };

        println!("An attack landed for {} damage!", damage);

        // The other side gets the next attack
        *next_attack = match *next_attack {
            NextAttack::Player => NextAttack::Enemy,
            NextAttack::Enemy => NextAttack::Player,
        }
    }
}

// Damage = attack - defense
// Damage dealt and life totals can never go below 0
fn make_attack(attack: &Attack, defense: &Defense, life: &mut Life) -> i8 {
    let damage = (attack.0 - defense.0).max(0);
    life.0 = (life.0 - damage).max(0);
    damage
}

// Reports the value of each health bar in the terminal after an attack has been made
fn report_health(query: Query<(&Name, &Life), Changed<Life>>) {
    // This query will only contain entities whose Life component
    // has been added or mutably accessed since the last time this system ran
    // due to the Changed<Life> filter
    for (name, life) in query.iter() {
        println!("{} has {} life left.", name.0, life.0);
    }
}
```

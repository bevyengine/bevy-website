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

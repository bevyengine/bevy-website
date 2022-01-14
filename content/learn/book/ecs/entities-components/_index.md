+++
title = "Entities have components"
weight = 1
template = "book-section.html"
page_template = "book-section.html"
+++

As we discussed in the introduction to this chapter, **entities** represent objects in your game world, whose data is stored in the form of strongly-typed components.

## Spawning and despawning entities

Before you can do much of anything in Bevy, you'll need to **spawn** your first entity, adding it to the app's {{rust_type(type="struct" crate="bevy_ecs" name="World")}}.
Once entities exist, they can likewise be despawned, deleting all of the data stored in their components and removing it from the world.

Generally, you will be adding and removing entities (and modifying which components they have) using [commands](../commands/_index.md).

```rust
// Commands store a queue of actions that modify the World,
// which are collected and processed at the end of each stage
fn spawning_system(mut commands: Commands){
    // Spawn an entity with no components
    commands.spawn();

    // Spawn a second entity with no components, but return the `Entity` identifier
    let my_entity = commands.spawn().id();

    // Use the stored identifier to then despawn the second entity
    commands.despawn(my_entity);
}
```

You can read about all the details of commands [later in this chapter](../commands/_index.md)).

## Working with components

Entities are entirely bare when they're spawned: they contain no data other than their unique {{rust_type(type="trait" crate="bevy_ecs" mod = "entity" name="Entity" no_mod = "true")}} identifier.
This of course is not very useful, so let's discuss how we can add and remove components to them which store data and enable behavior through systems.

### Defining components

To define a component type, we simply implement the {{rust_type(type="trait" crate="bevy_ecs" mod = "component" name="Component" no_mod = "true")}} trait to a Rust type of our choice.
You will almost always want to use the `#[derive(Component)]` macro to do this for you; which quickly and reliably generates the correct trait code for the trait.
Any underlying component data must be `Send + Sync + 'static` (enforced by the [trait bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax) on {{rust_type(type="trait" crate="bevy_ecs" mod = "component" name="Component" no_mod = "true")}}).
This ensures that the data can be sent across the threads safely and allows our [type reflection tools](https://github.com/bevyengine/bevy/tree/main/crates/bevy_reflect) to work correctly.

With the theory out of the way, let's define some components!

```rust
// This is a dataless "unit struct", which holds no data of its own.
// See https://doc.rust-lang.org/rust-by-example/custom_types/structs.html for beginner-level docs on structs.
// In Bevy, unit structs are commonly used as "marker components",
// which is a behavioral label given to components that toggle on/off certain behavior
// by allowing us to include or exclude entities with particular marker components from our systems
#[derive(Component)]
struct Combatant;

// These simple components wrap a u8 in a tuple struct
#[derive(Component)]
struct Life(u8);
#[derive(Component)]
struct Attack(u8);
#[derive(Component)]
struct Defense(u8);

// Here, we use a tuple struct to store 2 ordered pieces of data
#[derive(Component)]
struct Position(i32, i32);

// Naming your components' fields,
// makes them easier and safer to refer to
#[derive(Component)]
struct Stats {
    strength: u8,
    dexterity: u8,
    intelligence: u8,
}

// Enum components are great for storing mutually exclusive states
#[derive(Component)]
enum Allegiance {
    Friendly,
    Neutral,
    Hostile
}

// Owned data types (like `String`) are much easier to use in the ECS
// than those with references (like `&str`) due to 'static requirements
#[derive(Component)]
struct Name(String);
```

### Spawning entities with components

Now that we have some components defined, let's try adding them to our entities using {{rust_type(type="trait" crate="bevy_ecs" mod = "system" name="EntityCommands" method = "insert" no_mod = "true")}}.

```rust
fn spawn_combatants_system(mut commands: Commands) {
    commands
        .spawn()
        // This inserts a dataless `Combatant` component into the entity we're spawning
        .insert(Combatant)
        // We configure starting component values by passing in concrete instances of our types
        .insert(Life(10))
        // By chaining .insert method calls like this, we continue to add more components to our entity
        .insert(Attack(5))
        .insert(Defense(2))
        .insert(Position(0, 0))
        // Instances of named structs are constructed with {field_name: value}
        .insert(Stats {
            strength: 15,
            dexterity: 10,
            intelligence: 8,
        })
        // Instances of enums are created by picking one of their variants
        .insert(Allegiance::Friendly)
        .insert(Name("Gallant".to_string()));

    // We've ended our Commands method chain using a ;,
    // and so now we can create a second entity
    // by calling .spawn() again
    commands
        .spawn()
        .insert(Combatant)
        .insert(Life(10))
        .insert(Attack(5))
        .insert(Defense(1))
        .insert(Position(0, 5))
        .insert(Stats {
            strength: 17,
            dexterity: 8,
            intelligence: 6,
        })
        .insert(Allegiance::Hostile)
        .insert(Name("Goofus".to_string()));
}
```

### Adding and removing components

Once an entity is spawned, you can use commands to add and remove components from them dynamically.

```rust

#[derive(Component)]
struct InCombat;

// This query returns the `Entity` identifier of all entities
// that have the `Combatant` component but do not yet have the `InCombat` component
fn start_combat_system(query: Query<Entity, (With<Combatant>, Without<InCombat>>, mut commands: Commands){
    for entity in query.iter(){
        // The component will be inserted at the end of the current stage
        commands.entity(entity).insert(InCombat);
    }
}

// Now to undo our hard work
fn end_combat_system(query: Query<Entity, (With<Combatant>, With<InCombat>>, mut commands: Commands){
    for entity in query.iter(){
        // The component will be removed at the end of the current stage
        commands.entity(entity).remove(InCombat);
    }
}
```

## Bundles

As you might guess, the one-at-a-time component insertion syntax can be both tedious and error-prone as your project grows.
To get around this, Bevy abstracts these patterns using **bundles**: named and typed collections of components.
These are implemented by adding the {{rust_type(type="trait" crate="bevy_ecs" mod = "bundle" name="Bundle" no_mod = "true")}}  trait to a struct; turning each of its fields into a distinct component on your entity when they are inserted.

Let's try rewriting that code from above, by grouping commonly associated components together into a bundle.

```rust
#[derive(Bundle)]
struct CombatantBundle {
    // Adding a field of a given type to our bundle
    // results in a component of that type with that value
    // being added to our entity
    combatant: Combatant
    life: Life,
    attack: Attack,
    defense: Defense,
    position: Position,
    stats: Stats,
    allegiance: Allegiance,
}

// We can add new methods to our bundle type that return Self
// to create principled APIs for entity creation.
// The Default trait is the standard tool for creating
// new struct instances without configuration 
impl Default for CombatantBundle {
    fn default() -> Self {
        CombatantBundle {
            combatant: Combatant,
            life: Life(10),
            attack: Attack(5),
            defense: Defense(1),
            position: Position(0, 0),
            stats: Stats {
                strength: 10,
                dexterity: 10,
                intelligence: 10,
            }
            allegiance: Allegiance::Neutral,
        }
    }
}

fn spawn_combatants_system(mut commands: Commands) {
    commands
        .spawn()
        // We're using struct-update syntax to modify the instance
        // of `CombatantBundle` returned by its default() method
        // See the page on Rust Tips and Tricks at the end of this chapter for more info!
        .insert_bundle(CombatantBundle{
            defense: Defense(2),
            stats: Stats {
                strength: 15,
                dexterity: 10,
                intelligence: 8,
            }
            allegiance: Allegiance::Friendly,
            ..Default::default()
        })
        // We can continue to chain more .insert or .insert_bundle methods
        // to add more components and extend behavior
        .insert(Name("Gallant".to_string()));
    
    commands
        // .spawn_bundle(my_bundle) is just syntactic sugar for 
        // .spawn().insert_bundle(my_bundle)
        .spawn_bundle(CombatantBundle{
            stats: Stats {
                strength: 17,
                dexterity: 8,
                intelligence: 6,
            }
            position: Position(0, 5),
            allegiance: Allegiance::Hostile,
            ..Default::default()
        })
        .insert(Name("Goofus".to_string()));}
```

### Nested bundles

As your game grows further in complexity, you may find that you want to reuse various bundles across entities that share some but not all behavior.
You can use **nested bundles** to embed one bundle of components within another.
Be mindful; this can lead to overwrought, deeply nested code if overused.
Furthermore, bundles are [not currently checked](https://github.com/bevyengine/bevy/issues/2387) for duplicate component types.
Later components will overwrite new components of the same type.

With those caveats out of the way, let's take a look at the syntax.
In this example, we're converting the bundle above to a nested one.

```rust
#[derive(Bundle)]
struct AttackableBundle{
    life: Life,
    attack: Attack,
    defense: Defense,
}

#[derive(Bundle)]
struct CombatantBundle {
    combatant: Combatant
    // This attribute macro marks our attackable_bundle field as a bundle,
    // allowing Bevy to properly flatten it out when building the final entity
    #[bundle]
    attackable_bundle: AttackableBundle,
    position: Position,
    stats: Stats,
    allegiance: Allegiance,
}

impl Default for CombatantBundle {
    fn default() -> Self {
        CombatantBundle {
            combatant: Combatant,
            attackable_bundle: AttackableBundle {
                life: Life(10),
                attack: Attack(5),
                defense: Defense(1),
            }
            position: Position(0, 0),
            stats: Stats {
                strength: 10,
                dexterity: 10,
                intelligence: 10,
            }
            allegiance: Allegiance::Neutral,
        }
    }
}
```

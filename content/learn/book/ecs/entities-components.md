+++
title = "Entities Have Components"
insert_anchor_links = "right"
[extra]
weight = 1
status = 'hidden'
+++

**Entities** are the fundamental objects of your game world, whizzing around, storing cameras, being controlled by the player or tracking the state of a button.
On its own, the [`Entity`] type is a simple identifer: it has neither behavior nor data.
Components store this data, and define the overlapping categories that the entity belongs to.

Informally, we use the term "entity" to refer to the conceptual entry in our [`World`]: all of the component data with the correct identifier, although it's very rare to use all of the data for a single entity at once.
If you're an experienced programmer, you can reason about the [`World`] as something like a (very fast) [`HashMap`] from [`Entity`] to a collection of components.

[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html

## Spawning and Despawning Entities

Before you can do much of anything in Bevy, you'll need to **spawn** your first entity, adding it to the app's [`World`].
Once entities exist, they can likewise be despawned, deleting all of the data stored in their components and removing it from the world.

Spawning and despawning entities can have far-reaching effects, and so cannot be done immediately (unless you are using an [exclusive system](../exclusive-world-access)).
As a result, we must use [`Commands`], which queue up work to do later.

```rust,hide_lines=1
# use bevy::ecs::system::Commands;
// The `Commands` system parameter allows us to generate commands
// which operate on the `World` once all of the current systems have finished running
fn spawning_system(mut commands: Commands){
    // Spawning a single entity with no components
    commands.spawn(());
    // Getting the `Entity` identifier of a new entity
    let my_entity = commands.spawn(()).id();
    // Selecting and then despawning the just-spawned second entity
    commands.entity(my_entity).despawn();
}
```

[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html

## Working With Components

Spawning an entity doesn't add any behavior or create a "physical object" in our game like it might in other engines.
Instead, all it does is provide us an [`Entity`] identifer for a collection of component data.

In order to make this useful, we need to be able to add, remove and modify component data for each entity.

### Defining Components

To define a component type, we simply implement the [`Component`] [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) for a Rust type of our choice.
You will almost always want to use the `#[derive(Component)]` [macro](https://doc.rust-lang.org/reference/attributes/derive.html) to do this for you; which quickly and reliably generates the correct trait implementation.

With the theory out of the way, let's define some components!

```rust,hide_lines=1
# use bevy::ecs::component::Component;
// This is a "unit struct", which holds no data of its own.
#[derive(Component)]
struct Combatant;

// This simple component wraps a u8 in a tuple struct
#[derive(Component)]
struct Life(u8);

// Naming your components' fields makes them easier to refer to
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
    Hostile
}
```

[`Component`]: https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html

### Spawning Entities With Components

Now that we have some components defined, let's try adding them to our entities using [`Commands`].

```rust,hide_lines=1-20
# use bevy::ecs::prelude::*;
# 
# #[derive(Component)]
# struct Combatant;
# 
# #[derive(Component)]
# struct Life(u8);
# 
# #[derive(Component)]
# struct Stats {
#     strength: u8,
#     dexterity: u8,
#     intelligence: u8,
# }
# 
# #[derive(Component)]
# enum Allegiance {
#     Friendly,
#     Hostile
# }
fn spawn_combatants_system(mut commands: Commands) {
    commands.spawn((
        // This inserts a data-less `Combatant` component into the entity we're spawning
        Combatant,
        // We configure starting component values by passing in concrete instances of our types
        Life(10),
        // By chaining .insert method calls like this, we continue to add more components to our entity
        // Instances of named structs are constructed with {field_name: value}
        Stats {
            strength: 15,
            dexterity: 10,
            intelligence: 8,
        },
        // Instances of enums are created by picking one of their variants
        Allegiance::Friendly,
    ));

    // We've ended our Commands method chain using a ;,
    // and so now we can create a second entity
    // by calling .spawn() again
    commands.spawn((
        Combatant,
        Life(10),
        Stats {
            strength: 17,
            dexterity: 8,
            intelligence: 6,
        },
        Allegiance::Hostile,
    ));
}
```

### Adding and Removing Components

Once an entity is spawned, you can use [`Commands`] to add and remove components from them dynamically.

```rust,hide_lines=1-4
# use bevy::ecs::prelude::*;
# 
# #[derive(Component)]
# struct Combatant;
#[derive(Component)]
struct InCombat;

// This query returns the `Entity` identifier of all entities
// that have the `Combatant` component but do not yet have the `InCombat` component
fn start_combat_system(query: Query<Entity, (With<Combatant>, Without<InCombat>)>, mut commands: Commands){
    for entity in query.iter() {
        // The component will be inserted at the end of the current stage
        commands.entity(entity).insert(InCombat);
    }
}

// Now to undo our hard work
fn end_combat_system(query: Query<Entity, (With<Combatant>, With<InCombat>)>, mut commands: Commands){
    for entity in query.iter() {
        // The component will be removed at the end of the current stage
        // It is provided as a type parameter,
        // as we do not need to know a specific value in order to remove a component of the correct type
        commands.entity(entity).remove::<InCombat>();
    }
}
```

Entities can only ever store one component of each type: inserting another component of the same type will instead overwrite the existing data.

## Bundles

As you might guess, the one-at-a-time component insertion syntax can be both tedious and error-prone as your project grows.
To get around this, Bevy allows you to group components into **component bundles**.
These are defined by deriving the [`Bundle`] trait for a struct; turning each of its fields into a distinct component on your entity when the bundle is inserted.

Let's try rewriting that code from above.

```rust,hide_lines=1-20
# use bevy::prelude::*;
#
# #[derive(Component)]
# struct Combatant;
# 
# #[derive(Component)]
# struct Life(u8);
# 
# #[derive(Component)]
# struct Stats {
#     strength: u8,
#     dexterity: u8,
#     intelligence: u8,
# }
# 
# #[derive(Component)]
# enum Allegiance {
#     Friendly,
#     Hostile
# }
#[derive(Bundle)]
struct CombatantBundle {
    combatant: Combatant,
    life: Life,
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
            stats: Stats {
                strength: 10,
                dexterity: 10,
                intelligence: 10,
            },
            allegiance: Allegiance::Hostile,
        }
    }
}

fn spawn_combatants_system(mut commands: Commands) {
    commands.spawn((
        // We're using struct-update syntax to modify
        // the instance of `CombatantBundle` returned by its default() method
        // See the page on Rust Tips and Tricks at the end of this chapter for more info!
        CombatantBundle{
            stats: Stats {
                strength: 15,
                dexterity: 10,
                intelligence: 8,
            },
            allegiance: Allegiance::Friendly,
            ..default()
        },
    ));
    
    commands.spawn((
        CombatantBundle{
            stats: Stats {
                strength: 17,
                dexterity: 8,
                intelligence: 6,
            },
            allegiance: Allegiance::Hostile,
            ..default()
        },
    ));
}
```

[`Bundle`]: https://docs.rs/bevy/latest/bevy/ecs/bundle/trait.Bundle.html

### Nested Bundles

As your game grows further in complexity, you may find that you want to reuse various bundles across entities that share some but not all behavior.
One of the tools you can use to do so is **nested bundles**; embedding one bundle of components within another.
Try to stick to a single layer of nesting at most; multiple layers of nesting can get quite confusing.
Including duplicate components in your bundles in this way will cause a panic.

With those caveats out of the way, let's take a look at the syntax by converting the bundle above to a nested one by creating a bundle of components that deal with related functionality.

```rust,hide_lines=1-19
# use bevy::prelude::*;
#
# #[derive(Component)]
# struct Combatant;
# 
# #[derive(Component)]
# struct Life(u8);
#
# #[derive(Component)]
# struct Attack(u8);
#
# #[derive(Component)]
# struct Defense(u8);
#
# #[derive(Component)]
# enum Allegiance {
#     Friendly,
#     Hostile
# }
#[derive(Bundle)]
struct AttackableBundle{
    life: Life,
    attack: Attack,
    defense: Defense,
}

#[derive(Bundle)]
struct CombatantBundle {
    combatant: Combatant,
    // The #[bundle] attribute marks our attackable_bundle field as a bundle (rather than a component),
    // allowing Bevy to properly flatten it out when building the final entity
    // #[bundle] // commenting out to make code validator run; should be looked at TODO
    attackable_bundle: AttackableBundle,
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
            },
            allegiance: Allegiance::Hostile,
        }
    }
}
```

## Component Design

Over time, the Bevy community has converged on a few standard pieces of advice for how to structure and define component data:

- try to keep your components relatively small
  - combine common functionality into bundles, not large components
  - small modular systems based on common behavior work well
  - reducing the amount of data stored improves cache performance and system-parallelism
  - keep it as a single component if you need to maintain invariants (such as current life is always less than or equal to max life)
  - keep it as a single component if you need methods that operate across several pieces of data (e.g. computing the distance between two points)
- simple methods on components are a good tool for clean, testable code
  - logic that is inherent to how the component works (like rolling dice or healing life points) is a great fit
  - logic that will only be repeated once generally belongs in systems
  - methods make it easier to understand the actual gameplay logic in your systems, and fix bugs in a single place
- marker components are incredibly valuable for extending your design
  - it is very common to want to quickly look for "all entities that are a `Tower`", or "all entities that are `Chilled`
  - filtering by component presence/absence is (generally) faster and clearer than looping through a list of boolean values
  - try to model meaningful groups at several levels of abstraction / along multiple axes: e.g. `Unit`, `Ant`, `Combatant`
- enum components are very expressive, and help reduce bugs
  - enums can hold different data in each variant, allowing you to capture information effectively
  - if you have a fixed number of options for a value, store it as an enum
- implementing traits like [`Add`] or [`Display`] can provide useful behavior in an idiomatic way
- use [`Deref`] and [`DerefMut`] for tuple structs with a single item ([newtypes])
  - this allows you to access the internal data with `*my_component` instead of `my_component.0`
  - more importantly, this allows you to call methods that belong to the wrapped type directly on your component
- define builder methods for your [`Bundle`] types that return [`Self`]
  - this is useful to define a friendly interface for how entities of this sort tend to vary
  - not as useful as you might hope for upholding invariants; components will be able to be accidentally modified independently later
- use [struct update syntax] to modify component bundles
  - [`..default()`] is a particularly common idiom, to modify a struct from its default values
- consider definining traits for related components
  - this allows you to ensure a consistent interface
  - this can be very powerful in combination with generic systems that use trait bounds

[`Add`]: https://doc.rust-lang.org/std/ops/trait.Add.html
[`Display`]: https://doc.rust-lang.org/std/path/struct.Display.html
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[`Self`]: https://doc.rust-lang.org/reference/paths.html#self-1
[`..default()`]: https://docs.rs/bevy/latest/bevy/prelude/fn.default.html
[newtypes]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
[struct update syntax]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax

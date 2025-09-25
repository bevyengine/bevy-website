+++
title = "Entities Have Components"
insert_anchor_links = "right"
[extra]
weight = 1
status = 'hidden'
+++

**Entities** are the fundamental objects of your game world, whizzing around, storing cameras, being controlled by the player or tracking the state of a button.
On its own, the [`Entity`] type is a simple identifier: it has neither behavior nor data.
Components store this data, and define the overlapping categories that the entity belongs to.

Informally, we use the term "entity" to refer to the conceptual entry in our [world]: all of the component data with the correct identifier, although it's very rare to use all of the data for a single entity at once.
If you're an experienced programmer, you can reason about the [world] as something like a (very fast) [`HashMap`] from [`Entity`] to a collection of components.

Internally, [`Entity`] is roughly shaped like a `u64`, with arbitrary (unique) bits.
While it is possible to work with the [`Entity`] type directly, it should be treated as an opaque, black box key.
Bevy makes no guarantees that exact entity assignment or storage behavior will be stable across any version boundary.

[world]: /learn/book/storing-data/world
[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html

## A Different Perspective on Entities

There's another way to think about entities and components: the Bevy game world operates much like a traditional database application. Metaphorically, the "entities" represent rows in a table, and the "components" are much like columns. A "query" lets you read and manipulate the contents of the database by giving you access to a subset of the rows and columns in the table.

The ECS approach can take some getting used to, especially for someone coming from a traditional OOP (Object-Oriented Programming) background. However, the advantage of this approach is the ability to perform massive bulk operations efficiently within extremely large and complex game worlds.

## Spawning and Despawning Entities

Before you can do much of anything in Bevy, you'll need to **spawn** your first entity, adding it to the app's [`World`].
Once entities exist, they can likewise be despawned, deleting all of the data stored in their components and removing it from the world.

Spawning and despawning entities can have far-reaching effects, and so cannot be done immediately (unless you are using an [exclusive system]).
As a result, we must use [`Commands`], which queue up work to do later.

```rust,hide_lines=1
# use bevy::ecs::system::Commands;
// The `Commands` system parameter allows us to issue commands
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

[exclusive system]: ../../control-flow/systems/#exclusive-systems
[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html

## Working With Components

As mentioned in the [introduction section on components], a **Component** is a small, modular, reusable piece of data that can be attached to an entity.
Components are necessary to make individual entities useful.
Otherwise, entities are just identifiers that don't point to anything!

Spawning an entity doesn't add any behavior or create a "physical object" in our game like it might in other engines.
To make a basic cube, you'd probably need to add:

- A [`Transform`] component to store where the object is
- A [`Mesh`] component to store its geometry (in practice, actually a [`Handle`] to a [`Mesh`])
- A [`StandardMaterial`] to define how that mesh should be rendered (in practice, actually a [`Handle`] to a [`StandardMaterial`])
- Etc.

[introduction section on components]: ../../intro/the-three-letters#the-c-components
[`Transform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html
[`Mesh`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Mesh.html
[`Handle`]: https://docs.rs/bevy/latest/bevy/asset/enum.Handle.html
[`StandardMaterial`]: https://docs.rs/bevy/latest/bevy/pbr/struct.StandardMaterial.html

### Defining Components

To define a component type, we simply implement the [`Component`] [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) for a Rust type of our choice.
You will almost always want to use the `#[derive(Component)]` [macro](https://doc.rust-lang.org/reference/attributes/derive.html) to do this for you; which quickly and reliably generates the correct trait implementation.

Components come in a variety of tasty flavors:

```rust,hide_lines=1
# use bevy::ecs::component::Component;
// This is a "unit struct", which holds no data of its own.
// However, it can still be used in queries as a filter!
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

## Component Design

Over time, the Bevy community has converged on a few standard pieces of advice for how to structure and define component data:

- Try to keep your components relatively small
  - Common functionality can be handled by putting it on a shared [Required Component]
  - Small modular systems based on common behavior work well
  - Reducing the amount of data stored improves cache performance and system-parallelism
  - Group properties together within a single component if you need to maintain invariants (such as current life is always less than or equal to max life)
  - Additionally, group properties together within a single component if you need methods that operate across several pieces of data (e.g. computing the distance between two points)
- Simple methods on components are a good tool for clean, testable code
  - Logic that is inherent to how the component works (like rolling dice or healing life points) is a great fit
  - Logic that will only be repeated once generally belongs in systems
  - Methods make it easier to understand the actual gameplay logic in your systems, and fix bugs in a single place
- **Marker components** (using unit structs) are incredibly valuable for extending your design
  - It is very common to want to quickly look for "all entities that are a `Tower`", or "all entities that are `Chilled`"
  - Filtering by component presence/absence is (generally) faster and clearer than looping through a list of boolean values
  - Try to model meaningful groups at several levels of abstraction / along multiple axes: e.g. `Unit`, `Ant`, `Combatant`
- Enum components are very expressive, and help reduce bugs
  - Enums can hold different data in each variant, allowing you to capture information effectively
  - If you have a fixed number of options for a value, store it as an enum
- Implementing traits like [`Add`] or [`Display`] can provide useful behavior in an idiomatic way
- Use [`Deref`] and [`DerefMut`] for tuple structs with a single item ([newtypes])
  - This allows you to access the internal data with `*my_component` instead of `my_component.0`
  - More importantly, this allows you to call methods that belong to the wrapped type directly on your component
- Consider defining traits for related components
  - This allows you to ensure a consistent interface
  - This can be very powerful in combination with generic systems that use trait bounds

[Required Component]: ../required-components
[`Add`]: https://doc.rust-lang.org/std/ops/trait.Add.html
[`Display`]: https://doc.rust-lang.org/std/path/struct.Display.html
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[newtypes]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html

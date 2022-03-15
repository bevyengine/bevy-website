+++
title = "Entities, components and systems"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

Bevy is fundamentally powered by its ECS (Entity-Componenent System, a high-performance, modular [paradigm](https://ajmmertens.medium.com/ecs-from-tool-to-paradigm-350587cdf216) for organizing and manipulating data):
data is stored as components which belong to entities, and logic is executed by systems.

**Components** store data for a single entity in a strongly-typed fashion: defining "what" an entity is in a composable fashion.
Generally, components are kept quite small (to increase the flexibility of the design and reduce the amount of unneeded data that is fetched), and are grouped together in **component bundles** to enable complex behavior.

```rust
use bevy::prelude::*;
```

**Entities** are simply collections of components, and the [`Entity`] type is simply a unique identifier for that particular type: something like a name, URL or row number in a database.

```rust
use bevy::prelude::*;

fn setup(mut commands: Commands){
    // This creates a new entity
    commands.spawn()
      // Then, we add our first component to it
      .insert(Player)
      // And add more components to it, in the form of a bundle
      .insert_bundle(
          SpriteBundle {
              // Each of these fields are inserted as seperate components
              todo!(),
      });
}
```

In order to manipulate and act on this data, we must use systems.
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

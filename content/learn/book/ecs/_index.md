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
Once the systems are added to our app the **scheduler** takes in this information and automatically runs our systems: typically once during each pass of the **game loop**.

Bevy's scheduler is remarkably powerful: it uses the information about data access defined in our system parameters to automatically run systems in parallel.
By default, every system in a stage runs in parallel with every other system in that stage (as long as threads exist to take the work): the only rule is that systems which have the ability to *write* to a particular piece of data (such as a resource) cannot be run at the same time as other systems which read or write to that same data.
In Rust, this corresponds to mutable access, as declared by the use of `Query<&mut MyComponent>` or `ResMut<MyResource>`.

If you'd like to see how this all fits together, check out our [game examples](https://github.com/bevyengine/bevy/tree/latest/examples/game) to see a few simple but working demonstrations of how these ECS features interact in practice.

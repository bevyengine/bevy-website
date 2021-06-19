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
* the primary key of this database is the [`Entity`](https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html) identifier, which can use to look up specific entities using [`Query::get(my_entity)`](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Query.html#method.get)

Of course, this database is not very well-normalized: not all entities will have every component!
We can use this to specialize behavior between entities: only performing work on them in our systems if they have the correct combination of components.
You don't want to apply gravity to entities without a position in your world, and you're only interested in using the UI layout algorithm to control the layout of UI entities!

When we want to go beyond this tabular data storage, we can use **resources**: global singletons which store data in monolithic blobs.
You might use resources to store one-off bits of state like the game's score, use it to interface with other libraries, or store secondary data structures like indexes to augment your use of entity-component data.
Just like with components on entities, resources are accessed by type and you can only have one resource of each Rust type.

In order to actually perform logic on all of this data, we must use systems, which are automatically run by our scheduler.
**Systems** are Rust functions which request specific data from the world, as declared in their system parameters (function arguments): generally resources and entities that have a particular combination of components using queries.
Once the systems are added to our app the **scheduler** takes in this information and automatically runs our systems: typically once during each pass of the **game loop**.

Bevy's scheduler is remarkably powerful: it uses the information about data access defined in our system parameters to automatically run systems in parallel.
By default, every system in a stage runs in parallel with every other system in that stage (as long as threads exist to take the work): the only rule is that systems which have the ability to *write* to a particular piece of data (such as a resource) cannot be run at the same time as other systems which read or write to that same data.
In Rust, this corresponds to mutable access, as declared by the use of `Query<&mut MyComponent>` or `ResMut<MyResource>`.

On the next page, we'll create a simple "game" using the ECS so you can see how this all fits together.

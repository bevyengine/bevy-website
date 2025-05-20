+++
title = "The World"
insert_anchor_links = "right"
[extra]
weight = 3
+++

The [`World`] is a catch-all container for *stuff*. 
Entities, components, resources, assets -- all of these exist within a world.
It's where your data lives.

## The World as a Database
If you are familiar with SQL database terminology, you can think of a world as a database.
Just like a database, a world contains several tables called **Archetypes**.
Each [entity] is an entry within this database, stored as a row in on one of the archetype tables.
The components on an entity form the columns of the archetype table.
This allows archetype tables to be stored as dense arrays, containing only entities with exactly the same set of components.
However, it also means that entities must be moved between tables when components are added or removed.

## Using the World
With a `&World` reference, you can read anything out of the ecs. And with a `&mut World`, you can write to it as well.

[entity]: /learn/book/storing-data/entities-components
[`World`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html

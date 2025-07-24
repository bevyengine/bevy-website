+++
title = "Queries"
insert_anchor_links = "right"
[extra]
weight = 4
status = 'hidden'
+++

Queries are your primary tool for interacting with the Bevy world,
allowing you to carefully and efficiently read and write component data from matching entities.

## Anatomy of a query

To understand how queries work in a bit more detail, let's take a look at the anatomy of a [`Query`].
The [`Query`] type has two generics: `D`, which must implement the [`QueryData`] trait,
and `F`, which must implement the [`QueryFilter`] trait.
`D` describes "which data should I access", while `F` describes "how should I restrict the returned entities".
Only entities which match *all* of the terms in `D` *and* `F` will be included in your query.

When we write `Query<&Life, With<Player>>`, we're supplying these generics, setting `D` to `&Life` and `F` to `With<Player>`,
separated by a comma.
Bevy uses the information in the [`QueryData`] and [`QueryFilter`] traits,
along with the [`WorldQuery`] supertrait, in order to know how to lookup components of
the correct type in the world and supply them to your system via [dependency injection].

If we don't want to fetch any data, or perform any filtering,
we can use `()`, Rust's "unit type" in the place of `D` or `F`.
The `F: QueryFilter` argument defaults to `()`, allowing us to conveniently write
`Query<&Life>`, rather than spelling out that we don't want to filter via `Query<&Life, ()>`.

To access more than one component at once, or filter by multiple predicates at the same time,
we can combine [`QueryData`] or [`QueryFilter`] types by putting them inside of a [tuple],
wrapping them with parentheses.

If we have two components, `Life` and `Mana`, we can fetch all entities with the `Life` component
by requesting `Query<&Life>` (if we want the data), or `Query<(), With<Life>>` (if we don't need the data).
We can do the same thing for `Mana`, and we could add two queries in the same system, one for `Life`, and one for `Mana`,
if we wanted to examine these two lists side by side.

But to fetch entities with *both* the life and mana components,
we would need:

- `Query<(&Life, &Mana)>`, giving us access to the data of both components
- `Query<&Life, With<Mana>>` or `Query<&Mana, With<Life>>`, accessing the data of one and filtering on the presence of the other
- `Query<(), (With<Life>, With<Mana>)>` to filter on both components

These tuples can be extended to multiple elements: `Query<(&Life, &Mana, &Energy)>` and so on works just fine!
To swap from the default "and" logic, where all of the components must be present, to "or" logic,
where any of the components can be present, you can use:

- `Query<Option<&Life>>`, for an [`Option`] that contains the component value if present and nothing if it is absent
- `Query<AnyOf<(&Life, &Mana)>>` which acts as a wrapper for multiple `Option` `QueryData` types
- `Query<Has<Life>>`, which contains `true` if the component is present, and `false` if the component is absent
- `Query<(), Or<(With<Life>, With<Mana>)>>`, which combines query filters via an `Or` relationship

As you can see, Bevy's type-driven querying can be quite expressive and elaborate!
Don't worry though: most of your queries will be quite simple, requesting a few pieces of data with some filter.

[dependency injection]: https://en.wikipedia.org/wiki/Dependency_injection

## Mutable and immutable query data

The most useful way to modify a query is to change whether we're requesting the data "immutably" (read-only) or "mutably" (read-write).
We can do that by changing `Query<&Life>` to `Query<&mut Life>` (pronounced "ref Life" and "ref mute Life" respectively).
The ampersand is Rust's read-only [reference] indicator,
while `&mut` is for mutable references, making it easy to remember the syntax once you're familiar with Rust.

{% callout(type="info") %}

You can include multiple queries within a single system, allowing you to access component data in more flexible ways.
But if Bevy is handing out mutable references to component data in safe Rust, how does it ensure that users don't
invoke undefined behavior due to the forbidden [mutable aliasing]?

Bevy protects against this by examining the [`Access`] of each of the system params in each systems,
and then panicking if they could conflict.
If you run into this, you'll be pointed to the [B0002] error page,
which has advice on how to fix and avoid this problem.

{% end %}

By changing our [`QueryData`] terms in this way, we change the type of [query item] returned,
changing the type of object we get when we iterate over our queries.
`Query<&Life>` corresponds to a `&Life`, giving us direct read-only access to data inside of our world.
However, you may have noticed that `Query<&mut Life>` returns a [`Mut<Life>`]. Why?

This [smart pointer] wraps a `&mut T` and allows Bevy to automatically detect changes.
While this is talked about in more depth in the chapter on [change detection],
it's helpful to know that [`Changed`] and [`Added`] are both query filters.

[reference]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
[query item]: https://dev-docs.bevy.org/bevy/ecs/query/trait.QueryData.html#associatedtype.Item
[`Mut<Life>`]: https://dev-docs.bevy.org/bevy/ecs/change_detection/struct.Mut.html
[smart pointer]: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
[change detection]: ../control-flow/change-detection.md
[mutable aliasing]: https://doc.rust-lang.org/rust-by-example/scope/borrow/alias.html
[`Access`]: https://dev-docs.bevy.org/bevy/ecs/query/struct.Access.html
[B0002]: https://bevy.org/learn/errors/b0002/

## Accessing data on specific entities

While many systems will operate by simply iterating over all of the entities in a query,
it is often helpful to look up (and possibly mutate) the data of a specific entity.

This is fast and easy to do, using [`Query::get`] and its mutable sibling [`Query::get_mut`].
These respect the query data and query filters of the query they are called on,
making them an extremely powerful tool.

Of course, this begs the question: where do we get the [`Entity`] identifier.
The simplest way to get this information is to record it when spawning an entity.

```rust
# use bevy::prelude::*;

#[derive(Resource)]
struct SelectedEntity(Entity);

fn spawn_selected_entity(mut commands: Commands, mut selected_entity: ResMut<SelectedEntity>){
    // .id() records the allocated identifier of the entity that is about to be spawned
    let special_entity_id = commands.spawn(Name::new("Throckmorton")).id();
    special_entity.0 = special_entity_id;
}

fn print_selected_entity_name(query: Query<&Name>, special_entity: Res<SelectedEntity>){
    if let Ok(name) = query.get(special_entity.0){
        info!("{name} is selected.");
    } else {
        warn!("Selected entity {} has been despawned, or does not have a Name component", special_entity.0);
    }
}
```

As the example shows, you can store the retrieved `Entity` identifiers inside of components or resources,
creating flexible connections between entities and lookup tables.
Inside of Bevy itself, this pattern is combined with [hooks] to make it more robust
and exposed to users as [relations].

The other common way to get an [`Entity`] is to take advantage of its [`QueryData`] implementation,
allowing you to determine the identifier for entities in queries that you are iterating over.

```rust
# use bevy::prelude::*;
#
#[derive(Component)]
struct Enemy;

// Note: no `&` for Entity as a QueryData!
fn despawn_all_enemies(enemies: Query<Entity, With<Enemy>>, mut commands: Commands){
    for enemy_entity in enemies.iter(){
        commands.entity(enemy_entity).despawn();
    }
}
```

[hooks]: ../control-flow/hooks.md
[relations]: ./relations.md

## Working with singleton entities

QUERY::SINGLE

CONTRAST TO SINGLE SYSTEM PARAM

## Accessing multiple items from the same query

QUERY::GET_MANY

## Disabling entities

DEFAULT QUERY FILTERS

DISABLED COMPONENT

## Working with complex queries

DERIVE QUERYDATA / QUERYFILTER

USE SYSTEMPARAM DERIVE INSTEAD

CONTRAST TO TYPE ALIASES

## Accessing all data from a single entity via queries

- change detection
- Mut and Ref
- EntityMut, EntityRef, FilteredEntityMut, FilteredEntityRef, EntityMutExcept, EntityRefExcept

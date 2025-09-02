+++
title = "Queries"
insert_anchor_links = "right"
[extra]
weight = 4
status = 'hidden'
+++

Queries are your primary tool for interacting with the Bevy world,
allowing you to efficiently read and write component data from entities.
Queries create a filtered "view" into the metaphorical database that makes up our ECS.
With that view, you can iterate over the requested components, ask what the "row number" (`Entity`) is for each element,
or fetch the matching components for a particular `Entity` value.

We introduced queries briefly in our [introduction](../intro/) to Bevy: if you're brand to new Bevy or ECS, start there.

## Anatomy of a query

To understand how queries work in a bit more detail, let's take a look at the anatomy of a [`Query`].
The [`Query<D, F>`] type has two [generic type parameters]: `D`, which must implement the [`QueryData`] trait,
and `F`, which must implement the [`QueryFilter`] trait.

`D` describes "which data should I access", while `F` describes "how should I restrict the returned entities".
Only entities which match *all* of the terms in `D` *and* `F` will be included in your query.

When we write `Query<&Life, With<Player>>`, we're supplying these generics, setting `D` to `&Life` and `F` to `With<Player>`,
separated by a comma.
Bevy uses the information in the [`QueryData`] and [`QueryFilter`] traits,
along with the [`WorldQuery`] supertrait, to look up components of
the correct type in the world and supply them to your system via [dependency injection].

If we don't want to fetch any data, or perform any filtering,
we can use `()`, Rust's ["unit type"] in place of `D` or `F`.

Inside the `Query` type, the `F: QueryFilter` generic defaults to `()`, allowing us to avoid explicitly writing `Query<&Life, ()>` when we don't want to filter. This simplified, filter-less form of query looks like `Query<&Life>`, which will fetch all instances of the `Life` component in the world.

To access more than one component at once, or add multiple filters at the same time,
we can combine [`QueryData`] or [`QueryFilter`] types by putting them inside of a [tuple],
wrapping them with parentheses.

[generic type parameters]: (https://doc.rust-lang.org/book/ch10-01-syntax.html)
["unit type"]: https://doc.rust-lang.org/core/primitive.unit.html

### Accessing multiple components at once

Let's say we have three components: `Energy`, `Life`, and `Mana`.

As shown above, we can grab a list of all entities with the `Life` component with `Query<&Life>`.
But what if we wanted to see the `Life` and `Mana` of our entities at the same time?

We need to somehow communicate that the `D` generic of our `Query` should cover both `&Life` and `&Mana`.
Bevy uses [tuples] as the syntax for this, wrapping all of the types that we want to combine in parentheses.
`&Life` becomes `(&Life, &Mana)`, which we slot into the `D` generic to become `Query<(&Life, &Mana)>`.

We can iterate over this query like so:

```rust,hide_lines=1-2
# use bevy::prelude::*;
#
#[derive(Component)]
struct Life {
    value: u32
}


#[derive(Component)]
struct Mana {
    value: u32
}

fn life_and_mana_system(query: Query<(&Life, &Mana)>){
    // This pattern is called "destructuring",
    // and is very convenient when working with queries.
    // The type annotations (": &Life") are optional;
    // they're shown here for clarity!
    for (life: &Life, mana: &Mana) in query.iter(){
        todo!();
    } 
}
```

When we use queries with multiple terms like this, the critical thing is that we're accessing the `Life` and `Mana` components
on the same entity whenever we iterate over our query.
This allows you to perform operations on entities as a whole: relating their properties in flexible but efficient ways.

We can add more terms to this tuple to request more components at once, like `Query<(&Life, &Mana, &Energy)>`.
Up to 16 elements can be combined in this way.
While this should be plenty, Bevy is currently limited by the lack of [variadic generics] in Rust itself.

This works for `QueryFilter` terms too: if we set `F` to `(With<Life>, Without<Mana>)`,
we can use it like `Query<&Energy, (With<Life>, Without<Mana>)>`.
This means "get me the energy component of all entities that have a life component but do not have a mana component".

Combining multiple terms in your queries like this should be the first tool you reach for when trying to implement more complex logic in Bevy.

[tuples]: https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
[variadic generics]: https://poignardazur.github.io/2025/06/07/report-on-variadics-rustweek/

### Optional Components

Sometimes, you want to swap from the default "and" logic, where all of the components must be present, to "or" logic, where any of the components can be present. To do so, you can use `Option` and a few special types:

- `Query<Option<&Life>>`, for an [`Option`] that contains the component value if present and nothing if it is absent
- `Query<AnyOf<(&Life, &Mana)>>` which acts as a wrapper for multiple `Option` `QueryData` types
- `Query<Has<Life>>`, which contains `true` if the component is present, and `false` if the component is absent
- `Query<(), Or<(With<Life>, With<Mana>)>>`, which combines query filters via an `Or` relationship
  - Using `()` in the `QueryData` field means that no data will be fetched: the only information retrieved is whether or not the query contains a given entity.

As you can see, Bevy's type-driven querying can be quite expressive and elaborate!
Don't worry, though: most of your queries will be quite simple, requesting a few pieces of data with a simple filter.

[dependency injection]: https://en.wikipedia.org/wiki/Dependency_injection

## Mutable and immutable query data

The most useful way to modify a query is to change whether we're requesting the data "immutably" (read-only) or "mutably" (read-write).
We can do that by changing `Query<&Life>` to `Query<&mut Life>` (pronounced "ref Life" and "ref mute Life", respectively).
The ampersand is Rust's read-only [reference] indicator,
while `&mut` is for mutable references, making it easy to remember the syntax once you're familiar with Rust.

{% callout(type="info") %}

You can include multiple queries within a single system, allowing you to access component data in more flexible ways.
But, if Bevy is handing out mutable references to component data in safe Rust, how does it ensure that users don't
invoke undefined behavior due to the forbidden [mutable aliasing]?

Bevy protects against this by examining the [`Access`] of each of the system params in each systems,
then panicking if they could conflict.
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

```rust,hide_lines=1-2
# use bevy::prelude::*;
#
#[derive(Resource)]
struct SelectedEntity(Entity);

fn spawn_selected_entity(mut commands: Commands, mut selected_entity: ResMut<SelectedEntity>) {
    // .id() records the allocated identifier of the entity that is about to be spawned
    let special_entity_id = commands.spawn(Name::new("Throckmorton")).id();
    special_entity.0 = special_entity_id;
}

fn print_selected_entity_name(query: Query<&Name>, special_entity: Res<SelectedEntity>) {
    if let Ok(name) = query.get(special_entity.0){
        info!("{name} is selected.");
    } else {
        warn!(
            "Selected entity {} has been despawned, or does not have a Name component", 
            special_entity.0
        );
    }
}
```

As this example shows, you can store the retrieved `Entity` identifiers inside of components or resources,
creating flexible connections between entities and lookup tables.
Inside of Bevy itself, this pattern is combined with [hooks] to make it more robust
and exposed to users as [relations].

The other common way to get an [`Entity`] is to take advantage of its [`QueryData`] implementation,
allowing you to determine the identifier for entities in queries that you are iterating over.

```rust,hide_lines=1-2
# use bevy::prelude::*;
#
#[derive(Component)]
struct Enemy;

// Note: `Entity` is the correct form of QueryData: no `&`!
fn despawn_all_enemies(enemies: Query<Entity, With<Enemy>>, mut commands: Commands) {
    for enemy_entity in enemies.iter() {
        commands.entity(enemy_entity).despawn();
    }
}
```

[hooks]: ../control-flow/hooks.md
[relations]: ./relations.md

## Working with singleton entities

From time-to-time, you may find yourself writing systems that expect there to be only a single matching entity.
This might be a player, the sun, or something more abstract like your camera.

While you could iterate over a query of length one, this can be confusing to read and feel a bit silly.
To make working with these patterns more comfortable, Bevy provides two tools:
`Query::single` and the `Single` system param.
Let's try writing the same simple system in each of the three ways.

```rust,hide_lines=1-2
# use bevy::prelude::*;
#
#[derive(Component)]
struct Life(u32);

fn kill_player_when_dead_query_iter(player_query: Query<(Entity, &Life), With<Player>>, mut commands: Commands) {
    for (player_entity, player_life) in player_query.iter() {
        if player_life.0 == 0 {
            commands.entity(player_entity).despawn();
        }
    }
}

fn kill_player_when_dead_query_single(player_query: Query<(Entity, &Life), With<Player>>, mut commands: Commands) {
    let Ok((player_entity, player_life)) = player.single() else {
        // We could instead use the ? operator and return an error;
        // see the error handling chapter        
        return;
    }

    if player_life.0 == 0 {
        commands.entity(player_entity).despawn();
    }
}

// This system will be skipped unless there is exactly one matching player entity
// so there's no need to handle the error case in the system
fn kill_player_when_dead_query_single(player: Single<(Entity, &Life), With<Player>>, mut commands: Commands) {
    // We have to dereference out of the Single smart pointer
    // before we can use destructuring assignment to access the individual components
    let (player_entity, player_life): (Entity, &Life) = *player;

    if player_life.0 == 0 {
        commands.entity(player_entity).despawn();
    }
}
```

[`Query::single`] returns a [`QuerySingleError`], allowing you to check if zero, one, or more than one matching entities were found.

For more discussion on [`Single`] and how it works, please see the [error handling] chapter.
Similarly, see the [resources] chapter of this book for a discussion on the choice between using a singleton entity or a resource.

[error handling]: ../control-flow/error-handling.md
[resources]: ./resources.md

## Accessing multiple items from the same query

By contrast, you may have a query and need to access multiple items from it at once.
The obvious method is to simply call [`Query::get`] multiple times on it.
While this works for read-only access,
it falls apart when using [`Query::get_mut`], as the borrow checker complains at you.

To help with this, Bevy offers two particularly helpful methods on [`Query`]:

- [`Query::get_multiple_mut`]: fetch multiple entities by their [`Entity`] ids, which must be unique.
    - Helpful for things like collisions.
- [`Query::iter_combinations_mut`]: iterate over all pairs, triples or so on of query items.
    - Great for gravity simulations!

## Disabling entities

From time-to-time, you might want to hide or disable an entity without despawning it.
While simply setting its [`Visibility`] can work well,
it won't stop any gameplay effects.

Bevy offers a [`Disabled`] component, which works by hiding entities with this component from
queries unless the [`Disabled`] component is explicitly permitted,
such as via [`With`], [`Has`] or [`Allows`].

Under the hood, this acts as a [default query filter],
adding an overridable filter to each query.
You can even add your own disabling components,
which can be helpful if you want to assign a specific meaning for *why* entities are disabled.

## Working with complex queries

In real projects, queries can get quite complex!
As a result, Bevy users tend to disable [`clippy`'s `type_complexity` lint] altogether.
But even without red squiggles, working with complex types can be frustrating and hard to reuse.

Bevy offers three good tools for this, each with their own niche:

- [type aliases]
  - reduces typing and cognitive complexity
  - quick and easy to define
  - simple and flexible
  - no added functionality
- custom [`QueryData`] / [`QueryFilter`] types
  - derive macro makes this easy!
  - define custom methods on the struct or the generated item type
  - bypass the standard 16-element limit of terms without nesting tuples
  - more composable than a [`SystemParam`]
- custom [`SystemParam`] types
  - derive macro is great for simple cases
  - combine data from multiple queries, or queries with other resources
  - custom methods can be used to encapsulate complex logic
  - sometimes provided by third-party libraries for ease of use
[`clippy`'s `type_complexity` lint]: https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity
[type aliases]: https://doc.rust-lang.org/reference/items/type-aliases.html

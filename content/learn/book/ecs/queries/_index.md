+++
title = "Fetching data with queries"
weight = 2
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++



Once we have data stored on our entities in the form of components, we need to be able to get the data back out in a principled way.
**Queries** are system parameters that allow us to carefully request sets of entities from the [`World`] that meet the criteria we care about and then retrieve the data we need to operate on.

The [`Query<Q, F=()>`] type has two type parameters: `Q` describes which data should be requested, while `F` type parameter describes how it should be filtered.
By default, `F` is set to `()`, the "unit type", indicating that we do not want a filter.

The simplest case is where we need to access the data of a single component, for example `Query<&Life`>.
In this case, `Q` is the type `&Life`, and `F` is `()`, because we didn't specify a value for it.

This will request a shared reference to the `Life` component on every entity that has that component.
This reference is read-only: we cannot mutate the values returned by our query.
If we want to be able to change these values, we need to request a mutable reference using `Query<&mut Life>` instead.

```rust
# use bevy::ecs::prelude::*;
# #[derive(Component)]
# struct Life;

// A system that has read-access to the Life component of each entity
fn read_life(query: Query<&Life>) {}

// A system that has write-access to the Life component of each entity
// Remember to set your query argument as mutable
fn write_life(mut query: Query<&mut Life>) {}
```

In order to access multiple components at once, we need to replace our `&Life` type with a tuple type that bundles many types into one.

```rust
# use bevy::ecs::prelude::*;
# #[derive(Component)]
# struct Life;
#
# #derive(Component)
# struct Defense;

// A system that has write-access to the Life component, and read-access to the Defense component
// Only entities with both Life and Defense will be included in this Query
fn life_and_defense(mut query: Query<(&mut Life, &Defense)>) {}
```

Here, the type of `Q` is `(&mut Life, &Defense)` (and `F` is still `()`).

Queries operate using "AND" logic (unless you use an `Option<&C>` query parameter): adding more components will always strictly reduce the number of entities returned by your query.

[`Query<Q, F=()>`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html

## Filtering queries

When components are fetched in queries, the data of *every* component in `Q` passed will be fetched and made available to the system.
However, this isn't always what you want!
In many cases, you just want to filter the query based on the presence (or absence) of a component, and don't want to deal with unpacking data you're never going to use.
This is particularly true when working with **marker components**: dataless structs designed to convey the identity or current state of an entity.

The two most important query filter types are [`With<C>`] and [`Without<C>`], which filter based on whether or not an entity has a component of the type `C`.

[`With<C>`]: https://docs.rs/bevy/latest/bevy/ecs/query/struct.With.html
[`Without<C>`]: https://docs.rs/bevy/latest/bevy/ecs/query/struct.Without.html

```rust
# use bevy::ecs::prelude::*;
# #[derive(Component)]
# struct Life;
#
# #derive(Component)
# struct Player;

// A system that has write-access to the Life component of entities with the Player component
// Q is &mut Life, and F is With<Player>
fn regenerate_player_life(mut query: Query<&mut Life, With<Player>>) {}

// This query requests the Entity identifier of all entities that don't have the Player component,
// so then they can be passed into our Commands system parameter
// Q is Entity, and F is Without<Player>
fn despawn_all_non_player_entities(mut commands: Commands, query: Query<Entity, Without<Player>>) {}

// This systems has two Query system parameters
// For player_query, Q is (&Position, &mut Targeting) and F is With<Player>
// For target_query, Q is (Entity, &Position, &TargetPriority) and F is (With<Enemy>, Without<Player>)
fn select_target(
    player_query: Query<(&Position, &mut Targeting), With<Player>>,
    target_query: Query<(Entity, &Position, &TargetPriority), (With<Enemy>, Without<Player>)>
) {
}
```

As with requested data, filters combine in an "and" fashion.
Only entities with the `Position`, `TargetPriority`, and `Enemy` components which don't have a `Player` component will be returned by `target_query` in the example above.

## Iterating over queries

Once we have a query, the most common pattern is perform some logic on every entity returned.
To do so, we can use straightforward for-loops:

```rust
#[derive(Component, Debug)]
struct Life {
    val: u8,
}

#[derive(Component)]
struct IncomingDamge {
    val: u8,
}

/// Prints the current life total of every entity with the Life component
fn report_life(query: Query<&Life>) {
    for life in query.iter() {
        dbg!(life);
    }
}

#[derive(Component)]
struct Age(u64);

fn increment_age(mut query: Query<&mut Age>) {
    // We need to use mut query, &mut Age, mut age, and .iter_mut() here because we need mutable access
    for mut age in query.iter_mut() {
        // age.0 refers to the first (only) field on our tuple type
        // We could make this more ergonomic by implementing the Add<Age, u64> trait
        // or the AddAssign<Age> trait on our Age component type
        age.0 = age.0 + 1;
    }
}

fn take_damage(query: Query<(&mut Life, &mut IncomingDamage)>) {
    // You can unpack your query iterator into several variables
    for (mut life, mut incoming_damage) in query.iter_mut() {
        life.val -= incoming_damage.val;
        incoming_damage.val = 0;
    }
}
```

If you're more experienced with Rust, you will be unsurprised to discover that you can also use common iterator tools like `.for_each`, `.map`, and `.filter` to work with your queries.

If you find yourself needing to iterate over all pairs (or triples or...) of a query (perhaps for collision detection), turn to the `iter_combinations` function demonstrated in the [corresponding example](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/iter_combinations.rs) to avoid borrow-checker headaches.

## Queries that return one entity

When we have a query that we *know* will always return a single entity, iterating over the query tends to result in unclear code.
To get around this, we can use [`Query::single()`] and [`Query::single_mut()`], depending on whether or not we need to mutate the returned data.

Note that these functions return a [`Result`]: if you expect this could fail in real scenarios (in case the query does not contain exactly one entity), handle the result properly.
Otherwise, just call `let (component_a, component_b) = query.single().unwrap()` to make use of the data quickly.

[`Query::single()`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html#method.single
[`Query::single_mut()`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html#method.single_mut
[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html

## Looking up specific entities

Each entity in our ECS data storages has a unique, arbitrarily assigned identifier: its [`Entity`], made up of two [`u32`]s.
We can fetch the [`Entity`] of each entity returned by our queries by including it as part of the first type parameter of [`Query`] as if it were a component (although no `&` is used):

```rust
// This system reports the Entity of every entity in your World
fn all_entities(query: Query<Entity>) {
 for entity in query.iter(){
  dbg!(entity);
 }
}

#[derive(Component)]
struct Marker;
struct MyEntities {
 entities: Vec<Entity>,
}
// Typically you'll combine this pattern with query filters 
// to extract the entities of a relevant subset, 
// and then store it somewhere where you can access it later
fn identify_yourself(query: Query<Entity, With<Marker>>, my_entities: ResMut<MyEntities>){
 for entity in query.iter(){
  my_entities.push(entity);
 }
}

```

Once we have a particular entity in mind, we can grab its data using [`Query::get()`] and the related methods on [`Query`].
This is fallible, and so it returns a [`Result`] that you must unwrap or handle.

[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`u32`]: https://doc.rust-lang.org/std/primitive.u32.html
[`Query::get()`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html#method.get

## Optional components in queries

If we want a query to include a component's data if it exists, we can use an `Option<&MyComponent>` query parameter in the first type parameter of [`Query`].
This can be a powerful tool for branching logic (use [`match`] on the [`Option`] returned), especially when combined with marker components.

[`match`]: https://doc.rust-lang.org/std/keyword.match.html
[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html

### `Or` Queries

By default, query filters (just like query data requests) operate on a "and" basis: if you have a filter for `With<A>` and another filter for `With<B>`, only entities with both the `A` and `B` components will be fetched.
We can change this behavior by using the [`Or`] type, nesting primitive query filters like [`With`], [`Without`] and [`Changed`] inside of it to return entities that meet any of the criteria inside.
If we wanted to purchase fruits that were either `Delicious` or `Cheap`, we would use `Query<&mut Owner, (Or<With<Deliciou>, With<Cheap>>)>` as the type of our query, allowing us to change the owner of any delicious and cheap fruit that we found.

Note that the `Or` type (and other query tuples) can be nested indefinitely, allowing you to construct very complex logic if needed.

[`Or`]: https://docs.rs/bevy/latest/bevy/ecs/query/struct.Or.html
[`With<C>`]: https://docs.rs/bevy/latest/bevy/ecs/query/struct.With.html
[`Without<C>`]: https://docs.rs/bevy/latest/bevy/ecs/query/struct.Without.html
[`Changed`]: https://docs.rs/bevy/latest/bevy/ecs/query/struct.Changed.html

### Running multiple queries at once

As the logic in your systems become more complex, you may find that you want to access data from two different queries at once.
In most cases, simply adding a second query as another system parameter works perfectly fine:

```rust
fn defense_aura_system(aura_query: Query<&Transform, With<Aura>>, target_query: Query<(&mut Defense, &Transform), With<Creature>>){
 // Give all allies near an aura-generator a bonus to their defense
}
```

But as you use this pattern more, you may encounter an error that looks something like:

```
   Query<&mut Transform, With<Camera>> in system move_player accesses component(s) &mut Transform in a way that conflicts with a previous system parameter. Allowing this would break Rust's mutability rules. Consider using `Without<T>` to create disjoint Queries or merging conflicting Queries into a `QuerySet`.
```

What went wrong? It worked just fine before!

Well, it turns out that Rust, in its infinite wisdom,
does not like it when you access the same data in multiple places at once,
if at least one of those accesses is mutable.
That's a result of its [ownership rules](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html): we could mutate data in the first query while the second query is trying to read the data, resulting in undefined behavior.
Which is bad.

Of course, you already knew that, and have carefully thought about the architecture of your system, designing something like:

```rust
fn camera_follow_system(player_query: Query<&Transform, With<Player>>, camera_query: Query<&mut Transform, With<Camera>>){
 let player_transform = player_query.single().unwrap();
 let camera_query = camera_query.single_mut.unwrap();
 // Insert logic here
}
```

You know that there's never going to be an entity that has both `Player` and `Camera` on it, so there's no way that you're ever accessing the same [`Transform`] component twice.
Unfortunately, Rust *doesn't* know that.
We can fix this by making *sure* our queries our disjoint, no matter what bizarre entities might exist, through the judicious application of `Without` queries.

```rust
fn camera_follow_system(player_query: Query<&Transform, With<Player>>, camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>){
 let player_transform = player_query.single().unwrap();
 let camera_query = camera_query.single_mut.unwrap();
 // Insert logic here
}
```

The other way to get around this issue is to use a [`QuerySet`], which permits multiple conflicting queries to exist in a single system.
The catch is that you can only access one query at a time.
Query sets can be useful when you need to access genuinely conflicting data, such as if we truly had an entity with both `Player` and `Camera` that we wanted to operate on in both loops of our system.
Let's rewrite our broken system again, using a [`QuerySet`] instead.

```rust
fn camera_follow_system(queries: QuerySet<Query<&Transform, With<Player>>, Query<&mut Transform, With<Camera>>){
 let player_transform = queries.0.single().unwrap();
 let camera_query = queries.1.single_mut.unwrap();
 // Insert logic here
}
```

Bevy's systems automatically run in parallel by default, so long as the scheduler can guarantee that the same data is never accessed in another place while it is being mutated.

As a result, we can use the same query filtering techniques described  to allow our *systems* to safely run in parallel.
In addition to improving parallelism, this also reduces the false positives when checking for [system execution order ambiguities](https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.ReportExecutionOrderAmbiguities.html), as we can guarantee that the relative order of two systems that do not share data never changes the final outcome.

[`Transform`]: https://docs.rs/bevy/latest/bevy/transform/components/struct.Transform.html
[`QuerySet`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.QuerySet.html

+++
title = "Resources, Queries, and Commands"
insert_anchor_links = "right"
[extra]
weight = 4
status = 'hidden'
+++

If ECS is the first three letters, then **Resources, Queries, and Commands** are the next three.
These concepts are core to Bevy's ECS (so much so that they're used in the previous section!), but they aren't inherent to the architecture.

## Resources

**Resources** are global state singletons.
Unlike [components](../the-three-letters#the-c-components), which are reused across multiple entities, resources are unique. For any given resource type (like `Score` in the example below), there can only be one instance in your game world at a time.

Like components, resources in Bevy are also "just Rust structs" (or enums).

```rs
#[derive(Resource)]
struct Score {
    points: i32
}
```

They're accessed and updated in systems, similar to entities and components:

```rs
fn update_score(mut score: ResMut<Score>) {
    // ResMut gets the resource mutably, so we can update it
    score.points += 1;
}
```

## Queries

Queries are used to fetch data from the ECS, either in read only mode (like a [`&`](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing)), or in mutable access mode (like a [`&mut`](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)).
When writing a query, you provide a set of components, and Bevy will fetch all entities that have every requested component.
(The entities fetched may also have other components, but those are ignored if the query does not ask for them.)

Any ECS system can make queries by adding the appropriate argument to the function signature:

```rs
fn my_system(mut query: Query<(&Color, &mut Location)>) {
    for (color, mut location) in query.iter_mut() {
        if color == Color::Red {
            location += 1;
        }
    }
}
```

Then, when systems are run as part of a Bevy [app](../../the-game-loop/app), the engine automatically fetches the requested data, parallelizing work between systems wherever possible:

```rs
fn main() {
    App::new()
        .add_systems(Update, (my_system, my_other_system))
        .run();
}
```

{% callout(type="info") %}
Bevy queries let you update massive amounts of game data in a tight, cache-friendly loop.

Going back to our database analogy, a query is a lot like a [SQL `SELECT` statement](https://www.w3schools.com/sql/sql_select.asp): `SELECT Color, Location from World`
{% end %}

Queries have a lot more functionality than what's shown here.
You can also request optional components for `OR` semantics, add query filters, and much more!
Queries are covered in more detail in the [Queries](../../storing-data/queries) chapter.

## Commands

Commands are a very flexible structure that allows for arbitrary changes to the ECS.
They are mostly used for write operations, such as spawning entities (as we saw in the previous section).
Any system can access the command queue by adding a `mut commands: Commands` to the function signature:

```rs
fn spawn_entities(mut commands: Commands) {
    // Spawn an entity with all our components
    commands.spawn((Location::zero(), Color::Red, Player));
    // Spawn an entity with only one component
    commands.spawn(Color::Heliotrope);
}
```

Commands can do nearly anything to the ECS that you can imagine, including running queries!
You can also write custom commands to extend the ECS however you might need to.
They are talked about in more detail in the [Commands](../../control-flow/commands) chapter.

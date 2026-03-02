+++
title = "Resources, Queries, and Commands"
insert_anchor_links = "right"
[extra]
weight = 4
status = 'hidden'
+++

If entities, components and systems are the first three concepts, then **Resources**, **Queries**, and **Commands** are the next three.
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

Resources can be accessed and updated in systems, similar to the components on entities:

```rs
fn update_score(mut score: ResMut<Score>) {
    // ResMut gets the resource mutably, so we can update it
    score.points += 1;
}
```

## Queries

Queries are used to fetch data from the ECS, either in read only mode (like a [`&`](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing)), or in mutable access mode (like a [`&mut`](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)).
When writing a query, you provide a set of components, and Bevy will fetch all entities that have every requested component.
The entities fetched may also have other components, but only the matching component types will be returned to read or write.

Any ECS system can make queries by adding the appropriate `Query`-typed argument to the function signature:

```rs
#[derive(Component)]
struct Poison{
    stacks: u32
};

#[derive(Component)]
struct Life {
    current: u32,
    max: u32,
}

fn apply_poison(mut query: Query<&Poison, &mut Life>){
    for (poison, mut life) in query.iter_mut(){
        life.current.saturating_sub(poison.stacks);
    }
}

fn tick_down_poison(mut query: Query<&mut Poison>){
    for poison in query.iter_mut(){
        if poison.stacks > 0 {
            poison.stacks -= 1;
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
You can request optional components, fetch the `Entity` associated with each item, add query filters, and much more!
Queries are covered in more detail in the [Queries](../../storing-data/queries) chapter.

## Commands

Commands allow for arbitrary, deferred changes to the ECS `World`.
They are mostly used for complex write operations, such as spawning entities (as we saw in the previous chapter).
Any system can access the command queue by adding a `mut commands: Commands` to the function signature:

```rs
fn spawn_entities(mut commands: Commands) {
    // Spawn an entity with all our components
    commands.spawn((Location::zero(), Color::Red, Player));
    // Spawn an entity with only one component
    commands.spawn(Color::Heliotrope);
}
```

A wide range of built-in commands are provided, 
but you can also write custom commands to queue up any ECS logic you might desire.
You can read about these in more detail in the [Commands](../../control-flow/commands) chapter.

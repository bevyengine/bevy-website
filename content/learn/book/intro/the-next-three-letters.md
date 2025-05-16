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
Unlike [components](../the-three-letters#the-c-components), which are reused across multiple entities, there is only one instance of a resource of type `R` at a time.
Like components, resources in Bevy are also "just Rust structs" (or enums).

```rs
#[derive(Resource)]
struct InGameClock {
  current_time: Instant
}
```

They're accessed and updated in systems, similar to entities and components:

```rs
fn update_game_time(time: ResMut<InGameClock>) {
  // ResMut gets the resource mutably, so we can update it
  *time = Instant::now();
}
```

## Queries

Queries are used to fetch data from the ECS, either in read only mode (like a `&`), or in mutable access mode (like a `&mut`).
When writing a query, you provide a set of components, and Bevy will fetch all entities that have those components.
(The entities fetched may also have other components, but those are ignored if the query does not ask for them.)

In the database model, a query is a lot like a SQL `SELECT` statement: `SELECT component1, component2 from world`

Any ECS system can make queries by adding the appropriate argument to the function signature:
```rs
fn my_system(mut entities: Query<(&Color, &mut Location)>) {
    for (color, mut location) in entities.iter_mut() {
        if color == Color::Red {
            location += 1;
        }
    }
}
```
Then, the query information is passed into the system by Bevy automatically when the system is added to an [app](todo-link-to-apps):
```rs
fn main() {
    App::new()
        .add_systems(Update, (my_system, my_other_system))
        .run();
}
```

## Commands

Commands are a very flexible structure that allows for aribtrary changes to the ECS.
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
They are talked about in more detail in the [TODO COMMANDS CHAPTER](todo-link) chapter.

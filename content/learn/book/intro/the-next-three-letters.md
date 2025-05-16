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

todo

## Commands

todo

outline:
- Resources are global state singletons
- Queries fetch data (entities and their components) from the ECS that match a pattern
	- It's a lot like a `SELECT col1, col2 from ECS`
- Commands are an interface for write operations to the ECS
	- They are used in systems
	- You can do other things with them too, but that's covered in the main commands section

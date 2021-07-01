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

On the next page, we'll create a simple "game" using the ECS so you can see how this all fits together.

## Rust tips and tricks for Bevy's ECS

If you're new to Rust, there are a few standard patterns and pitfalls that are particularly relevant to Bevy's ECS.
Feel free to come back to this section later as you learn more about Bevy!

### Tuple structs

Tuple structs, defined like `struct Score(u64);` or `struct Position(u8, u8);`, are structs with unnamed fields, accessed by the order in which they were defined.
`.0`, `.1` and so on are used to access fields, counting up from 0 in the order they were declared.

`my_position.0 = 42` can seem opaque when you first read it, but in Rust it just means that you're assigning `42` to the first field of the tuple struct `my_position` (by convention, the x coordinate).
Using ordinary structs with named fields rather than tuple structs (even for simple components and resources!) can go a long way to improving the clarity of your code by providing more descriptive field names.
In this example, `my_position.x = 42` immediately communicates intent, rather than relying on the end user to remember the convention used.

### Smart pointers

There are a few points to be aware of when working with "smart pointers" like components and resources:

1. The value returned in a system when you ask for `Res<Score>` is not a plain `Score`, it is a `Res<Score>`. This is a resource that stores the score, similar to an ordinary `&` pointer, but with some special behavior (like change detection).
2. Because `Res` and other smart pointers implement the `Deref` trait, Rust will attempt to automatically dereference it to match required types when it can.
   1. This is commonly done when accessing fields and methods: calling `.title` on `Res<WindowDescriptor>` will grab the title field of `WindowDescriptor`, not the wrapping `Res<WindowDescriptor>` as the `Res` type has no field named `title`.
3. When your types cannot be automatically dereferenced, you will need to manually dereference out of the outer smart pointer using `*`.
   1. This is commonly needed when assigning a value to your data (rather than just one of its fields), or when using `match`.

### Initializing structs with some default values

Bevy often requires defining complex structs when creating entities with bundles or configuring the app with resources.
In many cases, we want to use *some* default fields, but not all of them.
As Rust does not have default arguments, we instead use [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax) in combination with the [`Default::default()`](https://doc.rust-lang.org/std/default/trait.Default.html) method to allow us to quickly set some values while leaving others unchanged.

```rust
struct CombatStats {
	attack: u8,
	defense: u8,
	speed: u8,
}

impl Default for CombatStats {
	// This method is being used to create an initial struct
	// whose values we copy and then override
	fn default() -> Self {
		CombatStats {
			attack: 100,
			defense: 100,
			speed: 100,
		}
	}
}

let my_stats = CombatStats {
	attack: 150,
	defense: 50,
	..Default::default()
}
```

This pattern is commonly used when working with bundles: groups of components typically used for entity initialization.

### Type aliases don't play nice

Bevy's ECS uses Rust's type system to dispatch data to our systems as requested.
This is very convenient for safety and ergonomic reasons, but means that only one resource of each Rust type can exist at once, and only one component of each type can be stored on each entity.

"One of each Rust type" has somewhat surprising consequences for new Rust users though: [type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html) will not result in unique types, and should generally not be used to define resource or component types.

```rust
use std::any::type_name;

// Our first component type
struct Life(u32);

// An attempt at a second component type, using a type alias
type Health = Life;

// Unfortunately, these two types share the same id :(
// Attempting to insert both Life and Health would result in overwritten values,
// and any query for one would return either
assert_eq!(type_id(Life(42)), type_id(Health(42));
```

Instead, you have to define entirely new types for each component or resource you wish to use,
commonly using the [`newtype` pattern](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types) to wrap a single value of some common or external type.

```rust
use std::any::type_name;

struct Life(u32);
struct Health(u32);

// Our components have different types now!
assert_ne!(type_id(Life(42)), type_id(Health(42));
```

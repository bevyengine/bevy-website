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

## Rust tips and tricks for Bevy's ECS

If you're new to Rust, there are a few standard tricks and idioms that are used by Bevy's ECS that are worth learning about.
Feel free to come back to this section later as you learn more about Bevy!

### Tuple structs

When working with tuple structs like `struct Score(u64)` or `struct Position(u8, u8)`, `.0`, `.1` and so on are used to access fields, counting up from 0 in the order they were defined.

`let foo.0 = bar` can seem opaque when you first read it, but in Rust it just means that you're assigning `bar` to the first field of the tuple struct `foo`.
Using ordinary structs with named fields rather than tuple structs (even for simple components and resources!) can go a long way to improving the clarity of your code by providing more descriptive field names.

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
As Rust does not have default arguments, we instead use [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax) in combination with the [`default()`](https://doc.rust-lang.org/std/default/trait.Default.html) method from the `Default` trait to allow us to quickly set some values while leaving others unchanged.

```rust
// We want to quickly generate new enemies from this component bundle
// By using some default fields
#[derive(Bundle, Default)]
struct EnemyBundle {
	enemy: Enemy,
	life: Life,
	attack: Attack,
	defense: Defense,
}

// Dataless unit structs automatically have a default value
// Since only one possible value exists
struct Enemy;

struct Life(u32);

// Manually implementing Default for one of our component types
// allowing us to control the starting value
impl Default for Life {
	fn default() -> Self {
		Life(100)
	}
}

// These components will have a default value of 0, as u32::default() == 0
#[derive(Default)]
struct Defense(u32);

#[derive(Default)]
struct Attack(u32);

let my_enemy = EnemyBundle {
	attack: 10,
	defense: 5,
	// All other fields are initialized from their default value
	// setting enemy: Enemy and life: Life(100)
	..Default::default()
}
```

+++
title = "Rust tips and tricks"
weight = 7
template = "book-section.html"
page_template = "book-section.html"
+++

If you're new to Rust, there are a few standard patterns and pitfalls that are particularly relevant to Bevy's ECS.
Feel free to refer back to this section later as you learn more about Bevy!

### Tuple structs

[Tuple structs](https://doc.rust-lang.org/1.9.0/book/structs.html), defined like `struct Score(u64);` or `struct Position(u8, u8);`, are structs with unnamed fields, accessed by the order in which they were defined.
`.0`, `.1` and so on are used to access fields, counting up from 0 in the order they were declared.

`my_position.0 = 42` can seem opaque when you first read it, but in Rust it just means that you're assigning `42` to the first field of the tuple struct `my_position` (by convention, the x coordinate).
Using ordinary structs with named fields rather than tuple structs (even for simple components and resources!) can go a long way to improving the clarity of your code by providing more descriptive field names.
In this example, `my_position.x = 42` immediately communicates intent, rather than relying on the end user to remember the convention used.

### Smart pointers

There are a few points to be aware of when working with "smart pointers" like components and resources:

1. The value returned in a system when you ask for `Res<Score>` is not a plain `Score`, it is a `Res<Score>`. This is a resource that stores the score, similar to an ordinary `&` pointer, but with some special behavior (like change detection).
2. Because {{rust_type(type="struct" crate="bevy" mod = "ecs/system" name="Res" no_mod = "true")}} and other smart pointers implement the {{rust_type(type="trait" crate="std" mod = "ops" name="Deref" no_mod = "true")}} trait, Rust will attempt to automatically dereference it to match required types when it can.
   1. This is commonly done when accessing fields and methods: calling `.title` on `Res<WindowDescriptor>` will grab the title field of {{rust_type(type="struct" crate="bevy" mod = "window" name="WindowDescriptor" no_mod = "true")}}, not the wrapping `Res<WindowDescriptor>` as the {{rust_type(type="struct" crate="bevy" mod = "ecs/system" name="Res" no_mod = "true")}} type has no field named `title`.
3. When your types cannot be automatically dereferenced, you will need to manually dereference out of the outer smart pointer using `*`.
   1. This is commonly needed when assigning a value to your data (rather than just one of its fields), or when using {{rust_type(type="keyword" crate="std" name="match" no_mod = "true")}}.

### Initializing structs with some default values

Bevy often requires defining complex structs when creating entities with bundles or configuring the app with resources.
In many cases, we want to use *some* default fields, but not all of them.
As Rust does not have default arguments, we instead use [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax) in combination with the {{rust_type(type="trait" crate="std" mod = "default" name="Default" method = "default" no_mod = "true")}} method to allow us to quickly set some values while leaving others unchanged.

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
#[derive(Component)]
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

#[derive(Component)]
struct Life(u32);

#[derive(Component)]
struct Health(u32);

// Our components have different types now!
assert_ne!(type_id(Life(42)), type_id(Health(42));
```

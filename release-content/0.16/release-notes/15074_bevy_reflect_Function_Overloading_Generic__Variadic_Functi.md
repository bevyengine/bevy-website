<!-- bevy_reflect: Function Overloading (Generic & Variadic Functions) -->
<!-- https://github.com/bevyengine/bevy/pull/15074 -->

**Bevy 0.15** added support for reflecting functions to `bevy_reflect`, Bevy's type reflection crate.
This allows Rust functions to be called dynamically with a list of arguments generated at runtime—and safely!

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let reflect_add: DynamicFunction = add.into_function();

let args = ArgList::new()
  .push_owned(25_i32)
  .push_owned(75_i32);

let result = reflect_add.call(args).unwrap();

let sum = result.unwrap_owned().try_take::<i32>().unwrap();
assert_eq!(sum, 100);
```

However, due to limitations in Rust, it was not possible to make these dynamic functions generic.
This meant individual functions had to be created for all desired monomorphizations
and manually mapped to at runtime.

```rust
fn add<T: Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

let reflect_add_i32 = add::<i32>.into_function();
let reflect_add_u32 = add::<u32>.into_function();
let reflect_add_f32 = add::<f32>.into_function();
// ...
```

While the original Rust limitations still exist, Bevy 0.16 improves the developer experience by adding support for function overloading.
The term "function overloading" might be familiar to developers from other programming languages,
but essentially it means that one function can have multiple argument signatures.

This allows us to simplify the previous example:

```rust
let reflect_add = add::<i32>.into_function()
  .with_overload(add::<u32>)
  .with_overload(add::<f32>);
```

The first `add::<i32>` acts as the base case, with each overload acting on top of it.
When the function is called, the corresponding overload is selected based on the types of the provided arguments.

And by extension of the fact that function overloading allows for multiple argument signatures,
this also means that we can define functions that take a variable number of arguments,
commonly known as "variadic functions."

This allows for some interesting use cases:

```rust
#[derive(Reflect)]
struct Player {
    name: Option<String>,
    health: u32,
}

// Creates a `Player` with one of the following based on the provided arguments:  
// - No name and 100 health  
// - A name and 100 health  
// - No name and custom health  
// - A name and custom health
let create_player = (|| Player {
    name: None,
    health: 100,
  })
  .into_function()
  .with_overload(|name: String| Player {
    name: Some(name),
    health: 100,
  })
  .with_overload(|health: u32| Player {
    name: None,
    health
  })
  .with_overload(|name: String, health: u32| Player {
    name: Some(name),
    health,
  });
```
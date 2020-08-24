+++
title = "ECS"
weight = 3
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

All app logic in Bevy uses the Entity Component System paradigm, which is often shortened to ECS. ECS is a software pattern that involves breaking your program up into **Entities**, **Components**, and **Systems**. **Entities** are unique "things" that are assigned groups of **Components**, which are then processed using **Systems**.

For example, one entity might have a `Position` and `Velocity` component, whereas another entity might have a `Position` and `UI` component. Systems are logic that runs on a specific set of component types. You might have a `movement` system that runs on all entities with a `Position` and `Velocity` component.

The ECS pattern encourages clean, decoupled designs by forcing you to break up your app data and logic into its core components. It also helps make your code faster by optimizing memory access patterns and making parallelism easier.

## Bevy ECS

Bevy ECS is Bevy's implementation of the ECS pattern. Unlike other Rust ECS implementations, which often require complex lifetimes, traits, builder patterns, or macros, Bevy ECS uses normal Rust datatypes for all of these concepts:
* **Components**: normal Rust structs
    ```rs
    struct Position { x: f32, y: f32 }
    ```
* **Systems**: normal Rust functions
    ```rs
    fn print_position_system(position: &Position) {
        println!("position: {} {}", position.x, position.y);
    }
    ```
* **Entities**: a simple type containing a unique integer  
    ```rs
    struct Entity(u64);
    ```

Now lets see how this works in practice!

## Your First System

Paste the following function into your `main.rs` file:

```rs
fn hello_world() {
    println!("hello world!");
}
```

This will be our first system. The only remaining step is to add it to our App!

```rs
fn main() {
    App::build()
        .add_system(hello_world.system())
        .run();
}
```

Note the `hello_world.system()` function call. This is a "trait extension method" that converts the `hello_world` function into the {{rust_type(type="trait" crate="bevy_ecs" name="System")}} type.

The {{rust_type(type="trait" crate="bevy_ecs", name="IntoQuerySystem" method="add_system" no_struct=true)}} function adds the system to your App's {{rust_type(type="struct", crate="bevy_ecs", name="Schedule")}}, but we'll cover that more later.

Now run your App again using `cargo run`. You should see `hello world!` printed once in your terminal.

## Your First Components

Greeting the whole world is great, but what if we want to greet specific people? In ECS, you would generally model people as entities with a set of components that define them. Lets start simple with a `Person` component.

Add this struct to `main.rs`:
```rs
struct Person;
```

But what if we want our people to have a name? In a more traditional design, we might just tack on a `name: String` field to `Person`. But other entities might have names too! For example, dogs should probably also have a name. It often makes sense to break datatypes up in to small pieces to encourage code reuse. So lets make `Name` its own component:

```rs
struct Name(String);
```

We can then add `People` to our {{rust_type(type="struct" crate="bevy_ecs" name="World")}} using a "startup system". Startup systems are just like normal systems, but they run exactly once, before all other systems, right when our app starts. Lets use {{rust_type(type="struct" crate="bevy_ecs" name="Commands")}} to spawn some entities into our {{rust_type(type="struct" crate="bevy_ecs" name="World")}}:

```rs
fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}
```

Now register the startup system like this:

```rs
fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .run();
}
```

We could run this App now and the `add_people` system would run first, followed by `hello_world`. But our new people don't have anything to do yet! Lets make a system that properly greets the new citizens of our {{rust_type(type="struct" crate="bevy_ecs" name="World")}}:

```rs
fn greet_people(person: &Person, name: &Name) {
    println!("hello {}!", name.0);
}
```

And then register it in our App:

```rs
fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}
```

The parameters we pass in to a "system function" define what entities the system runs on. In this case, `greet_people` will run on all entities with the `Person` and `Name` component.

Now running our app will result in the following output:

```
hello world!
hello Elaina Proctor!
hello Renzo Hume!
hello Zayna Nieves!
```

Marvelous!

**Quick Note**: "hello world!" might show up in a different order than it does above. This is because systems run in parallel by default when they have no shared dependencies.

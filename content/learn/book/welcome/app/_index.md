+++
title = "Creating Bevy apps"
weight = 2
template = "book-section.html"
page_template = "book-section.html"
+++

When you're making a game in Bevy, you will have a single `App`, which stores and handles all of your game logic and data.
Let's take a closer look at that hello world code.

```rust
use bevy::prelude::*;

fn main(){
  App::new()
  .build()
  .add_system(hello.system())
  .run();
}

fn hello(){
    println!("Hello, Bevy!")
}
```

So, we first create our [`App`](https://docs.rs/bevy/latest/bevy/app/struct.App.html), then tell it to [`build`](https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.build), performing initialization steps.
This produces an [`AppBuilder`](https://docs.rs/bevy/latest/bevy/app/struct.AppBuilder.html), which we can use to configure the `App`.
We do so by add a simple system, which prints "Hello, Bevy!" when it is run.
Finally once all of our work is complete, we call [`AppBuilder::run()`](https://docs.rs/bevy/latest/bevy/app/struct.AppBuilder.html#method.run) to actually make our app *do things*.

## What makes an App?

So, what sort of data does are `App` really store?
Turns out: it's fairly simple.
Looking at the docs for [`App`](https://docs.rs/bevy/latest/bevy/app/struct.App.html), we find three fields:

```rust
struct App {
  world: World
  runner: Box<dyn Fn(App) + 'static, Global>
  schedule: Schedule
}
```

The `World` stores all of our data, both entity-component data and resources.
The `runner` field contains a function that tells our app how to loop and execute.
And the `Schedule` contains all of the systems that execute on our data, stored in a way that our scheduler can use to run them at the appropriate time.

Generally, you'll be operating at a higher-level of abstraction than this: thinking about data in terms of specific resources or components, adding systems to the right place rather than manipulating the `Schedule` yourself, and sticking to the default runner.

To do so, you'll use the API defined by `AppBuilder`.
The most fundamental things you'll do here are to initialize resources in the `World`, add systems to our schedule, and import logic in bulk using plugins.
Let's write a very simple demo that shows how those work.

```rust
use bevy::prelude::*;

fn main() {
    App::build()
        // Plugins are AppBuilder code that was written elsewhere,
        // imported as a single unit for organization and clarity
        .add_plugins(DefaultPlugins)
        // Resources are global singleton data stored in the `World`
        .insert_resource(Fibonacci { a: 1, b: 1 })
        // Systems run every pass of the game loop and perform logic
        .add_system(fibonacci_sum.system())
        .add_system(report_fibonacci.system())
        .run();
}

// A simple data structure to store the two current fibonacci numbers
struct Fibonacci {
    a: u64,
    b: u64,
}

// This system requires mutable access to our Fibonacci resource,
// so we add `ResMut` to its function parameters
fn fibonacci_sum(mut fibonacci: ResMut<Fibonacci>) {
    // This crashes fairly quickly, as we overflow our u64 data storage
    let new_sum = fibonacci.a + fibonacci.b;
    fibonacci.a = fibonacci.b;
    fibonacci.b = new_sum;
}

// This system only reads the value of our Fibonacci resource,
// so we only need `Res`
fn report_fibonacci(fibonacci: Res<Fibonacci>) {
    info!(fibonacci.a, fibonacci.b);
}
```

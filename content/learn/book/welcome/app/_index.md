+++
title = "Creating Bevy apps"
weight = 2
template = "book-section.html"
page_template = "book-section.html"
+++

Bevy programs store and execute all of their game logic and data with a single `App` data structure.
Let's make a trivial Hello World app.

```rust
use bevy::prelude::*;

fn main(){
  App::new()
  .add_system(hello.system())
  .run();
}

fn hello(){
    println!("Hello, Bevy!")
}
```

The process is straightforward: we first create a new [`App`](https://docs.rs/bevy/latest/bevy/app/struct.App.html).
Then, we add a simple system, which prints "Hello, Bevy!" when it is run.
Finally once we're done configuring the app, we call [`App::run()`](https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.run) to actually make our app *do things*.

## What makes an App?

So, what sort of data does are `App` really store?
Turns out: it's fairly simple.
Looking at the docs for [`App`](https://docs.rs/bevy/latest/bevy/app/struct.App.html), we find three fields: `world`, `runner` and `schedule`.
You can read more about these by exploring the reference documentation linked just above.

Generally, you'll be operating at a higher-level of abstraction than these basic primitives: thinking about data in terms of specific resources or components, adding systems via convenient methods rather than manually supplying a `Schedule`, and sticking to the default runner which runs your game logic in a loop.

To do so, you'll customize your own `App` using various methods with the builder pattern.
The most fundamental tools to do so are:

  1. Initializing resources in the `World` to store global data.
  2. Adding systems to our `Schedule` to perform logic in the specified order.
  3. Importing other blocks of logic and code using plugins.
Let's write a very simple demo that shows how those work.

```rust
use bevy::prelude::*;

fn main() {
    App::build()
        // Plugins are App code that was written elsewhere,
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

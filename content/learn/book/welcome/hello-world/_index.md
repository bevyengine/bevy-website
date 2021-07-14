+++
title = "Hello World"
weight = 3
template = "book-section.html"
page_template = "book-section.html"
+++

So, you've just made a project with Bevy. Great! However, currently, it isn't doing things 'the Bevy way'. Let's fix.

## Bevy App Structure

Bevy programs, which include games, tools and more, are conventially named 'Apps' and currently our Bevy program doesn't really follow the app structure or how bevy apps work.

Below is a basic outline of how your Bevy Apps should look.

```rust
// MODULES (only the first line is required)
use bevy::prelude::*;

use another::module::*;

mod some_file;
mod another_file;

// COMPONENTS AND RESOURCES

struct Player;

struct Health {
    value: u32,
}

impl Health {
    pub fn start() -> Self {
       Self {
           value: 100,
       } 
    }
}
// SYSTEMS

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert(Player)
        .insert(Health::start());
    println!("Setup Complete")
}

// MAIN

fn main() {
    App::build()
        .add_system(setup.system())
        .run();
}
```

Bear in mind, the order of everything beyond modules is completely up to you.

In detail:
- First, you should import your modules. In order for a bevy program to work properly, it is **ESSENTIAL** that you import the bevy prelude, as this contains the App struct and other things required for bevy to work.

- Secondly, you should define your components and implementations. As displayed, a component can **optionally have values and implementations** which can be used when creating an entity

- Thirdly, you should define your systems which are the logic of the game. In this example, we have a `setup()` function which spawns a new entity and inserts 2 componenets, Player and Health. We can use implementations in functions in the insertion. See how it links together!

- Finally, we have the main function which defines a Bevy App, adds any systems, builds and runs our new App. Whenever, you define a new system, you need to add it to this decleration. This can get messy quick, which is why you usually seperate these things into **plugins** (talked about in a later chapter).

A lot of this, you don't need to wory about right now. Just remember that:
- You add systems to your app in the main function and
- You define systems that can act on components

## Making A Bevy App

That was a lot of information to take in, but don't worry. Let's start off simple with a basic Bevy App!

Firstly open `main.rs` in the project from last page and import the Bevy prelude:

```rust
use bevy::prelude::*;

// Rest of code
```

Then, replace the main function with a Bevy App Decleration:

```rust
fn main() {
    App::build().run();
}
```

What you have done is define a Bevy App! Well done! Now compile your program!

```bash
$ cargo run
    Compiling my_bevy_game v0.1.0 (/home/test/projects/my_bevy_game)
    Finished dev [unoptimized + debuginfo] target(s) in 19.22s
    Running 'target/debug/my_bevy_game'
    
$
```

If you see something like this, you've successfully made a Bevy App (albeit a very empty one). Congrats!

## Hello World (the Bevy Way)

Let's spice up our Bevy App with a 'Hello Bevy!' message.

Above `fn main() {`, define a `hello_bevy()` function.

```rust
fn hello_bevy() {
    println!('Hello, Bevy!');
}
```

Right now, this function is just that: a function. But soon, it will become a blossoming system.

Now, in your app decleration, add the `hello_bevy` function as a system, as shown

```rust
fn main() {
    App::build()
        .add_system(hello_bevy.system())
        .run()
}
```

We just ran a **trait extension method** which allows us to convert our simple function to a powerful system.

Now, you can run the project again:

```bash
$ cargo run
    Compiling my_bevy_game v0.1.0 (/home/test/projects/my_bevy_game)
    Finished dev [unoptimized + debuginfo] target(s) in 19.22s
    Running 'target/debug/my_bevy_game'
    
Hello, Bevy!
$
```

Excellent Job! You have now made a 'Hello, Bevy!' app!

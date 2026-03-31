+++
title = "Handling Errors"
insert_anchor_links = "right"
[extra]
weight = 6
+++

Failures happen all the time in production software, from recoverable failures like not finding a user's save file on first run to unrecoverable errors which are symptoms of critical bugs.
Dealing with that failure in a productive way means games and applications become more reliable.
In Bevy, with data such as Entities, Components, and Resources being inserted and manipulated at runtime, handling cases when a Resource doesn't exist yet, an index needs to be updated, or a Component doesn't have the data you're looking for is routine work.

## To Recover or Not

Broadly speaking, there are two types of failure in Rust:

- Panics (which crash the app, and are "Unrecoverable")
- Everything else (which don't crash the app so are called "Recoverable")

Panics are usually undesirable, since a game or application crashing is a catastrophic experience for a player or user!
They happen immediately, and provide us no opportunity for clean up or a graceful shutdown. Current game progress cannot be saved, and the entire state of the application is discarded.

So panics are bad? Well, kind of. Using panicking macros like [`todo!`](https://doc.rust-lang.org/std/macro.todo.html) can be extremely useful during prototyping. Just try to remove them before you let anyone run your code.

## Avoiding crashes

In Rust, unrecoverable failures are explicitly written in the source code.
Some functions that will signal a potential crash include:

- Macros that explicitly panic, such as [`panic!`](https://doc.rust-lang.org/std/macro.panic.html)
- `unwrap` or `expect` on `Result` and `Option`
- Indexing into a `Vec` when the `Vec` doesn't have enough items.

  ```rust
  let my_data = vec![1,2,3];
  // this panics because there is no item at index 10!
  let tenth_item = my_data[10];
  ```

  > thread 'main' (19237675) panicked at src/main.rs:4:29:
  > index out of bounds: the len is 3 but the index is 10

You can check your own code for these panicking functions and remove them, but some functions outside of your own program code, such as from third party crates, could also contain them.
The only way to make sure a function won't panic is to read and understand the source code.
Crates like Bevy document which functions they provide could call panics internally and in what scenarios they will panic.

{% callout() %}

Additionally, there are some clippy lints that can help catch _some_ panics, but not all of them.
Here's an example of a few and how to configure them in a workspace.
You can look up what each does [on the clippy website](https://rust-lang.github.io/rust-clippy/master/)

```toml
[workspace.lints.clippy]
unwrap_used = "warn"
expect_used = "warn"
arithmetic-side-effects = "warn"
indexing_slicing = "warn"
panic = "warn"
todo = "warn"
```

{% end %}

## Recoverable failures

The [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) and [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) types are regular enums commonly used to represent different kinds of failure.
An `Option`'s variants, `None` and `Some`, indicate whether a value exists or not while a `Result`'s variants, `Ok` and `Err` store a successful value or an error value respectively.

```rust
fn main() {
    let some_value: Option<u32> = Some(10);
    let no_value: Option<u32> = None;
    let success: Result<u32, String> = Ok(20);
    let error: Result<u32, String> = Err("couldn't find it!".to_string());
}
```

When dealing with functions that could fail the return type will be wrapped in a `Result`, `Option`, or similar type.
[`Srgba::hex`](https://docs.rs/bevy/latest/bevy/prelude/struct.Srgba.html#method.hex) is one example of this from the `bevy_color` crate.
Parsing hex-based colors could fail, since not all characters are valid hex values.
`Srgba::hex` returns a `Result<Srgba, HexColorError>` value to indicate this.

Here are two attempts at parsing a hex color.
One succeeds, `FF00FF`, while the other fails since `M` isn't a valid hex digit.

```rust
use bevy::prelude::*;

fn main() {
    let color = Srgba::hex("FF00FF");
    dbg!(color);

    let failed_color = Srgba::hex("M00M00");
    dbg!(failed_color);
}
```

The resulting output shows the parsing success and failure values.

```rust
[src/main.rs:5:5] color = Ok(
    Srgba {
        red: 1.0,
        green: 0.0,
        blue: 1.0,
        alpha: 1.0,
    },
)
[src/main.rs:8:5] failed_color = Err(
    Parse(
        ParseIntError {
            kind: InvalidDigit,
        },
    ),
)
```

### Handling Error Values

Each hex parse result value is wrapped in either `Result::Ok` or `Result::Err`.
A common way to get the interior value is to [`unwrap`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap) the `Result`, but remember that in the case of an `Err`, `unwrap` will crash the program!
With this in mind `unwrap` is best used when you know the value will be a success, and an `Err` value would indicate a bug in your application.

This is notably true when [writing tests](../development-practices/testing.md), which can use panicking macros like `assert_eq!` to fail a test!

#### Default Values

If the value that failed to parse has a good default value, then [`unwrap_or`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or) and [`unwrap_or_default`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default) are good alternatives to `unwrap`.
Each enables passing the parsed value through if the parse was successful, or returning a different value if the parse failed, and neither will panic.

```rust
use bevy::prelude::*;

fn main() {
    let color = Srgba::hex("FF00FF").unwrap();
    dbg!(color);

    let failed_color_1 = Srgba::hex("M00M00").unwrap_or(Srgba {
        red: 0.,
        green: 1.,
        blue: 1.,
        alpha: 1.,
    });
    dbg!(failed_color_1);

    let failed_color_2 = Srgba::hex("M00M00").unwrap_or_default();
    dbg!(failed_color_2);
}
```

The output of the above program shows the alternative values in the failure cases.

```rust
[src/main.rs:5:5] color = Srgba {
    red: 1.0,
    green: 0.0,
    blue: 1.0,
    alpha: 1.0,
}
[src/main.rs:13:5] failed_color_1 = Srgba {
    red: 0.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
}
[src/main.rs:16:5] failed_color_2 = Srgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
}
```

## Control Flow

Once you have a `Result`, `Option`, or other enum, there are a number of options for control flow to better handle the specific variants that matter for your application.

### `match`

Here we have a Bevy application with a single system that runs every frame.
The system queries for all of the `Camera` components that have been spawned and uses [`Query::single`](https://docs.rs/bevy/latest/bevy/prelude/struct.Query.html#method.single) to test if there is only a single `Entity` matching the `Query`.
In the case of 0 or 2+ matching entities, the returned value is the `Err` variant of the `Result`, which itself contains [an enum](https://docs.rs/bevy/latest/bevy/ecs/query/enum.QuerySingleError.html) indicating the reason for the error.
Using `match` allows running different branches of logic based on matching patterns.

```rust
use bevy::{ecs::query::QuerySingleError, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update)
        .run()
}

fn update(query: Query<&Camera>, mut commands: Commands) {
    match query.single() {
        Ok(camera) => {
            info_once!(?camera);
        }
        Err(QuerySingleError::NoEntities(debug_name)) => {
            commands.spawn(Camera2d);
            info!(?debug_name);
        }
        _ => {}
    }
}
```

In this case, if there is a `NoEntities` error, we use `Commands` to spawn an entity with a `Camera2d`, which requires `Camera`.
The next frame this system runs again, and matches the `Ok` value, giving us access to the `Camera` value.

### `let else`

An alternative to `match` is let-else.
`let-else` allows matching a specific pattern and handling all other cases in the else block.
The matched value can then be used in the rest of the function.
The caveat is that the else block _must_ diverge, which means using concepts like `return`, `break`, or `continue`, and can not fill a value into the match.

```rust
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update)
        .run()
}

fn update(query: Query<&Camera>, mut commands: Commands) {
    let Ok(camera) = query.single() else {
        commands.spawn(Camera2d);
        return;
    };
    let Some(target_info) = &camera.computed.target_info else {
        return;
    };

    info!(?target_info);
}
```

### if let

if-let is a similar concept to let-else that doesn't require the else block to diverge, and can have multiple additional conditions.
In turn, this means that _if_ an arm doesn't diverge, then each arm of the if-let must return the same type.

In this example, the first else block checks to make sure there are no other cameras spawned before spawning in a new camera instead of returning a value from the `if let`.
Values that _are_ returned from the `if let` are stored in the `computed_target_info` variable and can be used in the rest of the program.

```rust
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update)
        .run()
}

fn update(query: Query<&Camera>, mut commands: Commands) {
    let computed_target_info = if let Ok(camera) = query.single()
        && let Some(target_info) = &camera.computed.target_info
    {
        target_info
    } else if query.count() == 0 {
        commands.spawn(Camera2d);
        return;
    } else {
        return;
    };

    info!(?computed_target_info);
}
```

if-let enables using a concept called "let chaining".
In the example program, a series of lets are chained together using `&&`, using and matching on values that were previously matched.
If all of the let patterns match successfully, the relevant branch is executed.

## System Requirements

We've been using `Query::single` to validate query data and dispatch logic accordingly.
Since systems require certain data to be available to function, there are two ways to validate that data before a system runs:

- SystemParam validation
- Run conditions

Run conditions and fallible system parameters are covered in [run conditions](run-conditions.md)

## Errors in Bevy Apps

While Rust loosely buckets errors into "recoverable" and "unrecoverable", Bevy takes this a bit further.

- Unrecoverable errors, or panics, are for ensuring safety invariants are upheld, if you're writing unsafe code. Do not explicitly call panicking functions for anything else.
- Errors that might normally panic, such as those that would indicate a logic bug, can be returned directly from systems instead
- Errors which should _never_ panic, can be logged or handled locally with `match`, `if-let`, and other options

This works because Bevy contains a global, configurable error handler.

### Returning errors from Systems

Bevy's prelude contains a custom [`Result`](https://docs.rs/bevy/latest/bevy/ecs/error/type.Result.html) type alias that amounts to `Result<(), BevyError>` in the default case.
This can be used as the return type from systems.

{% callout() %}

Bevy's built-in error type [`BevyError`](https://docs.rs/bevy/latest/bevy/ecs/error/struct.BevyError.html) has a blanket `From` impl for any type that implements Rust’s [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html) trait.
This means you can write your own errors either manually or with a higher-level crate like [`thiserror`](https://docs.rs/thiserror/latest/thiserror/), and use those custom errors alongside `?` to return them in a system that returns Bevy's `Result` as we cover next.

{% end %}

Consider the `Query::single` case, which returns a `Result`.
In a system that returns `Result`, we can return errors by explicitly returning `Err` or using `?` on `Result`s.
`?` will unwrap the value if it is `Ok`, and return the value if it is an `Err`.

Note that the system must return `Ok(())` at the end as well, instead of `()`.

```rust
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update)
        .run()
}

fn update(query: Query<&Camera>) -> Result {
    let camera = query.single()?;

    info!(?camera);

    Ok(())
}
```

Since Bevy controls system execution, it also controls what happens to the `Result` we return.
A `Result::Err` returned from a system gets passed to Bevy's global error handler which panics by default:

```
Encountered an error in system `test_programs::update`: No entities fit the query bevy_ecs::system::query::Query<'_, '_, &bevy_camera::camera::Camera>
```

This can help raise issues quickly in development, but you may prefer a different approach to handling errors globally, especially in production.
The job of the global error handler is to decide how to log a global error, so Bevy provides [a series of preset handlers](https://docs.rs/bevy/latest/bevy/ecs/error/index.html) ranging from panicking to completely ignoring errors and all log levels in between.

Here is an example that changes the default global error handler to `warn`, one of the presets.

```rust
use bevy::{ecs::error::warn, prelude::*};

fn main() -> AppExit {
    let mut app = App::new();

    app.set_error_handler(warn);

    app.add_plugins(DefaultPlugins)
        .add_systems(Update, update)
        .run()
}

fn update(query: Query<&Camera>) -> Result {
    let camera = query.single()?;

    info!(?camera);

    Ok(())
}
```

Note that the error that is displayed has the same content, but doesn't panic and crash the application anymore; It is now a warn-level log.

```
WARN bevy_ecs::error::handler: Encountered an error in system `test_programs::update`: No entities fit the query bevy_ecs::system::query::Query<'_, '_, &bevy_camera::camera::Camera>
```

This allows the use of succinct error handling using `?` without crashing the application.

Global error handling is a large hammer that affects all systems, so you may prefer to keep the panicking default in development and use a different log level like trace in production.

### Per-system error handling

If you want to handle errors returned from a system in more complex ways, system piping can pass the return value of one system to another.
More details on system piping can be found in [systems](./systems.md).

In this case, the program can use the default global error handler and handle the error using a second system instead.
The second system in this case takes Bevy's `Result` as a system input using [`In`], and the value can be piped from `update` to `handle_error`.

```rust
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update.pipe(handle_error))
        .run()
}

fn update(query: Query<&Camera>) -> Result {
    let camera = query.single()?;

    info!(?camera);

    Ok(())
}

fn handle_error(In(input): In<Result>) {
    let Err(err) = input else { return };
    info_once!(?err);
}
```

The input type can be any type, which means we don't need to use the generic `BevyError`, and can instead use a custom error type or a pre-existing type like `QuerySingleError` that is returned by the operations the program uses.

This allows specific matching of those errors in the piped system.

```rust
use bevy::{ecs::query::QuerySingleError, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update.pipe(handle_error))
        .run()
}

fn update(query: Query<&Camera>) -> Result<(), QuerySingleError> {
    let camera = query.single()?;

    info!(?camera);

    Ok(())
}

fn handle_error(In(input): In<Result<(), QuerySingleError>>) {
    let Err(err) = input else { return };
    match err {
        QuerySingleError::NoEntities(debug_name) => todo!(),
        QuerySingleError::MultipleEntities(debug_name) => todo!(),
    }
}
```

[`In`]: https://docs.rs/bevy/latest/bevy/prelude/struct.In.html

## Errors in Commands

Commands can also return errors and be handled with [`queue_handled`](https://docs.rs/bevy/latest/bevy/prelude/struct.Commands.html#method.queue_handled) or silenced with [`queue_silenced`](https://docs.rs/bevy/latest/bevy/prelude/struct.Commands.html#method.queue_silenced).

Given a command that returns a `Result`, `queue_handled` allows passing in a handler that will receive the resulting `BevyError` as well as the [`ErrorContext`](https://docs.rs/bevy/latest/bevy/ecs/error/enum.ErrorContext.html) enum.

```rust
use bevy::{ecs::error::ErrorContext, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run()
}

#[derive(Resource)]
struct SomeData(u32);

fn startup(mut commands: Commands) {
    commands.queue_handled(
        |world: &mut World| -> Result {
            world
                .get_resource::<SomeData>()
                .ok_or("it wasn't inserted!".to_string())?;
            Ok(())
        },
        |error: BevyError, context: ErrorContext| {
            error!(?error, ?context);
        },
    );
}
```

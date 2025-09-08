+++
title = "Handling Errors"
insert_anchor_links = "right"
[extra]
weight = 6
status = 'hidden'
+++

When making a game, [errors] are inevitable: they may be temporary failures, lurking bugs, or simply part of normal control flow.

Generally, when your game encounters an error, you want to:

1. As the programmer, be made aware of the problem, so you can fix any bug that has been discovered.
2. Recover as gracefully as possible, and keep the game running as if nothing ever happened.

Unfortunately, these goals are often in tension!

## Deliberately panicking in Rust

The easiest way to make sure that you are made aware of any errors is to simply panic your program if they are encountered,
causing your game to crash.

Rust generously gives you a variety of ways to quickly and easily crash your program:

- [`panic!`]: Causes your program to crash, with a custom error message.
- [`unwrap`]: Converts an [`Option<T>`] into a bare `T` but only if it is `Some`. Crashes your program if it was `None`.
  - Also works on [`Result<T, E>`], converting `Ok<T>` into `T` but crashing if it was `Err<E>`.
- [`expect`]: Just like unwrap, but with a custom error message.
  - Note that if you've defined a nice error type, `unwrap` will give you good error messages already.
- [`assert!`]: Panic if the expression does not evaluate to true. This is used for checking that important preconditions are true.
  - There's a number of related macros, but [`debug_assert!`] and [`assert_eq!`] are the most helpful.
- [`todo!`]: Tells the compiler to stop complaining and compile your code, but crashes when the statement is reached.

While there are other ways to suddenly crash your program ([out of bound array indexing] is common), these are the primary tools used for deliberate panics.

Early in development, panicking is often the right choice!
It's super easy to do, it quickly surfaces any assumptions, and by virtue of being extremely obvious and annoying,
forces you to actually fix your problems.

Let's demonstrate how you might use this method in a simple Bevy system:

```rust, hide-lines=1-2
# use bevy::prelude::*;
#
#[derive(Component)]
struct Player;

fn print_player_name(mut player_query: Query<&Name, With<Player>>){
    // Query::single returns a Result, which is an Err if there are 0 or 2+ matching entities
    // Most of the time, we expect there to be a single player, so we can get away with just calling .unwrap()
    let player_name = player_query.single().unwrap();
    // This is one of Bevy's logging macros!
    // We go into them in more detail below
    info!("The player's name is {player_name}.");
}
```

## Defensive programming in Rust

As your project matures, you will probably find yourself growing frustrated with all of the crashes,
especially when seemingly unrelated code crashes when you make a change.

The primary cause of unexpected panics in Rust is a violated assumption,
resulting in an `unwrap` call on an `Option::None` or `Result::Err`.

The solution is deceptively simple: don't assume that those values contain the actual type you care about.
There are two steps to implementing this:

1. Detecting the variant of the `Option` or `Result`.
2. Doing something other than panicking if it's not the one you were hoping for.

To detect and gracefully unwrap the variant, Rust gives you a range of helpful tools, which work on any [`enum`]:

- [`match`]: works like a powerful multi-armed if statement, analogous to `switch` statement in other languages
- [`if let` statements]: takes the `if` branch if the variant matches the supplied pattern
- [`let else` statements]: assigns the value to a new variable if the pattern matches, taking the `else` branch if it didn't
- [`matches!`]: evaluates to `true` if the supplied pattern matches
  - a handy tool for working with enums in general, but not very useful for error handling specifically

But getting the desired value out is only half the challenge: what do you do with the error?
There are a few good options:

- simply return early from the function
- log the problem, choosing from [`trace!`], [`debug!`], [`info!], [`warn!], or [`error!] based on the severity
- fill in a default value, using [`unwrap_or_default`], [`unwrap_or`] or [`unwrap_or_else`]
- attempt to retry later
- bubble the problem up to your caller, as discussed by the next section

Let's use the same example from above, and try out each of these unwrapping alternatives.

```rust, hide-lines=1-2
# use bevy::prelude::*;
#
#[derive(Component)]
struct Player;

fn print_player_name_match(mut player_query: Query<&Name, With<Player>>){
    let player_name = match player_query.single() {
        Ok(name) => name,
        // We can choose to match again, drilling down into the error
        Err(err) => match err {
            // The data inside of our enum variants can be discarded with _
            QuerySingleError::NoEntities(_) => {
                // This is an expected failure: the player may be dead or not yet spawned
                // So we can silently return early
                return;
            },
            QuerySingleError::MultipleEntities(_) => {
                // Something very weird has happened if we're here
                // So we're logging the error to alert us to the bug.
                error!("Multiple player entities were found!");
                return;
            }
        }
    };
    
    info!("The player's name is {player_name}.");
}

fn print_player_name_if_let(mut player_query: Query<&Name, With<Player>>){
    // We can add an else statement in case the pattern matching fails,
    // but we don't have to if we're fine with just silently returning without doing work
    if let Some(player_name) = player_query.single() {
        info!("The player's name is {player_name}.");
    };
}

fn print_player_name_let_else(mut player_query: Query<&Name, With<Player>>){
    let Some(player_name) = player_query.single() else {
        return;
    }

    info!("The player's name is {player_name}.");
}

fn print_player_name_unwrap_or(mut player_query: Query<&Name, With<Player>>){
    // If `Name` had a `Default` implementation, we could use `unwrap_or_default` instead
    let player_name = player_query.single().unwrap_or("The Unnamed Hero");

    info!("The player's name is {player_name}.");
}

```

{% callout(type="info") %}

That's a lot of options for handling error variants gracefully! Which one should you pick?
While the choice is often stylistic, there's a few key characteristics to consider:

- `match`: great for drilling down into errors, verbose, extremely flexible
- `if let`: nice and short but increases indentation, making it a poor choice when you have multiple values to check
- `let else`: flexible and fairly terse, but can get repetitive
- `?` error bubbling: very terse, but requires some setup around system return types and error handlers, discussed below

Whenever a good default value exists, use that and carry on happily.
But if none exists, we recommend taking the time to use `?`-based error bubbling as your default strategy.
When you want to actually respond to errors in a nuanced way,
rather than just logging them and moving on, `match` is a very powerful, straightforward tool that will do everything you need.

{% end %}

[`enum`]: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
[`match`]: https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
[`matches!`]: https://doc.rust-lang.org/std/macro.matches.html
[`if let` statements]: https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
[`let else` statements]: https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html

## Bubbling errors upward

Rust gives you one other great option for dealing with errors: make it someone else's problem!
Rather than returning `()` (the default, implicit return type), we can pass the error to our caller and move on.

This pattern is so common and helpful that Rust has special syntax to do exactly this: [`?`].
When used on a [`Result`], errors are converted into the return type and then returned, while `Ok` values are unwrapped.
This can be very convenient!

```rust, hide-lines=1-2
# use bevy::prelude::*
#
// This is a special catch-all Error type exported in Bevy's prelude
// We discuss this type  and what happens to the error below
fn print_player_name_question_mark(mut player_query: Query<&Name, With<Player>>) -> Error {
    let player_name = player_query.single()?;

    info!("The player's name is {player_name}.");
}
```

Look at how terse that is! When handling multiple `Result`s at once, this really adds up.

However, the `?` operator cannot be used with `Option` directly:
instead, you must convert your type into something that can be converted into your return type.
The most convenient way to do this is usually with [`Option::map_or`].

While you could use a fancy custom error type for this, strings implement the [`Error`] trait!
A quick `my_option.map_or("quick message for logging")?` works great.

{% callout(type="info") %}

As your programs grow more sophisticated and you start writing complex data types and helper functions,
you will develop your own operations that can fail.

In Rust, the convention is to define a new type (usually an `enum`), which explains exactly why and how the failure occurred.
You should implement the [`Error`] trait for these types, along with any conversions from contained error types via the [`From`] trait.

Bevy uses the [`thiserror`] crate to make this easier. You probably should too!

[`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`thiserror`]: https://docs.rs/thiserror/latest/thiserror/
{% end %}

[`?`]: https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html
[`Option::map_or`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or

## Handling errors in systems

## Handling errors in commands

## Avoiding panics in production

[Errors]: https://doc.rust-lang.org/book/ch09-00-error-handling.html
[`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
[`unwrap`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
[`expect`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect
[`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
[`assert_eq!`]: https://doc.rust-lang.org/std/macro.assert_eq.html
[`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
[`todo!`]: https://doc.rust-lang.org/std/macro.todo.html
[out of bound array indexing]: https://rust-lang.github.io/rust-clippy/master/#indexing_slicing
[`Option<T>`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Result<T, E>`]: https://doc.rust-lang.org/std/result/enum.Result.html

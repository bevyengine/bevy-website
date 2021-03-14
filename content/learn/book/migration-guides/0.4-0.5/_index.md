+++
title = "0.4 to 0.5"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
[extra]
long_title = "Migration Guide: 0.4 to 0.5"
+++

<!-- TODO: link to release blog post here -->

## `commands: &mut Commands` SystemParam is now `mut commands: Commands`

```rust
// 0.4
fn foo(commands: &mut Commands) {
}

// 0.5
fn foo(mut commands: Commands) {
}
```

Systems using the old `commands: &mut Commands` syntax in 0.5 will fail to compile when calling `foo.system()`.

This change was made because `Commands` now holds an internal `World` reference to enable safe Entity allocations.

Note: The internal `World` reference requires two lifetime parameters to pass Commands into a non-system function: ```commands: &'a mut Commands<'b>```

## Systems allow a maximum of 12 top-level SystemParams, down from 15

```rust
// 0.4
fn foo(r1: Res<Thing1>, r2: Res<Thing2>, r3: Res<Thing3>, r4: Res<Thing4>, ... r15: Res<Thing15>) {
}

// 0.5
fn foo(r1_thru_3: (Res<Thing1>, Res<Thing2>, Res<Thing3>), r4: Res<Thing4>, ... r15: Res<Thing15>) {
}
```

System functions with more than 12 arguments will no longer compile, as SystemParams rely on Rust's default impl for tuples.

To work around this limitation (and improve function signature readability), systems can use nested tuples, as shown above, or leverage [derived parameters](https://github.com/bevyengine/bevy/blob/main/examples/ecs/system_param.rs).

## Commands insert() API is now used for a single component

```rust
// 0.4
// component
commands.insert_one(entiy, MyComponent)
commands.insert(entity, (MyComponent,))
// bundle
commands.insert(entity, Bundle)


// 0.5
// component
commands.insert(entity, MyComponent)
// bundle
commands.insert_bundle(entity, MyBundle)
```

Instead of using `commands.insert_one()` for a lone component, it is now possible to simply call `commands.insert()`.

This means that `commands.insert()` will no longer accept a bundle as an argument. For this, the new API is `commands.insert_bundle()`.

This change helps to clarify the difference between components and bundles, and brings `Commands` into alignment with other Bevy APIs. It also eliminates the confusion associated with calling `commands.insert()` on a tuple for the single-component case.

## {{rust_type(type="struct" crate="bevy_core" version="0.5.0" name="Timer" no_mod=true)}} uses `Duration`

```rust
// 0.4
if timer.tick(time.delta_seconds()).finished() { /* do stuff */ }
timer.elapsed() // returns a bool

// 0.5
if timer.tick(time.delta()).finished() { /* do stuff */ }
timer.elapsed() // returns a `Duration`
```

Most of the methods of {{rust_type(type="struct" crate="bevy_core" version="0.5.0" name="Timer" no_mod=true)}}
now use `Duration` instead of `f32`.

This change allows timers to have consistent, high precision. For convenience, there is also an
`elapsed_secs` method that returns `f32`.  Otherwise, when you need an `f32`, use the
`as_secs_f32()` method on `Duration` to make the conversion.

## New: {{rust_type(type="struct" crate="bevy_core" version="0.5.0" name="Stopwatch" no_mod=true)}}

```rust
// 0.5
let mut stopwatch = Stopwatch::new();
stopwatch.tick(Duration::from_secs_f32(1.5));
stopwatch.pause();
stopwatch.tick(time.delta()); // does nothing while stopwatch is paused
assert_eq(stopwatch.elapsed(), Duration::from_secs_f32(1.5));
stopwatch.reset();
```

{{rust_type(type="struct" crate="bevy_core" version="0.5.0" name="Stopwatch" no_mod=true)}} can be
used to measure elapsed time.


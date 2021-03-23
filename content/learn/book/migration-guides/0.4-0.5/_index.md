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

This change was made because {{rust_type(type="struct" crate="bevy_ecs" version="0.5.0" name="Commands" no_mod=true)}}
now holds an internal {{rust_type(type="struct" crate="bevy_ecs" version="0.5.0" name="World" no_mod=true)}}
reference to enable safe entity allocations.

Note: The internal {{rust_type(type="struct" crate="bevy_ecs" version="0.5.0" name="World" no_mod=true)}} reference requires two lifetime parameters to pass Commands into a non-system function: ```commands: &'a mut Commands<'b>```

## Systems allow a maximum of 12 top-level `SystemParam`s, down from 15

```rust
// 0.4
fn foo(r1: Res<Thing1>, r2: Res<Thing2>, r3: Res<Thing3>, r4: Res<Thing4>, ... r15: Res<Thing15>) {
}

// 0.5
fn foo(r1: Res<Thing1>, r2: Res<Thing2>, r3: Res<Thing3>, r4: Res<Thing4>, ... r12: Res<Thing12>) {
// or
fn foo(r1_thru_3: (Res<Thing1>, Res<Thing2>, Res<Thing3>), r4: Res<Thing4>, ... r15: Res<Thing15>) {
}
```

System functions with more than 12 arguments will no longer compile, as `SystemParam`s rely on Rust's default impl for tuples.

To work around this limitation (and improve function signature readability), systems can use nested tuples, as shown above, or leverage [derived parameters](https://github.com/bevyengine/bevy/blob/main/examples/ecs/system_param.rs).

## {{rust_type(type="struct" crate="bevy_ecs" version="0.5.0" name="Commands" no_mod=true)}} `insert()` API is now used for a single component

```rust
// 0.4
// component
commands.insert_one(entity, MyComponent)
commands.insert(entity, (MyComponent,))
// bundle
commands.insert(entity, Bundle)


// 0.5
// component
commands.insert(entity, MyComponent)
// bundle
commands.insert_bundle(entity, MyBundle)
```

Instead of using `commands.insert_one()` for a single component, use `commands.insert()`.

This means that `commands.insert()` will no longer accept a bundle as an argument. For bundles, use `commands.insert_bundle()`.

This change helps to clarify the difference between components and bundles, and brings {{rust_type(type="struct" crate="bevy_ecs" version="0.5.0" name="Commands" no_mod=true)}} into alignment with other Bevy APIs. It also eliminates the confusion associated with calling `commands.insert()` on a tuple for the single-component case.

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

## Simplified Events

```rust
// 0.4
fn event_reader_system(
    mut my_event_reader: Local<EventReader<MyEvent>>,
    my_events: Res<Events<MyEvent>>,
) {
    for my_event in my_event_reader.iter(&my_events) {
        // do things with your event
    }
}

// 0.5
fn event_reader_system(mut my_event_reader: EventReader<MyEvent>) {
    for my_event in my_event_reader.iter() {
        // do things with your event
    }
}
```
You no longer need two system parameters to read your events. One `EventReader` is sufficient.

Following the above example of using an `EventReader` to read events, you can now use `EventWriter` to create new ones.
```rust
// 0.4
fn event_writer_system(
    mut my_events: ResMut<Events<MyEvent>>,
) {
    my_events.send(MyEvent);
}

// 0.5
fn event_writer_system(
    mut my_events: EventWriter<MyEvent>
) {
    my_events.send(MyEvent);
}
```

## `AppBuilder::add_resource` is now called `AppBuilder::insert_resource`

This is a small change to have function names on `AppBuilder` consistent with the `Commands` API.

## TextBundle

This bundle has been reworked to allow multiple differently-styled sections of text within a single bundle. `Text::with_section` was added to simplify the common case where you're only interested in one text section.

```rust
// 0.4
TextBundle {
    text: Text {
        value: "hello!".to_string(),
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        style: TextStyle {
            font_size: 60.0,
            color: Color::WHITE,
            ..Default::default()
        },
    },
    ..Default::default()
}

// 0.5
TextBundle {
    text: Text::with_section(
        "hello!",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 60.0,
            color: Color::WHITE,
        },
        TextAlignment::default()
    ),
    ..Default::default()
}
```

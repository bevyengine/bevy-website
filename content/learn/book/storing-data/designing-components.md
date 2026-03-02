+++
title = "Designing components"
insert_anchor_links = "right"
[extra]
weight = 2
status = 'hidden'
+++

Simply understanding the basic operations of components isn't enough to immediately understand how to
model that elaborate game idea in real, functioning code.
Getting the core data model correct makes it much easier to write and revise flexible, fast and correct code.
In Bevy, this usually means what data goes on which components.

This chapter covers some high-level guidance for what you should consider when designing your components,
and additional tools that are helpful when considering this organization.

## Guidance for structuring components

Over time, the Bevy community has converged on a few standard pieces of advice for how to structure and define component data:

- Try to keep your components relatively small
  - Common functionality can be handled by putting it on a shared [Required Component], discussed below
  - Small modular systems based on common behavior work well
  - Reducing the amount of data stored improves cache performance and system-parallelism
  - Group properties together within a single component if you need to maintain invariants (such as current life is always less than or equal to max life)
  - Additionally, group properties together within a single component if you need methods that operate across several pieces of data (e.g. computing the distance between two points)
- Simple methods on components are a good tool for clean, testable code
  - Logic that is inherent to how the component works (like rolling dice or healing life points) is a great fit
  - Logic that will only be repeated once generally belongs in systems
  - Methods make it easier to understand the actual gameplay logic in your systems, and fix bugs in a single place
- **Marker components** (using unit structs) are incredibly valuable for extending your design
  - It is very common to want to quickly look for "all entities that are a `Tower`", or "all entities that are `Chilled`"
  - Filtering by component presence/absence is (generally) faster and clearer than looping through a list of boolean values
  - Try to model meaningful groups at several levels of abstraction / along multiple axes: e.g. `Unit`, `Ant`, `Combatant`
- Enum components are very expressive, and help reduce bugs
  - Enums can hold different data in each variant, allowing you to capture information effectively
  - If you have a fixed number of options for a value, store it as an enum
- Implementing traits like [`Add`] or [`Display`] can provide useful behavior in an idiomatic way
- Use [`Deref`] and [`DerefMut`] for tuple structs with a single item ([newtypes])
  - This allows you to access the internal data with `*my_component` instead of `my_component.0`
  - More importantly, this allows you to call methods that belong to the wrapped type directly on your component
- Consider defining traits for related components
  - This allows you to ensure a consistent interface
  - This can be very powerful in combination with generic systems that use trait bounds

## Required components

In the previous chapter, we saw how simple components can be composed to form complex
entities. Often times, a given component will need certain other components in order to do anything
useful. Drawing from the example in the previous section, it probably doesn't make much sense to
mark an entity as a `Combatant` if it doesn't also have `Life`.

To help prevent errors of omission, and to simplify the task of spawning, Bevy lets you declare
that a given component depends on the presence of another component:

```rust,hide_lines=1
# use bevy::ecs::prelude::*;
#[derive(Component)]
struct Life(u8);

#[derive(Component)]
#[require(Life)]
struct Combatant;
```

Here, we've told the engine that `Combatant` requires `Life`: "Whenever you insert a `Combatant`
component, insert a `Life` component as well."

However, we didn't specify in the `require` clause how _much_ `Life` we should give the new entity.
As a result, what it got was `Life::default()`. We can do better by supplying an initializer
expression:

```rust,hide_lines=1
# use bevy::ecs::prelude::*;
#[derive(Component)]
struct Life(u8);

#[derive(Component)]
#[require(Life(10))]
struct Combatant;
```

This means that every `Combatant` will get 10 `Life` by default. But what if we want to give the
combatant a different amount of life? That's OK! You see, the "required" clause only sets a
default. You can override the default by specifically including a `Life` component, either
as part of the initial bundle while spawning, or inserted at a later point.

Alternatively, you can use `=` to assign an initial value to the component:

```rust,hide_lines=1
# use bevy::ecs::prelude::*;
#[derive(Component)]
struct Life(u8);

#[derive(Component)]
#[require(Life = initial_health())]
struct Combatant;
```

The `require` macro is the easiest way to declare required components, but you can also define
the required components programmatically by manually implementing the `Component` trait.


[Required Component]: ../required-components
[`Add`]: https://doc.rust-lang.org/std/ops/trait.Add.html
[`Display`]: https://doc.rust-lang.org/std/path/struct.Display.html
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[newtypes]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html


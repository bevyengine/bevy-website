+++
title = "Required Components"
insert_anchor_links = "right"
[extra]
weight = 2
status = 'hidden'
+++

In the previous section, we saw how simple components can be composed to form complex
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

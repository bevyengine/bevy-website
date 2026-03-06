+++
title = "Designing components"
insert_anchor_links = "right"
[extra]
weight = 2
status = 'hidden'
+++

In previous chapters, you've been introduced to the basic mechanics of how to work with components.
But if you want to build complex projects, you should think carefully about how you model your domain as data.
In Bevy, this usually means deciding what data goes on which components.  

This chapter covers some high-level guidance for what you should consider when designing your components,
and additional tools that are helpful when considering this organization.

## Guidance for structuring components

Over time, the Bevy community has converged on a few standard pieces of advice for how to structure and define component data:

- Try to keep your components relatively small
  - Common functionality can be handled by putting it on a shared "required component", discussed below
  - Then, create small modular systems based on common behavior
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
useful. 
As an example, it probably doesn't make much sense to mark an entity as a `Combatant` if it doesn't also have `Life`.

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

This means that every `Combatant` will get 10 `Life` by default.
But this is just a default: that can be overridden by explicitly including a `Life` component, either
as part of the initial bundle while spawning, or by insertion at a later point.

The `require` macro is the easiest way to declare required components, but you can also define
the required components programmatically by manually implementing the `Component` trait.
Check the `Component` trait documentation for up-to-date guidance.


[`Add`]: https://doc.rust-lang.org/std/ops/trait.Add.html
[`Display`]: https://doc.rust-lang.org/std/path/struct.Display.html
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[newtypes]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html

## Reusable logic for components

You may have heard that, in ECS, components are "just data", and cannot or should not store logic.
Generally this is sound advice: performing logic in systems based on component presence/absence and values
is fast and can lead to flexible game design that creates interesting emergent behavior.

But, like all advice, there are limits to its validity.
Being able to perform common or complex operations on data in a consistent way is the basis of a good abstraction.
Define these operations once, use everywhere, and then when you inevitably want to revise it, you only need to update it once.

There are a number of tools available to help you reuse component-related logic, listed in order of increasing complexity.
In most cases, simple traits and methods suffice, but it's helpful to be aware of more powerful tools.

### Methods and traits for `Component` types

Components are just structs (or enums, or tuple structs)!
Like any Rust type, you can define methods for these types, and implement traits for them.

```rust
#[derive(Component)]
pub struct Life {
  current: u32,
  max: u32,
}

impl Life {
  fn set(&mut self, new: u32) {
    self.current = if new > self.max {
      self.max
    } else {
      new
    }
  }
}

impl Sub<u32> for Life {
  type Output = Life;

  fn sub(self, rhs: u32) -> Self::Output {
    self.current = self.current.saturating_sub(rhs)
  }
}
```

Keeping fields private can be very useful to ensure that key invariants are upheld.
In the above example, we use this to ensure that the *current* life value cannot exceed the *max* life value.  


### Storing functions inside of components

Occasionally you might want to store arbitrarily complex, one-off logic on your components.
This usually comes up in the context of UI or script-like behavior,
where you want each instance of a similar object to perform easily-customized behavior
in response to some cue, like a button being pressed or the player interacting with an object.
While the patterns described here will be relatively slow (due to poor cache locality) and hard to debug,
this pattern can be easier to work with than approaches which rely on a huge number of marker components.

The core pattern here is to store an owned [trait object](https://doc.rust-lang.org/reference/types/trait-object.html) inside of your component, usually in a `Box`.

The simplest example of this is to store a `Box<dyn Command>`:

```rust
#[derive(Component)]
struct ClickableProp {
  on_click: Box<dyn Command + Clone>
}

// This is an observer!
// We're listening to picking events here!
fn handle_clickable_props(trigger: On<Pointer<Click>>, query: Query<&ClickableProp>, mut commands: Commands){
  let Ok(clickable_prop) = query.get(trigger.entity()) else {
    return;
  };

  commands.queue(clickable_prop.on_click)
}
```

This can be repeated with other traits: `Event` and `Message` are quite powerful if you want to hook into existing logic.

Storing [one-shot systems](../control-flow/systems.md) can be even more expressive.
See the [callbacks example](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/callbacks.rs) for a demonstration of this pattern.

If your benchmarks show that you need to make this pattern more performant, you can consider swapping to [function pointers](https://doc.rust-lang.org/std/primitive.fn.html), at the cost of some flexibility (no trait methods, cannot capture the environment during creation).

### Accessing data beyond the component

As your logic grows in complexity, you may find yourself repeatedly needing to access data from multiple entities,
components or even resources simultaneously.
For example, you might find that you need to update a tilemap index while revising tile data,
animate a goblin by mutating both its weapon and the root entity,
or checking if an ability can be used by examining available mana, cooldowns and range to the target.

Duplicating this complex logic can be both error-prone and tedious!
Your first thought should be to ask "Can we combine this data into a single component",  
but that's not always feasible (sometimes for reasons outside of your control).

One solution is to abstract complex ECS lookups like this by creating our own custom `QueryData`, `QueryFilter`
and `SystemParam` types, using the provided derive macros.
`QueryData` and `QueryFilter` are useful when the data is in separate components on the same entity and is quite composable (you can always add more terms to your `Query`!),
while `SystemParam` is best reserved for when you need to access distinct entities, resources, commands, messages or other forms of data in the same logic.

While simply defining these types can save boilerplate and reduce error,
they become dramatically more powerful when we implement methods on them,
which automatically incorporate data across disparate sources into a single atomic operation.
When working with custom `QueryData` types, you should be aware that you can implement methods on
the generated (and doc-hidden) `QueryData::Item` types, allowing you to define operations for a single element of your complex queries.

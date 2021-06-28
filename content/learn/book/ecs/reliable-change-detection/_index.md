+++
title = "Reliable change detection"
weight = 6
template = "book-section.html"
page_template = "book-section.html"
+++

Bevy allows you to respond to the addition of or changes to specific component types using `Added<T>` and `Changed<T>` query filters.
These are incredibly useful, allowing you to:

- automatically complete initialization of components
- keep data in sync
- save on work by only operating on entities where the relevant data has been updated
- automatically respond to changes in the data of UI elements

A component will be marked as "added" on a per-system basis if it has been added to that entity (via component insertion or being spawned on an entity) since the last time that system ran.
Similarly, a component will be marked as "changed" on a per-system basis if it has been added or mutably dereferenced since the last time that system ran.
As you (almost always) need to mutably dereference data out of its wrapper in order to mutate it, this gives an accurate (and fast!) indication that the data may have changed.

Let's take a look at an example that uses change detection to respond to changes in our data:

TODO: write example code
```rust

```

### Change detection for resources

Change detection works for resources too!
Use the `is_changed()`() and `is_added()` methods on any resource to check if they've been added or changed, following exactly the same rules.

### The details of change detection

Change detection in Bevy works via a custom implementation of the `DerefMut` of `Mut` and `ResMut`, our mutable wrappers for components and resources respectively.
As a result:

1. Changes won't be flagged when you use [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html). You can (and should!) manually flag the data as having changed using `set_changed()` when you do this.
2. Changes will be flagged whenever you mutably access a component or resource, even if you don't change its value. Only dereference the data if you know you're going to change it to avoid false positives caused in this way.

### Removal detection

TODO: discuss removal detection

+++
title = "Change Detection"
insert_anchor_links = "right"
[extra]
weight = 8
status = 'hidden'
+++

Change detection is built into Bevy's Entity Component System (ECS). Each time a
[`Component`] or [`Resource`] is modified, Bevy marks the item as changed. This mechanism helps
optimize performance by avoiding unnecessary calculations when no relevant changes have
occurred. Change detection can also be used to trigger actions in response to changes, such as
synchronizing data between two contexts.

In this section, we'll explore how Bevy tracks changes and how you can leverage this
feature in your game development workflow.

## Filtering

You can configure queries to filter out entities unless certain components have been modified.

The [`Added<T>`] query filter detects new component instances, either if the component was added to
an existing entity, or a new entity with that component was spawned.
This is also triggered if a component is reinserted on an entity that already had it.

The [`Changed<T>`] query filter detects when a component has been changed. Adding a new component
counts as "changed" - in otherwords, this is a superset of [`Added<T>`].

```rust
// Detecting added components
fn detect_added_position(query: Query<Entity, Added<Position>>) {
    for entity in query.iter() {
        println!("Entity {:?} was just given a Position component", entity);
    }
}

// Detecting changed components.
fn detect_changed_position(query: Query<(Entity, &Position), Changed<Position>>) {
    for (entity, position) in query.iter() {
        println!("Entity {:?} position changed to {:?}", entity, position);
    }
}
```

Removing components works differently, see the section below.

## Checking for changes

In some cases, you may want to know whether or not an entity's components have changed, but at
the same time want to access all the entities, even the ones that have _not_ changed.

You can use the special [`Ref<T>`] query parameter instead of `&` for immutable access. This provides
methods [`.is_changed()`] and [`.is_added()`] which return true if the component was changed or
added, respectively.

For mutable access, you don't need to do anything special to get the change detection methods. Using
`&mut` in a query parameter causes Bevy to use the special [`Mut<T>`] type, which also supports change
detection.

```rust
// Using Ref<T> for change detection with immutable access
fn check_position_changes(query: Query<(Entity, Ref<Position>)>) {
    for (entity, position) in query.iter() {
        if position.is_changed() {
            println!("Entity {:?} position changed to {:?}", entity, position);
        } else {
            println!("Entity {:?} position remained {:?}", entity, position);
        }
    }
}

// Using Mut<T> for change detection with mutable access
fn check_and_update_position(mut query: Query<(Entity, Mut<Position>)>) {
    for (entity, mut position) in query.iter_mut() {
        if position.is_changed() {
            println!("Entity {:?} position changed to {:?} before update", entity, *position);
            position.x += 1.0; // This will mark the component as changed
        }
    }
}
```

{% callout(type="info") %}
Performance-wise, there's no real difference between using query filters and using the change
detection methods: The [`Added<T>`] and [`Changed<T>`] query filters cause the iterator to skip
over entities that have not changed, but they don't reduce the number of entities that get fetched
by the query.
{% end %}

## Resources

The [`Res<T>`] and [`ResMut<T>`] provide the same [`.is_added()`] and [`.is_changed()`] methods as
components do:

```rust
fn detect_changed_score(score: Res<Score>) {
    if score.is_changed() {
        println!("The score changed to {}", score.0);
    }
}
```

## Removed Components

Change detection works differently for removed components, since the component (and possibly the
entity) no longer exists!

To detect when components are removed, you can use the [`RemovedComponents`] param:

```rust
fn detect_removed_position(mut removed: RemovedComponents<Position>) {
    for entity in removed.iter() {
        println!("Entity {:?} just lost its Position component", entity);
    }
}
```

{% callout(type="warning") %}
It's generally better to use an [`OnRemove`] observer or a component hook to detect removals.
This has a number of advantages over using [`RemovedComponents`]:

- You get access to the component values being removed.
- [`RemovedComponents`] can miss component removals when used in `FixedUpdate`.

{% end %}

Bevy does not provide any API for detecting when resources are removed.

## What gets detected?

Change detection is triggered when a component or resource is mutably dereferenced. [`Mut<T>`] and
[`ResMut<T>`] implement `DerefMut`, with an implementation that marks the item as changed.

Simply reading components via a mutable query, or resources via [`ResMut`], will _not_ trigger change
detection. But dereferencing the component, or taking a mutable borrow, will.

```rust
fn update_player_health(mut query: Query<(Entity, &mut Health), With<Player>>) {
    // Calling `.iter_mut()` does not, by itself, trigger change detection.
    for (entity, mut health) in query.iter_mut() {
        // Players at zero health will *NOT* be marked as changed here.
        if health.0 > 0.0 {
            // This marks the component as changed.
            health.0 -= 10;
        }
    }
}
```

When you mutate a component, Bevy does not check whether the old value is actually different from
the new value - it will always trigger change detection. If you want to avoid that, you'll need
to check it yourself:

```rust
fn update_player_health(mut query: Query<(Entity, &mut Health), With<Player>>) {
    for (entity, mut health) in query.iter_mut() {
        let new_health = (health.0 - 10.0).max(0.0);
        if health.0 != new_health {
            // This marks the component as changed.
            health.0 = new_health;
        }
    }
}
```

You can also use the [`set_if_neq()`] and similar helper methods for this, which update the component
only if the new value is different (this requires that the component support `PartialEq`):

```rust
fn update_player_health(mut query: Query<(Entity, &mut Health), With<Player>>) {
    for (entity, mut health) in query.iter_mut() {
        let new_health = (health.0 - 10.0).max(0.0);
        // This marks the component as changed if `new_health` is different.
        health.set_if_neq(Health(new_health));
    }
}
```

Change detection applies to each ECS system separately: a system will see whatever changes occurred
since the last time the system ran. This includes systems that only run sometimes (such as when
using [states] or [run conditions]), so you do not need to worry about "missing" changes.

{% callout(type="info") %}
Internally, Bevy stores an "engine tick count" with every component and resource that marks the
last time that it was updated. When a system calls [`.is_changed()`], it compares the tick count
of the component with the tick count of the last time the system was run.
{% end %}

One thing to beware of is potential 1-frame delay, if the system that is causing the change
runs after the system that is checking for changes. You may need to pay attention to how you [run schedules]
or use [explicit system ordering] to prevent this.

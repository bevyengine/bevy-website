+++
title = "Bevy 0.10"
date = 2023-03-04
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), I'm happy to announce the **Bevy 0.10** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.10**, check out our [0.9 to 0.10 Migration Guide](/learn/book/migration-guides/0.9-0.10/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Headliner Feature**: Description here.

## Section Template

<div class="release-feature-authors">authors: @Foo, @Bar</div>

Description here.

## `Ref<T>` Queries

<div class="release-feature-authors">authors: @Guvante, @JoJoJet</div>

Since Bevy 0.1, `Mut<T>` has used to enable change detection (along with related types like `ResMut<T>`). It's a simple wrapper type that provides mutable access to a component, and marks a change any time it is mutated. It also provides access to change tick metadata, which allows you to react to changes made outside of the current system.

Now, the change detection family has grown with `Ref<T>`, which is the immutable variant of `Mut<T>`. It provides access to a component alongside its change tick metadata.

```rust
use bevy::prelude::*;

fn inspect_system<T: Debug>(q: Query<Ref<T>>) {
    // Iterate over each component of type `T`, and log its changed status.
    for val in &q {
        if val.is_changed() {
            println!("Value `{val:?}` is unchanged.");
        } else {
            println!("Value `{val:?}` was last changed at tick {}.", val.last_changed());
        }
    }
}
```

Now that we have `Ref<T>`, we are deprecating `ChangeTrackers<T>`, which is the old, more situational way of immutably accessing a component's change ticks within a system. This type will be removed in Bevy 0.10.

## What's Next?

* **Some Work In Progress Feature**: Description here.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](Bevy ) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

A huge thanks to the **X contributors** that made this release (and associated docs) possible! In random order:

* @Foo

## Full Change Log

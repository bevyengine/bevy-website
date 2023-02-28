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

## ECS Optimizations

<div class="release-feature-authors">authors: @james7132, @JoJoJet</div>

ECS underlies the entire engine, so eliminating overhead in the ECS results in engine-wide speedups. In **Bevy 0.10**, we've found quite a few areas where we were able to massively reduce the overhead and improve CPU utilization for the entire engine.

In [#6547](https://github.com/bevyengine/bevy/pull/6547), we enabled [autovectorization](https://en.wikipedia.org/wiki/Automatic_vectorization) when using `Query::for_each`, and its parallel variants. Depending on the target architecture the engine is being compiled for, this can result in a 50-87.5% speed up in query iteration time. In 0.11, we may be extending this optimization to all iterator combinators based on `Iterator::fold`, like `Iterator::count`. See [this PR](https://github.com/bevyengine/bevy/pull/6773) for more details.

In [#6681](https://github.com/bevyengine/bevy/pull/6681), by tightly packing entity location metadata and avoiding extra memory lookups, we've significantly reduced the overhead when making random query lookups via `Query::get`, seeing up to a 43% reduction in the overhead spent in `Query::get` and `World::get`.

In [#6800](https://github.com/bevyengine/bevy/pull/6800) and [#6902](https://github.com/bevyengine/bevy/pull/6902), we've found that rustc can optimize out compile-time constant branches across function boundaries, moving the branch from runtime to compile time, has resulted in up to a 50% reduction in overhead when using `EntityRef::get`, `EntityMut::insert`, `EntityMut::remove`, and their variants.

In [#6391](https://github.com/bevyengine/bevy/pull/6391), we've reworked `CommandQueue`'s internals to be more CPU-cache friendly, which has shown up to a 37% speedup when encoding and applying commands.

## Adaptive Batching for Parallel Query Iteration

<div class="release-feature-authors">authors: @james7132</div>

In 0.9 and before, `Query::par_for_each` required providing a batch size, which would be used to chunk the matched entities into batches for parallel processing. This often resulted in users just randomly picking a value and rolling with it, or fine tuning the value based on their development machines. Unfortunately, the most effective value is dependent on the runtime environment (i.e. how many logical cores does a player's computer have) and the state of the ECS World (i.e. how many entities are matched?). Ultimately most users of the API just chose a flat number and lived with the results, good or bad.

In 0.10, you no longer need to provide a batch size! We'll automatically evaluate the state of the World and task pools and select a batch size using a heuristic to ensure sufficient parallelism, without incurring too much overhead. For more complete details on how this is computed, see the documentation of [`Query::par_iter`]. In the future, we may further tune the backing heuristics to try to get the default to be more flexible.

```rust
// 0.9
const QUERY_BATCH_SIZE: usize = 32;

query.par_for_each(QUERY_BATCH_SIZE, |mut component| {
   // ...
});

// 0.10
query.par_iter().for_each(|mut component| {
   // ...
});
```

## `Ref<T>` Queries

<div class="release-feature-authors">authors: @Guvante, @JoJoJet</div>

Since Bevy 0.1, `Mut<T>` has been used to enable change detection (along with related types like `ResMut<T>`). It's a simple wrapper type that provides mutable access to a component alongside its change tick metadata, automatically marking a change when the value is mutated.

In **Bevy 0.10**, the change detection family has grown with `Ref<T>`, the immutable variant of `Mut<T>`. Like its mutable sibling, it allows you to react to changes made outside of the current system.

```rust
use bevy::prelude::*;

fn inspect_changes_system<T: Component + Debug>(q: Query<Ref<T>>) {
    // Iterate over each component of type `T` and log its changed status.
    for val in &q {
        if val.is_changed() {
            println!("Value `{val:?}` was last changed at tick {}.", val.last_changed());
        } else {
            println!("Value `{val:?}` is unchanged.");
        }
    }
}
```

We are also deprecating `ChangeTrackers<T>`, which is the old way of inspecting a component's change ticks. This type will be removed in the next version of Bevy.

## What's Next?

* **Some Work In Progress Feature**: Description here.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](Bevy ) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

A huge thanks to the **X contributors** that made this release (and associated docs) possible! In random order:

* @Foo

## Full Change Log

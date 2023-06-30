+++
title = "Bevy 0.11"
date = 2023-07-07
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.11** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.11**, check out our [0.10 to 0.11 Migration Guide](/learn/migration-guides/0.10-0.11/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Feature**: description

## Deref Derive Attribute

<div class="release-feature-authors">authors: @MrGVSV</div>

Bevy code tends to make heavy use of the [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern,
which is why we have dedicated derives for [`Deref`](https://docs.rs/bevy/latest/bevy/prelude/derive.Deref.html) and [`DerefMut`](https://docs.rs/bevy/latest/bevy/prelude/derive.DerefMut.html).

This previously only worked for structs with a single field:

```rust
#[derive(Resource, Deref, DerefMut)]
struct Score(i32);
```

For 0.11, we've improved these derives by adding the `#[deref]` attribute, which allows them to be used on structs with multiple fields.
This makes working with generic newtypes much easier:

```rust
#[derive(Component, Deref, DerefMut)]
struct Health<T: Character> {
    #[deref] // <- use the `health` field as the `Deref` and `DerefMut` target
    health: u16,
    _character_type: PhantomData<T>,
}
```

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author

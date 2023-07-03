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

## Better Proxies

<div class="release-feature-authors">authors: @MrGVSV</div>

Bevy's reflection API has a handful of structs which are collectively known as "dynamic" types.
These include [`DynamicStruct`], [`DynamicTuple`], and more, and they are used to dynamically construct types
of any shape or form at runtime.
These types are also used to create are commonly referred to as "proxies", which are dynamic types
that are used to represent an actual concrete type.

These proxies are what powers the [`Reflect::clone_value`] method, which generates these proxies under the hood
in order to construct a runtime clone of the data.

Unfortunately, this results in a few [subtle footguns] that could catch users by surprise,
such as the hashes of proxies differing from the hashes of the concrete type they represent,
proxies not being considered equivalent to their concrete counterparts, and more.

While this release does not necessarily fix these issues, it does establish a solid foundation for fixing them in the future.
The way it does this is by changing how a proxy is defined.

Before 0.11, a proxy was only defined by cloning the concrete type's [`Reflect::type_name`] string
and returning it as its own `Reflect::type_name`.

Now in 0.11, a proxy is defined by copying a reference to the static [`TypeInfo`] of the concrete type.
This will allow us to access more of the concrete type's type information dynamically, without requiring the `TypeRegistry`.
In a [future release], we will make use of this to store hashing and comparison strategies in the `TypeInfo` directly
in order to mitigate the proxy issues mentioned above.

[`DynamicStruct`]: https://docs.rs/bevy/0.11.0/bevy/reflect/struct.DynamicStruct.html
[`DynamicTuple`]: https://docs.rs/bevy/0.11.0/bevy/reflect/struct.DynamicTuple.html
[`Reflect::clone_value`]: https://docs.rs/bevy/0.11.0/bevy/reflect/trait.Reflect.html#tymethod.clone_value
[subtle footguns]: https://github.com/bevyengine/bevy/issues/6601
[`Reflect::type_name`]: https://docs.rs/bevy/0.11.0/bevy/reflect/trait.Reflect.html#tymethod.type_name
[`TypeInfo`]: https://docs.rs/bevy/0.11.0/bevy/reflect/enum.TypeInfo.html
[future release]: https://github.com/bevyengine/bevy/pull/8695

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author

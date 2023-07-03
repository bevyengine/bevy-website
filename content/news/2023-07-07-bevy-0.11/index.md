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

## `FromReflect` Ergonomics

<div class="release-feature-authors">authors: @MrGVSV</div>

Bevy's [reflection API] commonly passes around data using type-erased `dyn Reflect` trait objects.
This can usually be downcast back to its concrete type using `<dyn Reflect>::downcast_ref::<T>`;
however, this doesn't work if the underlying data has been converted to a "dynamic" representation
(e.g. `DynamicStruct` for struct types, `DynamicList` for list types, etc.).

```rust
let data: Vec<i32> = vec![1, 2, 3];

let reflect: &dyn Reflect = &data;
let cloned: Box<dyn Reflect> = reflect.clone_value();

// `reflect` really is a `Vec<i32>`
assert!(reflect.is::<Vec<i32>>());
assert!(reflect.represents::<Vec<i32>>());

// `cloned` is a `DynamicList`, but represents a `Vec<i32>`
assert!(cloned.is::<DynamicList>());
assert!(cloned.represents::<Vec<i32>>());

// `cloned` is equivalent to the original `reflect`, despite not being a `Vec<i32>`
assert!(cloned.reflect_partial_eq(reflect).unwrap_or_default());
```

To account for this, the [`FromReflect`] trait can be used to convert any `dyn Reflect` trait object
back into its concrete type— whether it is actually that type or a dynamic representation of it.
And it can even be called dynamically using the [`ReflectFromReflect`] type data.

Before 0.11, users had to be manually derive `FromReflect` for every type that needed it,
as well as manually register the `ReflectFromReflect` type data.
This made it cumbersome to use and also meant that it was often forgotten about,
resulting in reflection conversions difficulties for users downstream.

Now in 0.11, `FromReflect` is automatically derived and `ReflectFromReflect` is automatically registered for all types that derive `Reflect`.
This means most types will be `FromReflect`-capable by default,
thus reducing boilerplate and empowering logic centered around `FromReflect`.

Users can still opt out of this behavior by adding the [`#[reflect(from_reflect = false)]`][from_reflect = false] attribute to their type.

```rust
#[derive(Reflect)]
struct Foo;

#[derive(Reflect)]
#[reflect(from_reflect = false)]
struct Bar;

fn test<T: FromReflect>(value: T) {}

test(Foo); // <-- OK!
test(Bar); // <-- ERROR! `Bar` does not implement trait `FromReflect`
```

[reflection API]: https://docs.rs/bevy_reflect/latest/bevy_reflect/index.html
[`FromReflect`]: https://docs.rs/bevy_reflect/latest/bevy_reflect/trait.FromReflect.html
[`ReflectFromReflect`]: https://docs.rs/bevy_reflect/latest/bevy_reflect/struct.ReflectFromReflect.html
[from_reflect = false]: https://docs.rs/bevy_reflect/latest/bevy_reflect/derive.Reflect.html#reflectfrom_reflect--false

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author

+++
title = "Compiling Less Code"
insert_anchor_links = "right"
[extra]
weight = 2
+++

As a game engine, Bevy offers a wide variety of functionality out-of-the-box.
It's unlikely that your project will take advantage of all of the features at the same time though.
Keeping every feature enabled will result in increased compilation times and a bloated project binary size.
However, each project might require a different combination of features.

To get the best of both worlds, we need to allow users to customize which parts of Bevy are enabled.
This is called **conditional compilation**.
In Rust, this is done through the use of [feature flags](https://doc.rust-lang.org/cargo/reference/features.html).

## Bevy's Default Features

Rust allows us to designate "default" features for Bevy, causing them to be automatically enabled for users who add Bevy to their project.
When you call `cargo add bevy`, this creates the following entry in your `Cargo.toml`, which tracks your project's dependencies:

```rust
bevy = "0.18"
```

This is equivalent to:

```rust
bevy = { version = "0.18", default-features = true }

```

Bevy's default features are designed to provide a beginner-friendly experience that works out of the box.
This includes both starting new projects and exploring pre-written examples.
As a result, most of Bevy's functionality is enabled and the resulting setup is not particularly lightweight.

Default features do not create an opt-in vs opt-out distinction.
Features in Rust can only be enabled by various sources, never disabled directly.

To turn Bevy's default features off, you would need to configure your `Cargo.toml` like so:

```toml
bevy = { version = "0.18", default-features = false }
```

In almost every published Bevy project, you should set `default-features = false`, and instead opt in to only the features you need.
For most users, feature collections are the right level of granularity.

## Feature Collections

Bevy is a large project, with lots of different functionality and users who each have their own unique needs.
As a result, it has *many* feature flags, and these feature flags often change quickly across versions.

To make this easier to manage, Bevy offers "feature collections", which are feature flags that simply enable other features.
You can see the full list of features by reading `bevy`'s [Cargo.toml](https://github.com/bevyengine/bevy/blob/main/Cargo.toml).
The most important ones are:

- `2d`: For 2D games.
- `3d`: For 3D games.
- `ui`: Enables Bevy's built-in UI solutions.
- `audio`: Enables Bevy's audio solution.
- `dev`: Various tools for project development.
  - This flag should not be enabled for your released builds.

Disabling the default features and using these feature collections quickly removes large portions of the engine
that your project will not need, all without spending a great deal of time digging into and updating more granular feature flags.

{% callout(type="info") %}

Bevy's ECS makes dead code analysis surprisingly challenging.
Systems are registered regardless of whether or not any entities with those components ever exist.
All types referenced and methods used are then marked as alive,
and the compiler cannot strip them from the final binary.

As a result, disabling plugins or removing systems from schedules will not result in a binary size reduction.

{% end %}

## More Selective Feature Use

Users who are particularly interested in optimizing binary size, compile times, or even runtime performance may find it useful to try to shrink the set of feature flags they're using even further.
This process can be quite time-consuming and frustrating (especially when updating Bevy versions), and we recommend only doing so if it's absolutely needed.
You *will* have to dig into the internals regularly, and track the flow of features across multiple crates.

Bevy's feature flags follow a fairly complex flow:

1. A Bevy sub-crate (e.g. `bevy_ecs`) defines a feature.
2. This crate is wrapped by `bevy_internal` (a compile-time optimizing implementation detail), which exposes some or all of their feature flags as features.
3. These feature flags are mirrored in `bevy`.
4. Feature flags are assembled into feature collections, including `default`.

There are two viable approaches to constructing a minimal feature set:

1. Set `default-features = false` when beginning a new project, and only add features when functionality that you want to use is missing or broken.
2. Start with a fairly permissive set of features, construct a working project, and then pare them down. Replace feature collections with their corresponding set of features and then attempt to delete features one-at-a-time to see what breaks.

Both of these can work well; approach 1 prioritizes faster compilation times throughout development, while approach 2 will let you get off the ground more easily.
Again, this process is *fully* optional: you should not feel like you *have* to do this unless you have real data that it's important for your particular project.

{% callout(type="info") %}

Features in Rust are *additive*.
This means that if any crate in your tree enables a feature,
it will be enabled for all users of the crate.

Be warned that ecosystem crates, or `bevy`'s own subcrates, may enable various features of subcrates.
Simply not enabling them yourself is insufficient.

You can check which features are enabled using `cargo tree -f {p} {f}`.

If one of your dependencies is inadvertently enabling a feature that you don't need,
please open an issue or PR!
In many cases, this is a simple mistake, and can be fixed by disabling `default-features`
or by gating some of their functionality behind feature flags of their own.

{% end %}

## Looking For Duplicate Crates

Unlike in other languages, Rust projects can have multiple different versions of the same dependency.
This makes it much easier to have complex dependency trees, but can pointlessly bloat both binary size and compilation time.

The best tool to explore your dependencies in Rust is [cargo tree](https://doc.rust-lang.org/cargo/commands/cargo-tree.html).
You can search for duplicate dependencies using `cargo tree -d`.

Resolving duplicate dependencies can be quite challenging. Politely asking the upstream maintainers to bump their dependency is often effective, but coordinating releases can be tricky!

When resolving crate versions, all crates with the same minor version are treated as interchangeable,
as they are (at least theoretically) [semver compatible].
[Version ranges] can be used in upstream crates, allowing for the same behavior across major versions.
However, this can be quite challenging to test in an automatic fashion (especially if this is done for multiple crates).

You can change which version of a crate your project relies on globally using `cargo`'s patch functionality.
As described in the Cargo Book's chapter on [Overriding Dependencies], you can set the version used for a given dependency like so:

```toml
[patch.crates-io]
rand = "0.8.5"
```

This process is not without its problems, though. Compilation errors or subtle bugs are very possible,
as your dependencies have not been built or tested with a different major version of that crate in mind.

If this is critically important to your project, you may want to consider [maintaining your own forks](#maintaining-your-own-forks),
and patching the crates you rely on to align the versions needed, and fixing any bugs that arise due to changing the dependencies.

[semver compatible]: https://doc.rust-lang.org/cargo/reference/semver.html
[Overriding Dependencies]: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html
[Version ranges]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#comparison-requirements

## Maintaining Your Own Forks

In an effort to compile less code or fix a few critical bugs, it can be tempting to maintain your own forks of Bevy or other dependencies.

This can be extremely effective: simply deleting unused code is a great way to not compile it!
However, it comes at a very high cost: setting up, testing, and maintaining forks takes a lot of time and expertise.
These costs scale substantially the more you diverge from upstream, especially if there are new features and bug fixes in Bevy that you want to port to your fork.

As a result, we recommend:

1. Only forking during the final stages of a project's lifecycle, once you are no longer updating Bevy versions.
2. Only maintaining forks for particularly heavy dependencies, or for critical bug fixes.
3. Minimizing divergence from upstream unless there is a good reason to do so.
4. Committing your `Cargo.lock` file, and carefully testing any updates or upstream changes.

You can use local forks of a project with the use of cargo patches:

```toml
[patch.crates-io]
bevy = { path = "../bevy" }
```

For large or sophisticated teams, you may prefer to use an [alternate crate registry],
allowing you to coordinate the versions of crates you are using across multiple projects.

[alternate crate registry]: https://doc.rust-lang.org/cargo/reference/registries.html


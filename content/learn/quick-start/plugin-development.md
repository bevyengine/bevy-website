+++
title = "Building Bevy's Ecosystem"
aliases = ["learn/book/plugin-development"]
[extra]
weight = 4
+++

Bevy has a plug-and-play architecture, where you can easily add plugins for new features or use your own plugins instead of the built-in ones. You can also create third-party plugins that others may use in their applications.

With that in mind, this page provides some basic info that can be useful when authoring third-party plugins.

## Naming

You are free to use a `bevy_xxx` name for your plugin, but please be reasonable. If you are about to claim a generic name like `bevy_animation`, `bevy_color`, or `bevy_editor`, please ask first. The rationale is explained [here](https://github.com/bevyengine/bevy/discussions/1202#discussioncomment-258907).

## Licensing

Bevy is dual licensed under [MIT or Apache 2.0](https://www.rust-lang.org/policies/licenses), at your option. Most other Rust projects (including Rust itself) also use this dual-license approach. MIT-only is very popular, and you might be tempted to just use that (Bevy also used to be MIT-only), but there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both licenses. We highly recommend using the dual MIT / Apache 2.0 license for your Bevy Plugins and crates:

- Including the Apache 2.0 license option significantly reduces the difficulty and boilerplate of proper license compliance in published games because you only need to include one copy of the Apache 2.0 license.
- Provides maximum compatibility with Bevy and Rust, making it easier to upstream your changes.

## Rust API and Cargo SemVer Guidelines

While they are only guidelines, it can be useful for you to look at and consider the [Rust API guidelines](https://rust-lang.github.io/api-guidelines/) and [Cargo SemVer compatibility conventions](https://doc.rust-lang.org/cargo/reference/semver.html) for recommendations on how to write your API and what to consider a breaking or compatible change.

## Generic Plugin Types

It can be useful to allow your users to supply generic types to your plugins. It can enable them to write custom logic for components to be used; give your plugin a marker component to note an entity it should do some logic to; add events that your plugin should listen for; or a resource your plugin should use (which is useful if you want to apply your plugin to multiple resources of the same type via type aliases.)

You can define a generic plugin like so:

{{file_code_block(file="quick-start/generic_plugin.rs", anchor="generic_plugin")}}

A prime example of generic plugins in use is the [Bevy Cellular Automaton Plugin](https://github.com/ManevilleF/bevy_life).

## Small Crate Size

To avoid long build times in your plugin and in projects using it, you should aim for a small crate size:

- Only include the Bevy features you absolutely need.

  Features are additive — Bevy features enabled in your plugin cannot be disabled by someone using your plugin.

  You should add `default-features = false` to the Bevy dependency in your `Cargo.toml` and manually specify the features you need.

  You can find a list of Bevy's features [here](https://github.com/bevyengine/bevy/blob/main/docs/cargo_features.md).

- Avoid large new dependencies.
- Make sure your dependencies are not duplicated, using [`cargo tree`](https://doc.rust-lang.org/cargo/commands/cargo-tree.html) or [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny).
- Put optional functionality and dependencies behind a [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html).

## `no_std` Compatibility

Since the release of Bevy 0.16, it is now possible to use Bevy in a [`no_std`](https://doc.rust-lang.org/stable/reference/names/preludes.html#the-no_std-attribute) context.
If you don't want to or can't support `no_std` in your plugin, you'll likely want to enable the `std` feature in Bevy to regain some functionality that was previously included by default.

```toml
bevy = { version = "0.16", default-features = false, features = ["std"] }
```

If you _do_ want to pursue `no_std` support, please refer to the `no_std` example in `examples/no_std/library`.
This example includes recommended feature flags and some helpful tips for working with `no_std` Bevy.

## Tests and CI

Tests are always good! For CI, you can check out [this example](https://github.com/actions-rs/meta/blob/master/recipes/quickstart.md) for a quick start using GitHub Actions. As Bevy has additional Linux dependencies, you should install them before building your project ([here is how Bevy is doing it](https://github.com/bevyengine/bevy/blob/9788b386c7846c99978ab5c1a33698ec5a471d84/.github/workflows/ci.yml#L40)). Even if you don't have many (or any) tests, setting up CI will compile check your plugin and ensure a basic level of quality.

## Indicate Compatible Versions

Indicating which version of your plugin works with which version of Bevy can be helpful for your users. Some of your users may be using an older version of Bevy for any number of reasons. You can help them find which version of your plugin they should use. This can be shown as a simple table in your README with each version of Bevy and the corresponding compatible version of your plugin.

```markdown
| bevy  | bevy_awesome_plugin |
|-------|---------------------|
| 0.13  | 0.3                 |
| 0.12  | 0.1                 |
```

## Main Branch Tracking

Bevy is evolving very fast. There are often new features on the main branch but have not yet been released. Your plugin might depend on Bevy Main or the latest release. You can also do both on different branches (e.g., have a `bevy_main` branch).

If you intend to track Bevy's main branch, you can specify the latest commit you support in your `Cargo.toml` file:

```toml
bevy = { version = "0.5", git = "https://github.com/bevyengine/bevy", rev = "9788b386c7846c99978ab5c1a33698ec5a471d84", default-features = false }
```

You can specify the dependency [both as a version and with git](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#multiple-locations). The version will be used if the dependency is pulled from [crates.io](https://crates.io/). Otherwise, the git dependency will be used.

You can use one of these badges to communicate to your users how closely you intend to track Bevy's main branch.

[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)

```markdown
[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)
```

[![Following Bevy's main branch](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)

```markdown
[![Following Bevy's main branch](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)
```

## Documentation and Examples

Documentation and examples are very useful for a crate.

In the case of a Bevy plugin, a few screenshots or movies / animated GIFs from your examples can really help to understand what your plugin is capable of.

Additionally, it can be helpful to list:

- SystemSets available from your plugin, and their execution order if that's important.
- Components available from your plugin.

## Publishing Your Plugin

There are some [extra fields](https://doc.rust-lang.org/cargo/reference/manifest.html) that you can add to your `Cargo.toml` manifest in the `[package]` section:

- [`description`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-description-field) — A description of the plugin
- [`repository`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field) — URL of the plugin source repository
- [`license`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields) — The plugin license
- [`keywords`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-keywords-field) — Keywords for the plugin. `"bevy"` at least is a good idea here
- [`categories`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-categories-field) — Categories of the plugin. See [the full list on crates.io](https://crates.io/categories).
- [`exclude`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields) — Files to exclude from the released package. Excluding the `assets` folder that you may have is a good idea, as well as any large files that are not needed by the plugin.

Once a crate is published to [crates.io](https://crates.io), there are two badges that you can add to your `README.md` for easy links:

[![crates.io](https://img.shields.io/crates/v/bevy)](https://crates.io/crates/bevy)

```markdown
[![crates.io](https://img.shields.io/crates/v/bevy_awesome_plugin)](https://crates.io/crates/bevy_awesome_plugin)
```

[![docs.rs](https://docs.rs/bevy/badge.svg)](https://docs.rs/bevy)

```markdown
[![docs.rs](https://docs.rs/bevy_awesome_plugin/badge.svg)](https://docs.rs/bevy_awesome_plugin)
```

## Promotion

You can promote your plugin in Bevy's [communities](/community):

- Add it as an [Asset on the official website](https://github.com/bevyengine/bevy-assets).
- Announce it on [Discord](https://discord.gg/bevy), in the `#crates` channel.
- Announce it on [Reddit](https://reddit.com/r/bevy).

+++
title = "Creating libraries for Bevy"
insert_anchor_links = "right"
[extra]
weight = 99
+++

One of Bevy's strengths is its large collection of 3rd party **ecosystem crates**.
Many of these have been refactored out of existing, functioning projects.
Open sourcing parts of your project like this is a nice way to give back to the community,
but can also pay dividends as contributors help you refine and polish the crate that you now depend on.

This chapter covers some best practices for creating open source libraries with Bevy.
Most of these sections will *also* apply if you're splitting your work apart for reuse between projects!

## Setting up your crate

### Naming your crate

Names are famously one of the hardest problems in computer science,
and crates are no exception to this.

When considering a name for your crate, we recommend that you:

- **Always use `_` rather than `-`.** This keeps the crate name consistent with how users import it.
- **Pick a name which communicates what your crate does** in a somewhat unique way.
- **Avoid confusion with potential official Bevy crates.** `crates.io` does not have namespaces currently, and `bevy_color` and `bevy_colour` are easy to confuse.
- **Consider choosing a "branding" name** for your family of crates (e.g. `leafwing`). This can unify your projects without cluttering the main crate namespace.

### Licensing

In order for others to use your work, it needs a license.
Your license determines how others can use, modify, and distribute your work.
Work without a license is not legally usable by others.

The Bevy ecosystem has converged on **dual-licensing under MIT and Apache 2.0**,
matching Bevy itself and the vast majority of its ecosystem crates.
This is the standard choice for Rust projects more broadly, and we recommend it unless you have a specific reason to do otherwise.
If you would eventually like your work to be part of Bevy itself, this is the licensing setup needed.

To apply this to your crate:

1. Add a `LICENSE-MIT` and `LICENSE-APACHE` file to the root of your repository. You can copy these from the [Bevy repository](https://github.com/bevyengine/bevy).
2. Set the `license` field in your `Cargo.toml`:

```toml
[package]
license = "MIT OR Apache-2.0"
```

If you're new to open source licensing, the key thing to understand is that permissive licenses like MIT and Apache 2.0 allow anyone to use your code for any purpose, including in commercial projects, as long as they include the license text.
Apache 2.0 additionally provides an explicit patent grant, which is why the dual-license approach is preferred: users get to choose whichever license suits them better.

A copyleft license (like the GPL) requires people who modify your crate to share their changes.
This will likely reduce the willingness of companies using Bevy to rely on your crate.
Depending on your perspective, this may be a feature or a bug.
You should almost always avoid using unusual licenses, or those with atypical terms,
as this will seriously hinder the adoption of your work and the contributions made back to your crate.

### Depending on Bevy

When making a crate that relies on Bevy, there are two strategies:

- **Rely on `bevy` directly.** Simple to set up and update, and easy to patch if the user wants to point to their own fork or `bevy/main`.
- **Rely on Bevy subcrates**, like `bevy_ecs` and `bevy_input`. Works for projects that don't rely on `bevy` itself, and enables faster compilation: your crate can compile before `bevy` itself finishes.

We recommend subcrates, primarily for the compile-time benefits.

Regardless of your choice, you should *always* specify `default-features = false` in your `Cargo.toml`.
This ensures that your crate does not pull in unneeded code.
Remember: features can only be enabled, not disabled in Rust.

## Polishing your crate

### Idiomatic Bevy and Rust

Users of your crate will expect it to feel like a natural extension of Bevy.
A few conventions go a long way:

- **Expose a plugin.** The standard entry point for a Bevy library is a [`Plugin`](https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html). Even if your crate is simple, wrapping your setup logic in a plugin gives users a consistent, predictable way to integrate it: `app.add_plugins(YourPlugin)`.
- **Use the ECS where it makes sense.** Bevy users expect their crates to use components, resources, events and messages. Structuring your crate to use them makes it easier to extend, modify and inspect.
- **Don't panic.** Library code should almost never crash. Program defensively, and return `Result` when something goes wrong. Log an error or warning if there's nowhere to return. Reserve panics exclusively for upholding soundness invariants. Read more about [handling errors here](../control-flow/handling-errors.md).
- **Follow Rust API guidelines.** The [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) are a great reference for naming, documentation, and API design.
- **Use feature flags to make expensive or niche functionality opt-in.** If your crate has optional integrations (e.g. serialization, debug tooling, support for specific Bevy subsystems), gate them behind [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) rather than pulling everything in by default. This keeps compile times down and avoids forcing unwanted dependencies on your users. A few guidelines:
  - Name features after what they enable (e.g. `serialize`, `bevy_ui`), not what they depend on.
  - Keep your default features minimal.
  - Document your features in your `Cargo.toml` using comments above each feature.
- **Consider `no_std` support.** If your crate is primarily logic — math, data structures, ECS patterns — it may not need the standard library at all. This makes your crate usable in more environments, including embedded platforms and WebAssembly without a full runtime.
  - This is easiest to set up at the start of a project; retrofitting it later is harder but not impossible.
  - Add `#![no_std]` to your `lib.rs`, with a `std` feature flag to enable anything that requires it
  - In the majority of cases, you can just use `core` or `alloc` directly instead
  - Confirm that your dependencies work without `std` by compiling to a target that does not have `std` (such as `wasm32v1-none`).
  - For more advice on this topic, check out Bevy's own [`no_std` example], which will walk you through best practices and setup.

[`no_std` example]: https://github.com/bevyengine/bevy/tree/latest/examples/no_std/library

### Minimizing dependencies

You should try to keep your dependency tree lean.
Every dependency you add is a dependency your users inherit, and one that you need to regularly update.

Turn a critical eye to each included dependency: they can often be replaced by using a slightly different approach,
or by vendoring only the small bit of the library you need.

If you need to use a dependency, prefer to use the same dependencies as Bevy itself, and look for well-maintained, widely-used crates otherwise.

Be *particularly* careful about adding dependencies that create a strong ecosystem lock-in effect,
such as `tokio`.

### Documentation

Good documentation is essential for both initial adoption and sustained use.
As a bonus, it makes your crate easier to maintain!

At minimum, your crate should have:

- **A README** that explains what the crate does and shows a minimal usage example. This is the first thing people see on crates.io and GitHub.
  - If your crate is visual (rendering, UI, animation), screenshots or GIFs in the README make a huge difference. A short recording of an example running can communicate what your crate does faster than any paragraph.
- **A crate-level doc comment** (`//!` at the top of `lib.rs`) that provides an overview and links to the most important types.

  - You can avoid maintaining both a README and a crate-level doc comment by using `#![doc = include_str!("../README.md")]` at the top of `lib.rs`. This pulls your README in as the crate's documentation on [docs.rs](https://docs.rs), keeping everything in sync.
- **Doc comments on all public items.** `cargo doc --open` should produce useful, navigable documentation.
  - By convention, the first line of each doc comment in Rust describes *what* the type is or what the module does.
  - Then, after a blank line, detailed information is given. This might cover usage, return values, related methods or cases where the code panics.
  - Enable the `missing_docs` lint to detect problems automatically

Rust's doc comments support in-line examples (doc-tests) that are compiled and tested by `cargo test`. These help keep your documentation accurate as your code evolves.

You should also consider writing module-level doc comments (`//!` at the top of each module) that explain the purpose of that module and how its contents relate to each other. These are especially valuable in larger crates where users navigate the docs by module.

### Examples

Users first learn to use your crate by reading and running examples, so invest in making yours clear and self-contained.

- **Place examples in an `examples/` directory** at the root of your crate.
- **Make each example a complete, runnable program.** A user should be able to `cargo run --example <name>` and see something working.
- **Keep examples focused.** One concept per example is better than a single monolithic demo.
- **Add comments** explaining what the example demonstrates and any non-obvious setup.

```toml
# Cargo.toml
[[example]]
name = "basic_usage"
path = "examples/basic_usage.rs"
```

While you should always rely on Bevy or its subcrates using `default-features=false`,
it's common to need access to more features to create an interactive example.
You can depend on `bevy` as a `dev-dependency` using whatever features you need,
and get the best of both worlds.

### Automated quality control

Set up CI early. A basic GitHub Actions workflow can catch issues before they reach your users and makes accepting contributions much easier.

Here's a good, simple starting point:

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: cargo test --all-features
      - run: cargo doc --all-features --no-deps
```

This checks formatting, runs Clippy lints, runs your tests, and verifies your documentation compiles — all in one workflow.

We have a few additional setup suggestions:

- **Test against the minimum supported Rust version (MSRV)** if you declare one.
- **Run your examples in CI** to make sure they don't bit-rot. `cargo run --example <name>` works for headless examples; visual examples may need a headless setup, dedicated hardware or can simply be compiled without running.
- **Deny warnings in CI, not in your crate.** Use `-D warnings` as a Clippy flag in CI rather than `#![deny(warnings)]` in your code, which can break builds for your users when new Rust versions add lints.
- **Disable the `type_complexity` lint.** Bevy code naturally produces complex types, especially in queries and system parameters. Allow this lint in your `Cargo.toml` to avoid noisy warnings that don't reflect real problems in Bevy-style code:

  ```toml
  [lints.clippy]
  type_complexity = "allow"
  ```

Take a look at our chapter on [Testing](/learn/book/development-practices/testing):
this advice is particularly useful for library authors.

### Configuring Github repo settings

There are a few things we recommend configuring for every new public open source project:

- **Protect your main branch.** Require PRs to pass CI before merging. This keeps contributors from accidentally breaking things, and lets you review changes before they land. On GitHub, configure this under Settings → Branches → Branch protection rules.
- **Use squash-and-merge.** Squashing PR commits into a single commit keeps your main branch history clean and readable. Each merge becomes one self-contained entry in `git log`, which makes changelogs, bisecting, and reverts much simpler. You can set this as the default (or only) merge strategy in your GitHub repository settings.

## Sharing your crate

### Publishing

Before publishing to [crates.io](https://crates.io), make sure your `Cargo.toml` has the metadata that helps users find and evaluate your crate:

```toml
[package]
name = "bevy_your_crate"
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
description = "A short, one-line description of what your crate does"
repository = "https://github.com/you/bevy_your_crate"
readme = "README.md"
keywords = ["bevy", "gamedev"]
categories = ["game-development"]
```

The `bevy_` prefix is a strong convention in the ecosystem — it makes your crate discoverable and immediately signals that it's a Bevy library.

When you're ready, publish with:

```sh
cargo publish
```

Use [semantic versioning](https://semver.org/) for your releases. For pre-1.0 crates (which most Bevy ecosystem crates are), a bump to the minor version (`0.1.0` → `0.2.0`) signals breaking changes, while a patch bump (`0.1.0` → `0.1.1`) signals backwards-compatible fixes.

### Promoting your crate

If you went to the effort of making and publishing a crate, you probably want people to use it.
Getting your crate in front of users:

- **Write a short blog post or announcement.** Explaining *why* you built the crate and what problems it solves gives people a reason to try it.
- **Submit to [Bevy Assets](https://bevy.org/assets/).** This is the official directory of Bevy ecosystem crates and the first place many users look. Submit your crate by opening a PR on the [bevy-assets repository](https://github.com/bevyengine/bevy-assets).
- **Post in the [Bevy Discord](https://discord.gg/bevy).** The `#showcase` channel is an active, welcoming place to share new releases. The community is generous with feedback.
- **Share on social media.** Posts on Reddit ([r/bevy](https://reddit.com/r/bevy), [r/rust](https://reddit.com/r/rust), [r/rust_gamedev](https://reddit.com/r/rust_gamedev)), Mastodon, and Bluesky with the `#BevyEngine` tag can reach a wide audience.

Make sure that your README, crate docs and your posts on social media clearly explain what your work is and why someone might want to use it.
Don't underestimate the value of a pretty screenshot or GIF — polish is a strong indicator of a maintainer who's put the time in to make something worth using.

## Maintaining your crate

### Keeping up with Bevy versions

The major version of Bevy that your crate uses must match the Bevy version that your users are on.
As a result, users expect ecosystem crates to track Bevy releases actively.

This can be a fair bit of work, but there are a few practices that help:

- **Clearly document which Bevy version your crate supports**, both in the README and in your crate-level docs. A compatibility table is a common pattern:

  | `your_bevy_crate` | Bevy |
  | ----------------- | ---- |
  | 0.3-0.4           | 0.15 |
  | 0.2               | 0.14 |
  | 0.1               | 0.13 |

- **Release a new version of your crate for each Bevy release.** Users will need a compatible version, and a prompt update signals that your crate is actively maintained.
- **Separate breaking changes from version bumps.** It's much easier for users if you separate out breaking changes to your own API from Bevy upgrades, allowing them to fix one problem at a time.
- **Use branches or tags** to maintain older versions if you need to backport fixes.
- **Prepare your next release during Bevy's release candidate phase.** This phase exists to give ecosystem crate authors time to update and catch problems early: please use it!
- **Work with your community to update.** Active crates often get PRs from users eager to update versions. Work with them to polish the migration and spread the load.
- **Use the [Bevy migration guides](https://bevy.org/learn/migration-guides/).** These are the primary resource for updating your code.

### Managing contributions

If your crate gains traction, people will want to contribute.
You want to empower users to scratch their own itch, not stand in their way.

There are a few simple project management practices that go a long way:

- **Add a CONTRIBUTING.md** that explains how to build the project, run tests, and what kind of contributions you're looking for. Even a short one removes a real barrier.
- **Use issue labels** to help contributors find good starting points and search through the backlog. Modify the categories freely to meet your needs.
- **Add co-maintainers as your crate grows.** Sole maintainership is a bottleneck and a bus factor. If a regular contributor has earned your trust, give them merge rights on GitHub and add them as an owner on crates.io with `cargo owner --add <username>`. This ensures your crate can continue to ship updates even when life gets in the way.

But open source is as much a social process as it is a technical strategy.
Here are our most important tips on being a good open source maintainer:

- **Clearly define what your crate wants to be.** A clear vision goes a long way to reducing conflict and fostering productive collaboration. Don't be afraid to say what your crate is *not*.
- **Be responsive and kind.** Open source contributions are a gift of someone's time. A prompt, friendly response is always welcome, even if it's "thanks, but this doesn't fit the project's direction".
- **Set expectations about your availability.** If you maintain the crate in your spare time, say so. Contributors appreciate honesty about response times, and are surprisingly understanding if you're open about your limitations.
- **Review PRs thoughtfully.** Explain *why* you're requesting changes, not just *what* to change. Good reviews teach contributors your crate's conventions and make future PRs better.
- **Encourage reviews from the community.** Your users and contributors have useful perspectives on potential changes. You should welcome their interest and expertise. Bevy does this to great effect!
- **Ask for help when you need it.** Maintaining an open source crate can be overwhelming, especially alone. The Bevy community is friendly and experienced — don't hesitate to ask for advice, reviews, or help in the [Bevy Discord](https://discord.gg/bevy). If you're struggling with maintainer burnout, it's okay to say so and ask for volunteers to step up.

### Keeping a changelog

As your crate evolves, help your users keep up.

- **Keep a CHANGELOG.md** following the [Keep a Changelog](https://keepachangelog.com/) format. Organize changes under Added, Changed, Deprecated, Removed, and Fixed headings for each release. This gives users a quick way to see what's new and what might affect them.
- **Call out breaking changes prominently.** When a release includes breaking changes, list them clearly at the top of the release notes with guidance on how to migrate.
- **For larger breaking changes, write a migration guide.** This doesn't have to be elaborate — a few lines showing the before and after are often enough:

  ```rust
  // Before (0.2)
  app.add_plugins(YourPlugin { speed: 5.0 });

  // After (0.3)
  app.add_plugins(YourPlugin::default())
     .insert_resource(YourSettings { speed: 5.0 });
  ```

- **Tag releases in git** and use GitHub Releases (or your host's equivalent) to publish release notes alongside the tag. This gives users a natural place to find what changed.

### Offering support

As your user base grows, people will ask for help using your crate.
You should not feel compelled to do so (you're almost certainly not getting paid for this),
but there are a few practices that can help:

- **Update the docs in response to user questions.** If a user has a question, the docs were probably unclear. Update them, so you don't have to do this again.
- **Create an #ecosystem-crates thread on the Bevy Discord.** Ask the mods for this in #community, and collect a community that can help each other in one place.
- **Enable Github Discussions.** This can be a nice search-engine-indexed place for people to ask questions and get help.
- **Don't feel like you need to make everyone happy.** No crate is right for everyone: it's not your job to change your crate to meet every use case, or teach every new user how to solve their exact problem. It's okay to say no, or not respond sometimes.

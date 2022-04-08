+++
title = "Getting Started"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

This section will help you get started on your Bevy journey as quickly as possible. It will walk you through setting up your development environment and writing a simple Bevy app.

## Quick Start

If you want to dive in immediately and you already have a working Rust setup, feel free to follow this "quick start" guide. Otherwise, move on to the next page.

Note: the "fast compiles" setup is on the next page, so you might want to read that section first.

### Try the Examples

1. Clone the [Bevy repo](https://github.com/bevyengine/bevy):
    ```sh
    git clone https://github.com/bevyengine/bevy
    ```
2. Navigate to the new "bevy" folder
    ```sh
    cd bevy
    ```
3. Switch to the correct Bevy version (as the default is the git main development branch)
    ```sh
    # use the latest Bevy release
    git checkout latest
    # or a specific version
    git checkout v0.6.0
    ```
4. Try the examples in the [examples folder](https://github.com/bevyengine/bevy/tree/latest/examples#examples)
    ```sh
    cargo run --example breakout
    ```

### Add Bevy as a Dependency

Bevy is [available as a library on crates.io](https://crates.io/crates/bevy).


Add the bevy crate to your project's Cargo.toml like this:

```toml
[dependencies]
bevy = "0.6" # make sure this is the latest version
```

This is the current `bevy` crate version:

<a href="https://crates.io/crates/bevy"><img src="https://img.shields.io/crates/v/bevy.svg" style="height: 1.7rem;"/></a>

**_NOTE:_** Bevy is currently being updated at a rapid pace. Taking a dependency on the git repo instead of the cargo crate will allow you to receive the latest updates as fast as possible. *However*, **there are often breaking changes made to APIs and behavior**. This means that it will be important to keep up with the latest developments with bevy. **This is not recommended for people who are just getting started with bevy.**
```toml
[dependencies]
bevy = { git = "https://github.com/bevyengine/bevy" }
```

In general it's a good idea to lock in to a specific commit hash, which gives you control over when you take updates. You can find the [latest commit hash here](https://github.com/bevyengine/bevy/commits/main) (to the right of each commit).

```toml
[dependencies]
bevy = { git = "https://github.com/bevyengine/bevy", rev = "25f62f7250a0d750068dc32533b9433f7985af98" }
```

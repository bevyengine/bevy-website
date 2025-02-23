+++
title = "Getting Started"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 2
+++

This section will help you get started on your Bevy journey as quickly as possible. It will walk you through setting up your development environment and writing a simple Bevy app.

## Quick Start

If you want to dive in immediately and you already have a working Rust setup, feel free to follow this "quick start" guide. Otherwise, move on to the next page.

{% callout() %}
Depending on your platform, you may have to [install additional dependencies].
You can also speed up compile times by following the ["fast compiles"] section.

[install additional dependencies]: /learn/quick-start/getting-started/setup/#installing-os-dependencies
["fast compiles"]: /learn/quick-start/getting-started/setup/#enable-fast-compiles-optional
{% end %}

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
    git checkout v0.15.0
    ```

4. Try the examples in the [examples folder](https://github.com/bevyengine/bevy/tree/latest/examples#examples)

    ```sh
    cargo run --example breakout
    ```

### Add Bevy as a Dependency

Bevy is [available as a library on crates.io](https://crates.io/crates/bevy).

The easiest way to add it to your project is to use `cargo add`:

```sh
cargo add bevy
```

Alternatively, you can manually add it to your project's Cargo.toml like this:

```toml
[dependencies]
bevy = "0.15" # make sure this is the latest version
```

Make sure to use the latest `bevy` crate version ([![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy)).

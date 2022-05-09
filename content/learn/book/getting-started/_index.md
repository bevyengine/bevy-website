+++
title = "Getting Started"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

## Quick Start

Assuming that [Rust](https://www.rust-lang.org/learn/get-started) is installed, install dependencies for either [Linux](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) or [Windows](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)

The setup for "fast compiles" is on the next page, with notable less compute time.

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
    git checkout v0.7.0
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
bevy = "0.7" # make sure this is the latest version
```

This is the current `bevy` crate version:

<a href="https://crates.io/crates/bevy"><img src="https://img.shields.io/crates/v/bevy.svg" style="height: 1.7rem;"/></a>

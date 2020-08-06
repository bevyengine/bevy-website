+++
title = "Getting Started"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

This section is a short guide that aims to get you started on your Bevy journey as quickly as possible. It will walk you through setting up your development environment and writing a simple Bevy app. However if you already have a working Rust setup and want to dive in now, feel free to follow this "quick start" guide. Otherwise, move on to the next page.

## Quick Start

### Try the Examples

1. Clone the Bevy repo:
    ```
    git clone https://github.com/bevyengine/bevy    
    ```
2. Navigate to the new "bevy" folder
    ```
    cd bevy
    ```
3. Try the examples in the [examples folder](https://github.com/bevyengine/bevy/tree/master/examples)
    ```
    cargo run --example breakout
    ```

### Add Bevy as a Dependency
Bevy is <a href="https://crates.io/crates/bevy" target="_blank">available as a library on crates.io</a>: [![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy). Add the bevy crate to your project's Cargo.toml like this:

```toml
[dependencies]
bevy = "0.1.0" # make sure this is the latest version
```
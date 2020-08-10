+++
title = "Getting Started"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

This section will help you get started on your Bevy journey as quickly as possible. It will walk you through setting up your development environment and writing a simple Bevy app.

## Quick Start

If you want to dive in immediately and you already have a working Rust setup, feel free to follow this "quick start" guide. Otherwise, move on to the next page.

Note: the "fast compiles" setup is on the next page, so you might want to read that section first.

### Try the Examples

1. Clone the [Bevy repo](https://github.com/bevyengine/bevy):
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
 
Bevy is [available as a library on crates.io](https://crates.io/crates/bevy).


Add the bevy crate to your project's Cargo.toml like this:

```toml
[dependencies]
bevy = "0.1.0" # make sure this is the latest version
```

This is the current `bevy` crate version:

<img src="https://img.shields.io/crates/v/bevy.svg" style="height: 1.7rem; margin-bottom: 2rem"/>
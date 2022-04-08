+++
title = "Setup"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

I know you are itching to start making games, but we need to do a _small_ amount of setup first.

## Rust Setup

All Bevy app and engine code is written in Rust. This means that before we begin, we need to set up our Rust development environment.

### Installing Rust

Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).

Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.

### Install OS dependencies

* [Linux](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
* Windows: Make sure to install [VS2019 build tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
* MacOS: No dependencies here

### Code Editor / IDE

You can use any code editor you want, but we highly recommend one that has a [Rust Analyzer](https://github.com/rust-analyzer/rust-analyzer) plugin. Rust Analyzer is still in development, but it already provides top-tier autocomplete and code intelligence. [Visual Studio Code](https://code.visualstudio.com/) has an officially supported [Rust Analyzer Extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer).

### Rust Learning Resources

The goal of this book is to learn Bevy, so it won't serve as a full Rust education. If you would like to learn more about the Rust language, check out the following resources:

* [**The Rust Book**](https://doc.rust-lang.org/book/): the best place to learn Rust from scratch
* [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/): learn Rust by working through live coding examples

## Create a new Bevy Project

Now we are ready to set up a Bevy project! Bevy is just a normal Rust dependency. You can either add it to an existing Rust project or create a new one. For completeness we will assume you are starting from scratch.

### Create a new Rust executable project

First, navigate to a folder where you want to create your new project. Then, run the following command to create a new folder containing our rust executable project:

```sh
cargo new my_bevy_game
cd my_bevy_game
```

Now run ```cargo run``` to build and run your project. You should see ```Hello, world!``` printed to your terminal. Open the ```my_bevy_game``` folder in your code editor of choice and take some time to look through the files.

```main.rs``` is the entry point of your program:

```rs
fn main() {
    println!("Hello, world!");
}
```

```Cargo.toml``` is your "project file". It contains metadata about your project such as its name, dependencies, and build configuration.

```toml
[package]
name = "my_bevy_game"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Add Bevy to your project's Cargo.toml

Bevy is [available as a library on crates.io](https://crates.io/crates/bevy), the official Rust package repository. Find the latest version number ([![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy)) and add it to your Cargo.toml file:

```toml
[package]
name = "my_bevy_game"
version = "0.1.0"
edition = "2021" # this needs to be 2021, or you need to set "resolver=2"

[dependencies]
bevy = "0.6" # make sure this is the latest version
```

### Cargo Workspaces

If you are using [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), you will also need to add the resolver to your Cargo.toml file in the root directory:

```toml
[workspace]
resolver = "2" # Important! wgpu/Bevy needs this!
```

### Build Bevy

Now run ```cargo run``` again. The Bevy dependencies should start building. This will take some time as you are essentially building an engine from scratch. You will only need to do a full rebuild once. Every build after this one will be fast!

Now that we have our Bevy project set up, we're ready to start making our first Bevy app!

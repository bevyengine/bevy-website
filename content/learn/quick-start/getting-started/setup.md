+++
title = "Setup"
insert_anchor_links = "right"
aliases = ["learn/book/getting-started/setup"]
[extra]
weight = 1
+++

We know you are itching to start making games, but Bevy requires us to do a _small_ amount of setup first.

## Setting up your development environment

While all Bevy app and engine code is written in Rust, there are still a number of additional tools we need to configure before we can begin developing.
The first of which is making sure that Rust itself is installed!

### Installing Rust

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.

Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).

Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your path.

#### Rust learning resources

The goal of this guide is to get started learning Bevy quickly, so it won't serve as a full Rust education.
If you would like to learn more about the Rust language, check out the following resources:

* [**The Rust Book**](https://doc.rust-lang.org/book/): The best place to learn Rust from scratch.
* [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/): Learn Rust by working through live coding examples.
* [**Rustlings**](https://github.com/rust-lang/rustlings): Learn Rust through a series of fun and interactive exercises.

### Installing OS dependencies

<details>
  <summary>

  #### Linux
  </summary>

  Follow the instructions at [Linux Dependencies](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md)
</details>

<details>
  <summary>

  #### Windows
  </summary>

  * Run the [Visual Studio C++ Build Tools installer](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  * For easy setup, select the ```Desktop development with C++``` workload in the installer.
  * For a minimal setup, follow these steps:
      1. In the installer, navigate to `Individual components`
      2. Select the latest `MSVC` for your architecture and version of Windows
      3. Select the latest `Windows SDK` for your version of Windows
      4. Select the `C++ CMake tools` for Windows component
      5. Install the components
</details>

<details>
  <summary>

  #### MacOS
  </summary>

  Install the Xcode command line tools with `xcode-select --install` or the [Xcode app](https://apps.apple.com/en/app/xcode/id497799835)
</details>

### Code Editor / IDE

You can use any code editor you want, but we highly recommend one that has a [rust-analyzer](https://github.com/rust-lang/rust-analyzer) plugin.
It's still in development, but it already provides top-tier autocomplete and code intelligence.
[Visual Studio Code](https://code.visualstudio.com/) has an officially supported [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Create a new Bevy project

Now we are ready to set up a Bevy project!
Bevy is just a normal Rust dependency.
You can either add it to an existing Rust project or create a new one.
For completeness we will assume you are starting from scratch.

### Create a new Rust executable project

First, navigate to a folder where you want to create your new project.
Then, run the following command to create a new folder containing our rust executable project:

```sh
cargo new my_bevy_game
cd my_bevy_game
```

Now run `cargo run` to build and run your project.
You should see `Hello, world!` printed to your terminal.
Open the `my_bevy_game` folder in your code editor of choice and take some time to look through the files.

`main.rs` is the entry point of your program:

```rs
fn main() {
    println!("Hello, world!");
}
```

`Cargo.toml` is your "project file".
It contains metadata about your project such as its name, dependencies, and build configuration.

```toml
[package]
name = "my_bevy_game"
version = "0.1.0"
edition = "2024"

[dependencies]
```

### Add Bevy as a dependency

Bevy is [available as a library on crates.io](https://crates.io/crates/bevy), the official Rust package repository.

The easiest way to add it to your project is to use `cargo add`:

```sh
cargo add bevy
```

<details>
  <summary>Alternate - Manually Add Bevy to Cargo.toml</summary>
  You can also manually add it to your project's Cargo.toml like this:

  ```toml
  [package]
  name = "my_bevy_game"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  bevy = "0.18.1" # make sure this is the latest version
  ```
</details>

Make sure to use the latest `bevy` crate version ([![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy)).

### Build Bevy

Now run `cargo run` again. The Bevy dependencies should start building. This will take some time as you are essentially building an engine from scratch. You will only need to do a full rebuild once. Every build after this one will be fast!

Now that we have our Bevy project set up, we're ready to start making our first Bevy app!

{% callout(type="note") %}
If something went wrong, check out our [troubleshooting section](/learn/quick-start/troubleshooting/) or [ask for help on our Discord](https://discord.gg/bevy).
{% end %}

+++
title = "Setup"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

I know you are itching to start making games, but we need to do a _small_ amount of setup first.

## Rust Setup

All Bevy app and engine code is written in Rust. This means that before we begin, we need to set up our Rust development environment.

### Installing Rust

Install Rust by following the <a href="https://www.rust-lang.org/learn/get-started" target="_blank">Rust Getting Started Guide</a>.

Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.

### Code Editor / IDE

You can use any code editor you want, but we highly recommend one that has a <a href="https://github.com/rust-analyzer/rust-analyzer" target="_blank">Rust Analyzer</a> plugin. Rust Analyzer is still in development, but it already provides top-tier autocomplete and code intelligence. <a href="https://code.visualstudio.com/">Visual Studio Code</a> has an officially supported <a href="https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer">Rust Analyzer Extension</a>. 

### Rust Learning Resources

The goal of this book is to learn Bevy, so it won't serve as a full Rust education. If you would like to learn more about the Rust language, check out the following resources:

* <b><a href="https://doc.rust-lang.org/book/" target="_blank">The Rust Book</a></b>: the best place to learn Rust from scratch
* <b><a href="https://doc.rust-lang.org/rust-by-example/" target="_blank">Rust by Example</a></b>: learn Rust by working through live coding examples


## Create a new Bevy Project

Now we are ready to set up a Bevy project! Bevy is just a normal Rust dependency. You can either add it to an existing Rust project or create a new one. For completeness we will assume you are starting from scratch.

### Create a new Rust executable project

First, navigate to a folder where you want to create your new project. Then, run the following command to create a new folder containing our rust executable project:

```
cargo new my_bevy_game --bin
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
authors = ["You <you@veryrealemail.com>"]
edition = "2018"

[dependencies]
```

### Enable Fast Compiles (Optional)

Bevy can be built just fine using default configuration on stable Rust. However for really fast iterative compiles, we recommend the following configuration:

* **LLD linker**: The Rust compiler spends a lot of time in the "link" step. LLD is _much faster_ at linking than the default Rust linker. To install LLD, find your OS below and run the given command:
    * **Ubuntu**: `sudo apt-get install lld`
    * **Arch**: `sudo pacman -S lld`
    * **Windows**: `scoop install llvm` (using [scoop](https://scoop.sh/) package manager)
    * **MacOS**: Sorry MacOS users ... modern LLD does not support MacOS. They are working on a rewrite, but it will take time. Fortunately Bevy will soon support dynamic linking of App Plugins, which will give massive iterative compile speedups and make LLD less necessary.
* **Nightly Rust Compiler**: This gives access to the latest performance improvements and "unstable" optimizations
    ```
    rustup toolchain install nightly
    rustup default nightly
    ```
    * You can always switch back to stable by running: ```rustup default stable```.
* **Generic Sharing**: Allows crates to share monomorphized generic code instead of duplicating it. In some cases this allows us to "precompile" generic code so it doesn't affect iterative compiles. This is only available on nightly Rust.

To enable fast compiles, install the nightly rust compiler and LLD. Then copy [this file](https://github.com/bevyengine/bevy/blob/master/.cargo/config_fast_builds) to `YOUR_WORKSPACE/.cargo/config`. For the project in this guide, that would be `my_bevy_game/.cargo/config`.

### Add Bevy to your project's Cargo.toml


Bevy is <a href="https://crates.io/crates/bevy" target="_blank">available as a library on crates.io</a>, the official Rust package repository. Find the latest version number ([![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy)) and add it to your Cargo.toml file:

```toml
[package]
name = "my_bevy_game"
version = "0.1.0"
authors = ["You <you@veryrealemail.com>"]
edition = "2018"

[dependencies]
bevy = "0.1.0" # make sure this is the latest version
```

Run ```cargo run``` again. The Bevy dependencies should start building. This will take some time as you are essentially building an engine from scratch. You will only need to do a full rebuild once. Every build after this one will be fast!

Now that we have our Bevy project set up, we're ready to start making our first Bevy app!
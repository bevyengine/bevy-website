+++
title = "Setup"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

I know you are itching to start making games, but we need to do a _small_ amount of setup first.

## Rust Setup

All Bevy app and engine code is written in Rust. This means that before we begin, we need to set up our Rust development environment. Fortunately this is very straightforward!

### Installing Rust

Install Rust by following the <a href="https://www.rust-lang.org/learn/get-started" target="_blank">Rust Getting Started Guide</a>.

Alternatively you can just run the official ```rustup``` utility from the command line if you are running unix-like operating systems such as Linux or macOS:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Bevy can be built using stable Rust, but if you want fast compiles you should use a nightly compiler. Feel free to skip the following steps if you don't mind slower compiles.

```
rustup toolchain install nightly
```

Then make nightly the default by running:

```
rustup default nightly
```

You can always switch back to stable by running: ```rustup default stable```.

Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path. Try running ```cargo version``` and make sure it returns a "nightly" compiler version.

### Code Editor / IDE

You can use any code editor you want, but we highly recommend one that has a <a href="https://github.com/rust-analyzer/rust-analyzer" target="_blank">Rust Analyzer</a> plugin. Rust Analyzer is still in development, but it already provides top-tier autocomplete and code intelligence. <a href="https://code.visualstudio.com/">Visual Studio Code</a> has an officially supported <a href="https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer">Rust Analyzer Extension</a>. 

### Rust learning resources

The goal of this book is to learn Bevy, so it won't serve as a full Rust education. If you would like to learn more about the Rust language, check out the following resources:

* <b><a href="https://doc.rust-lang.org/book/" target="_blank">The Rust Book</a></b>: the best place to learn Rust from scratch
* <b><a href="https://doc.rust-lang.org/rust-by-example/" target="_blank">Rust by Example</a></b>: learn Rust by working through live coding examples


## Create a new Bevy Project

Now we are ready to set up a Bevy project! Bevy is just a normal Rust dependency. You can either add it to an existing Rust project or create a new one. For completeness we will assume you are starting from scratch.

### Create a new Rust executable project

First, navigate to a folder where you want to create your new project. Then, run the following command to create a new folder containing our rust executable project:

```
cargo new my_bevy_game --bin
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
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

Bevy relies heavily on improvements in the Rust language and compiler.
As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.

Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).

Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.

### Install OS dependencies

* [Linux](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
* Windows: Make sure to install [VS2019 build tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
* MacOS: Install the Xcode command line tools with `xcode-select --install` or the [Xcode app](https://apps.apple.com/en/app/xcode/id497799835)

### Code Editor / IDE

You can use any code editor you want, but we highly recommend one that has a [rust-analyzer](https://github.com/rust-lang/rust-analyzer) plugin. It's still in development, but it already provides top-tier autocomplete and code intelligence. [Visual Studio Code](https://code.visualstudio.com/) has an officially supported [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

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

### Add Bevy as a dependency

Bevy is [available as a library on crates.io](https://crates.io/crates/bevy), the official Rust package repository.

The easiest way to add it to your project is to use `cargo add`:

```sh
cargo add bevy
```

Alternatively, you can manually add it to your project's Cargo.toml like this:

```toml
[package]
name = "my_bevy_game"
version = "0.1.0"
edition = "2021" # this needs to be 2021, or you need to set "resolver=2"

[dependencies]
bevy = "0.9" # make sure this is the latest version
```

Make sure to use the latest `bevy` crate version ([![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy)).

### Cargo Workspaces

If you are using [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), you will also need to add the resolver to your Cargo.toml file in the root directory:

```toml
[workspace]
resolver = "2" # Important! wgpu/Bevy needs this!
```

### Compile with Performance Optimizations

While it may not be an issue for simple projects, debug builds in Rust can be _very slow_ - especially when you start using Bevy to make real games.

It's not uncommon for debug builds using the default configuration to take multiple minutes to load large 3D models, or for the framerate for simple scenes to drop to near-unplayable levels.

Fortunately, there is a simple fix, and we don't have to give up our fast iterative compiles! Add this to your `Cargo.toml`:

```toml
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

You might think to simply develop in release mode instead, but we recommend against this as it can worsen the development experience by slowing down recompiles and disabling helpful debug symbols and assertions.

### Enable Fast Compiles (Optional)

Bevy can be built just fine using default configuration on stable Rust. However for maximally fast iterative compiles, we recommend the following configuration:

* **Enable Bevy's Dynamic Linking Feature**: This is the most impactful compilation time decrease! If `bevy` is a dependency you can compile the binary with the "dynamic" feature flag (enables dynamic linking). Note that right now, this doesn't work on Windows.

  ```sh
  cargo run --features bevy/dynamic
  ```

  If you don't want to add the `--features bevy/dynamic` to each run, this flag can permanently be set via `Cargo.toml`:

  ```toml
  [dependencies]
  bevy = { version = "0.9.0", features = ["dynamic"] }
  ```

  NOTE: Remember to revert this before releasing your game! Otherwise you will need to include `libbevy_dylib` alongside your game if you want it to run. If you remove the "dynamic" feature, your game executable can run standalone.

* **LLD linker**: The Rust compiler spends a lot of time in the "link" step. LLD is _much faster_ at linking than the default Rust linker. To install LLD, find your OS below and run the given command:
  * **Ubuntu**: `sudo apt-get install lld`
  * **Arch**: `sudo pacman -S lld`
  * **Windows**: Ensure you have the latest [cargo-binutils](https://github.com/rust-embedded/cargo-binutils)

    ```sh
    cargo install -f cargo-binutils
    rustup component add llvm-tools-preview
    ```

  * **MacOS**: Modern LLD does not yet support MacOS, but we can use zld instead: `brew install michaeleisel/zld/zld`
* **Alternative - mold linker**: mold is _up to 5× (five times!) faster_ than LLD, but with a few caveats like limited platform support and occasional stability issues.  To install mold, find your OS below and run the given command:
  * **Ubuntu**: `sudo apt-get install mold`
  * **Arch**: `sudo pacman -S mold`
  * **Windows**: mold does not yet support Windows. [See this tracking issue](https://github.com/rui314/mold/issues/190) for more information.
  * **MacOS**: mold does not yet support MacOS. [See this tracking issue](https://github.com/rui314/mold/issues/189) for more information.

    You will also need to add the following to your Cargo config at `YOUR_WORKSPACE/.cargo/config.toml`:

    ```toml
    [target.x86_64-unknown-linux-gnu]
    linker = "clang"
    rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
    ```

    NOTE: Disabling `bevy/dynamic` may improve the performance of this linker.

* **Nightly Rust Compiler**: This gives access to the latest performance improvements and "unstable" optimizations

    Create a ```rust-toolchain.toml``` file in the root of your project, next to ```Cargo.toml```.

    ```toml
    [toolchain]
    channel = "nightly"
    ```

    For more information, see [The rustup book: Overrides](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file).

* **Generic Sharing**: Allows crates to share monomorphized generic code instead of duplicating it. In some cases this allows us to "precompile" generic code so it doesn't affect iterative compiles. This is only available on nightly Rust.

To enable fast compiles, install the nightly rust compiler and LLD. Then copy the contents of [this file](https://github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds) to `YOUR_WORKSPACE/.cargo/config.toml`. For the project in this guide, that would be `my_bevy_game/.cargo/config.toml`.

If something went wrong, check out our [troubleshooting section](/learn/book/troubleshooting/) or [ask for help on our Discord](https://discord.com/invite/gMUk5Ph).

### Build Bevy

Now run ```cargo run``` again. The Bevy dependencies should start building. This will take some time as you are essentially building an engine from scratch. You will only need to do a full rebuild once. Every build after this one will be fast!

Now that we have our Bevy project set up, we're ready to start making our first Bevy app!

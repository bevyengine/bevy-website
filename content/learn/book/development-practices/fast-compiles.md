+++
title = "Fast Compiles"
insert_anchor_links = "right"
[extra]
weight = 2
status = 'hidden'
+++

{% todo() %}

* Explain why you might want faster compiles
{% end %}

* **Enable Bevy's Dynamic Linking Feature**: This is the most impactful compilation time decrease! If `bevy` is a dependency you can compile the binary with the "dynamic" feature flag (enables dynamic linking):

    ```sh
    cargo run --features bevy/dynamic
    ```

    If you don't want to add the `--features bevy/dynamic` to each run, this flag can permanently be set via `Cargo.toml`:

    ```toml
    [dependencies]
    bevy = { version = "0.5.0", features = ["dynamic"] }
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
* **Nightly Rust Compiler**: This gives access to the latest performance improvements and "unstable" optimizations

 ```sh
    # Install the nightly toolchain
    rustup toolchain install nightly
    # Configure your current project to use nightly (run this command within the project)
    rustup override set nightly
    # OR configure cargo to use nightly for all projects -- switch back with `rustup default stable`
    rustup default nightly
    ```

  * You can use `cargo +nightly ...` if you don't want to change the default to nightly.
* **Generic Sharing**: Allows crates to share monomorphized generic code instead of duplicating it. In some cases this allows us to "precompile" generic code so it doesn't affect iterative compiles. This is only available on nightly Rust.

To enable fast compiles, install the nightly rust compiler and LLD. Then copy [this file](https://github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds.toml) to `YOUR_WORKSPACE/.cargo/config.toml`. For the project in this guide, that would be `my_bevy_game/.cargo/config.toml`.

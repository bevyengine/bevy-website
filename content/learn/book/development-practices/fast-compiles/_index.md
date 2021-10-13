+++
title = "Fast compiles"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

TODO: explain why you might want faster compiles

* **Bevy's Dynamic Linking Feature**: This is the most impactful iterative compilation time decrease! It requires no special setup except on Windows, where you also have to follow the three numbered steps below for it to work. If `bevy` is a dependency you can compile the binary with the `dynamic` feature flag (enables dynamic linking):

  ```sh
  cargo run --features bevy/dynamic
  ```
  
  If you don't want to add the `--features bevy/dynamic` to each run, this flag can permanently be set via `Cargo.toml`:
  
  ```toml
  [dependencies]
  bevy = { version = "0.5.0", features = ["dynamic"] }
  ```
  
  NOTE: Remember to revert this before releasing your game! Otherwise you will need to include `libbevy_dylib` alongside your game if you want it to run. If you remove the `dynamic` feature, your game executable can run standalone.

For the fastest iterative compile times, we recommend the following configuration as well. You need to follow all three steps for any effect:

1. **LLD linker**: The Rust compiler spends a lot of time in the "link" step. LLD is _much faster_ at linking than the default Rust linker. To install LLD, find your OS below and run the given command:
   * **Ubuntu**: `sudo apt-get install lld clang`
   * **Arch**: `sudo pacman -S lld`
   * **Windows**: Ensure you have the latest [cargo-binutils](https://github.com/rust-embedded/cargo-binutils)

      ```sh
      cargo install -f cargo-binutils
      rustup component add llvm-tools-preview
      ```

   * **MacOS**: Modern LLD does not yet support MacOS, but we can use zld instead: `brew install michaeleisel/zld/zld`
2. **Nightly Rust Compiler**: This gives access to the latest performance improvements and "unstable" optimizations

   ```sh
   # Install the nightly toolchain
   rustup toolchain install nightly
   # EITHER configure your current project to use nightly (run this command within the project)
   rustup override set nightly
   # OR configure cargo to use nightly for all projects -- switch back with `rustup default stable`
   rustup default nightly
   ```

   You can use `cargo +nightly ...` if you don't want to change the default to nightly, but just want to use it once for the current command.

3. **Configure cargo**: With the linker installed and nightly rust activated, all we need to do now is put them to proper use. Copy [this file](https://github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds) to `YOUR_WORKSPACE/.cargo/config.toml`. For the project in this guide, that would be `my_bevy_game/.cargo/config.toml`.

   Beyond enabling the LLD linker, this configuration file also activates **Generic Sharing** (unless you use Windows), which allows crates to share monomorphized generic code instead of duplicating it. In some cases this allows us to "precompile" generic code so it doesn't affect iterative compiles. This is only available on nightly Rust.

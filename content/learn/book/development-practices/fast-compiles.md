+++
title = "Fast Compiles"
insert_anchor_links = "right"
[extra]
weight = 1
+++

### Enable Fast Compiles (Optional)

Bevy can be built just fine using default configuration on stable Rust.
Unfortunately, the compile times are rather long.
This section explains how to speed up iterative compiles: the amount of time it takes to rebuild your project after changing a single file.

<details>
  <summary>

  #### Dynamic Linking
  </summary>

  This is the most impactful compilation time decrease!
  You can compile `bevy` as dynamic library, preventing it from having to be statically linked each time you rebuild your project. You can enable this with the `dynamic_linking` feature flag.

  ```sh
  cargo run --features bevy/dynamic_linking
  ```

  If you don't want to add the `--features bevy/dynamic_linking` to each run, this flag can permanently be set with this command (edits `Cargo.toml` for you):

  ```sh
  cargo add bevy -F dynamic_linking
  ```

  {% callout(type="warning") %}
  On Windows you must also enable the [performance optimizations](#compile-with-performance-optimizations) or you will get a ["too many exported symbols"](https://github.com/bevyengine/bevy/issues/1110#issuecomment-1312926923) error.

  In order to run `cargo test --doc`, you must also add the path returned by `rustc --print target-libdir` to your `PATH` environment variable.
  {% end %}

  {% callout(type="note") %}
  Shipping your game with dynamic linking enabled is not recommended because it requires you to include `libbevy_dylib` alongside your game, it prevents certain optimizations, and can increase the size of your game.
  If you remove the `dynamic_linking` feature, your game executable can run standalone.
  {% end %}
</details>

<details>
  <summary>

  #### Alternative Linkers
  </summary>

  The Rust compiler spends a lot of time in the final "link" step, especially with a massive library like Bevy.
  `lld` is _much faster_ at linking than the default Rust linker.
  To install LLD, find your OS below and run the given command.

  <details>
    <summary>LLD Installation</summary>

  * **Ubuntu**: `sudo apt-get install lld clang`
  * **Fedora**: `sudo dnf install lld clang`
  * **Arch**: `sudo pacman -S lld clang`
  * **Windows**: Ensure you have the latest [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) as this lets commands like `cargo run` use the LLD linker automatically.

    ```sh
    cargo install -f cargo-binutils
    rustup component add llvm-tools-preview
    ```

  * **MacOS**: On MacOS, the default system linker `ld-prime` is faster than LLD.

  </details>

  Then, add one of the following to your Cargo config at `/path/to/project/.cargo/config.toml` (where `/path/to/project` is the directory which contains `Cargo.toml`) depending on your OS:

  ```toml
  # for Linux
  [target.x86_64-unknown-linux-gnu]
  linker = "clang"
  rustflags = ["-C", "link-arg=-fuse-ld=lld"]

  # for Windows
  [target.x86_64-pc-windows-msvc]
  linker = "rust-lld.exe"
  ```

  <details>
    <summary>Alternative - Mold</summary>

  Mold is _up to 5× (five times!) faster_ than LLD, but with a few caveats like limited platform support and occasional stability issues.  To install mold, find your OS below and run the given command:

  * **Ubuntu**: `sudo apt-get install mold clang`
  * **Fedora**: `sudo dnf install mold clang`
  * **Arch**: `sudo pacman -S mold clang`
  * **Windows**: Support not planned; [See this tracking issue](https://github.com/rui314/mold/issues/1069#issuecomment-1653436823) for more information.
  * **MacOS**: Available as [sold](https://github.com/bluewhalesystems/sold), but this is unnecessary since the default linker is just as fast.

  You will also need to add the following to your Cargo config at `/path/to/project/.cargo/config.toml`:

  ```toml
  [target.x86_64-unknown-linux-gnu]
  linker = "clang"
  rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
  ```

  {% callout(type="note") %}
  Disabling `bevy/dynamic_linking` may improve Mold's performance.
  <sup>[citation needed]</sup>
  {% end %}

  </details>
</details>

<details>
  <summary>
  
  #### Nightly Rust Compiler
  </summary>

  This gives access to the latest performance improvements and "unstable" optimizations, including [generic sharing](#generic-sharing) below.

  Create a ```rust-toolchain.toml``` file in the root of your project, next to ```Cargo.toml```.

  ```toml
  [toolchain]
  channel = "nightly"
  ```

  For more information, see [The rustup book: Overrides](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file).
</details>

<details>
  <summary>
  
  #### Cranelift
  </summary>

  This uses a new nightly-only codegen that is about 30% faster at compiling than LLVM. 
  It currently works best on Linux.

  To install cranelift, run the following.
  ```
  rustup component add rustc-codegen-cranelift-preview --toolchain nightly
  ```

  To activate it for your project, add the following to your `.cargo/config.toml`.
  ```toml
  [unstable]
  codegen-backend = true

  [profile.dev]
  codegen-backend = "cranelift"

  [profile.dev.package."*"]
  codegen-backend = "llvm"
  ```

  This enables faster compiles for your binary, but builds Bevy and other dependencies with the more-optimized LLVM backend. See the [cranelift setup guide](https://github.com/rust-lang/rustc_codegen_cranelift#download-using-rustup) for
  details on other ways in which cranelift can be enabled. The installation process for Windows is a bit more involved. Consult the linked documentation for help.
  MacOS builds can currently crash on Bevy applications, so you should still wait a bit before using cranelift on that system.

  While cranelift is very fast to compile, the generated binaries are not optimized for speed. Additionally, it is generally still immature, so you may run into issues with it. 
  Notably, Wasm builds do not work yet.

  When shipping your game, you should still compile it with LLVM.
</details>

<details>
  <summary>

  #### Generic Sharing
  </summary>

  Allows crates to share monomorphized generic code instead of duplicating it.
  In some cases this allows us to "precompile" generic code so it doesn't affect iterative compiles.
  This is currently only available on nightly Rust ([see above](#nightly-rust-compiler)).

  ##### Generic sharing setup

  See [this file](https://github.com/bevyengine/bevy/blob/latest/.cargo/config_fast_builds.toml) for a more comprehensive, cross-platform example.

  ```toml
  # /path/to/project/.cargo/config.toml
  [target.x86_64-unknown-linux-gnu]
  rustflags = [
    # (Nightly) Make the current crate share its generic instantiations
    "-Zshare-generics=y",
  ]
  ```
</details>

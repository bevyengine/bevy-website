+++
title = "Fast Compiles"
insert_anchor_links = "right"
[extra]
weight = 1
+++

The previous chapters have focused on how to add and organize your code, but now we need to run it! Doing so involves running `cargo run`, which invokes the [Rust Compiler](https://doc.rust-lang.org/rustc/what-is-rustc.html) to compile and build your project. However, compile times can be quite long using the Rust Compiler's default settings. Thankfully we have several methods that can speed compile time up.

## Linking Optimizations

The Rust Compiler takes your source code turns it into binary code, either as a library to be used in other projects or as a standalone executable program. **Linking** is a major part of the compilation process. During this stage all program files, libraries, and external dependencies are collected and assembled for your project's code to access. By default the Rust Compiler will **statically** link your project, meaning that all code (including external dependency code) is placed inside your project executable.

### Dynamic Linking

In contrast to static linking, **dynamic** linking creates references to any shared libraries instead of repeatedly copying them. These references are loaded into memory at runtime and results in smaller binary sizes. This is the most impactful compilation time decrease!

You can compile `bevy` as dynamic library, preventing it from having to be statically linked each time you rebuild your project. You can enable this with the `dynamic_linking` feature flag.

```sh
cargo run --features bevy/dynamic_linking
```

If you don't want to add the `--features bevy/dynamic_linking` to each run, this flag can permanently be set with this command:

```sh
# This edits your project Cargo.toml file.
cargo add bevy -F dynamic_linking
```

{% callout(type="warning") %}
On Windows you must also enable the [performance optimizations] or you will get a ["too many exported symbols"](https://github.com/bevyengine/bevy/issues/1110#issuecomment-1312926923) error.

[performance optimizations]: /learn/book/releasing-projects/optimizing-performance

In order to run `cargo test --doc`, you must also add the path returned by `rustc --print target-libdir` to your `PATH` environment variable.
{% end %}

{% callout(type="note") %}
Shipping your game with dynamic linking enabled is not recommended because it requires you to include `libbevy_dylib` alongside your game, it prevents certain optimizations, and can increase the size of your game.
If you remove the `dynamic_linking` feature, your game executable can run standalone.
{% end %}

### Alternative Linkers

If dynamic linking isn't preferred or possible, we can use alternative linkers during compilation. However, changing away from your platform's default linker might not provide a substantial (or even noticeable) benefit. As an example, [with Rust 1.90.0](https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/) the Rust Compiler began using LLVM's `lld` linker on `x86_64-unknown-linux-gnu` target platforms by default to provide a faster linker.

<details>
  <summary>Mold</summary>
  
  [Mold](https://github.com/rui314/mold) is an alternative linker for Linux systems claiming to be faster than LLVM's `lld` linker. However, it also comes with drawbacks, such as limited platform support and occasional stability issues. To install Mold, use your preferred package manager.
  
Examples:
  
* **Ubuntu**: `sudo apt-get install mold clang`
* **Fedora**: `sudo dnf install mold clang`
* **Arch**: `sudo pacman -S mold clang`
  
You will also need to add the following to your Cargo config at `/path/to/project/.cargo/config.toml`:

  ```toml
  [target.x86_64-unknown-linux-gnu]
  linker = "clang"
  rustflags = ["-C", "link-arg=-fuse-ld=/path/to/mold"]
  # Where "/path/to/mold" is the location of your mold installation.
  ```
  
  {% callout(type="note") %}
    Disabling `bevy/dynamic_linking` may improve Mold's performance.
    <sup>[citation needed]</sup>
  {% end %}
</details>

## Nightly Rust Compiler

This gives access to the latest performance improvements and "unstable" optimizations, including [generic sharing](#generic-sharing) below.

Create a ```rust-toolchain.toml``` file in the root of your project, next to ```Cargo.toml```.

```toml
[toolchain]
channel = "nightly"
```

For more information, see [The rustup book: Overrides](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file).

## Cranelift

This uses a new nightly-only codegen that is about 30% faster at compiling than LLVM.
It currently works best on Linux.

To install Cranelift, run the following.

```sh
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
details on other ways in which Cranelift can be enabled. The installation process for Windows is a bit more involved. Consult the linked documentation for help.
MacOS builds can currently crash on Bevy applications, so you should still wait a bit before using cranelift on that system.

While Cranelift is very fast to compile, the generated binaries are not optimized for speed. Additionally, it is generally still immature, so you may run into issues with it.
Notably, Wasm builds do not work yet.

When shipping your game, you should still compile it with LLVM.

## Generic Sharing

Allows crates to share monomorphized generic code instead of duplicating it.
In some cases this allows us to "precompile" generic code so it doesn't affect iterative compiles.
This is currently only available on nightly Rust ([see above](#nightly-rust-compiler)).

### Generic sharing setup

See [this file](https://github.com/bevyengine/bevy/blob/latest/.cargo/config_fast_builds.toml) for a more comprehensive, cross-platform example.

```toml
# /path/to/project/.cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = [
# (Nightly) Make the current crate share its generic instantiations
"-Zshare-generics=y",
]
```

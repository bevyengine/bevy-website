
+++
title = "Compiling Release Builds"
insert_anchor_links = "right"
[extra]
weight = 0
+++

During compilation, the Rust compiler can make a number of different choices,
trading off debuggability and compile times against final binary size and performance.

There are a number of important settings we can experiment with here.
Compilation settings are recorded in your root `Cargo.toml` file, and should be grouped into [profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) designed for different tasks.
You should store your settings for optimized builds under the default `release` profile, like so:

```toml
# Optimized for size
# Good for mobile or web
[profile.release]
opt-level = 'z'
lto = "fat"
codegen-units = 1
strip = true

# Optimized for speed
# Good for desktop
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = "debuginfo"
```

Whenever you call `cargo build --release`,
these options will be used by `rustc` to produce a release-ready build of your project.

You can perform similar modifications to your `dev` profile, which is used by default when using `cargo run`.
This produces settings that preserve the needed debug info while also compile quickly
and ensuring that your performance is reasonable for testing.
We always recommend enabling optimizations in your dependencies (like `bevy`) using:

```toml
[profile.dev.package."*"]
opt-level = 3
```

Much of the advice in this chapter comes from the excellent [min-sized-rust](https://github.com/johnthagen/min-sized-rust) guide.
The options listed here are relatively safe and approachable, costing you only compilation time.
Even more advanced tips (such as `panic = "abort"`) with more nuanced tradeoffs can be found there.

## Rust Compiler Settings

### `opt-level`

The most important setting is "what do we tell the compiler to optimize for".
This is controlled with the [opt-level](https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level) setting.

In most cases, `opt-level='z` will produce the smallest binary (at the cost of runtime performance),
but you should experiment with other values, as the effect can vary from project-to-project.

Modifying this setting will slow down compilation.

### Link-Time Optimization

By default, each compilation unit (typically a crate) is compiled and optimized in isolation.
[Link time optimization](https://llvm.org/docs/LinkTimeOptimization.html) improves optimization
by allowing the linker to take a more global view.

In addition to performance gains,
this can improve dead code detection, reducing binary size.

Setting `lto = "fat"` will result in improved results beyond basic "thin" LTO performed with `lto = true`.

Modifying this setting will slow down compilation.

### Code-Gen Units

Similarly, we can eliminate parallelism during compilation by setting `codegen-units = 1`.
This will allow the compiler to find additional optimizations in much the same way.

Modifying this setting will slow down compilation.

### Stripping Symbols

The [`strip`](https://doc.rust-lang.org/cargo/reference/profiles.html#strip) setting controls which symbols are removed
by the compiler, reducing the final binary size and somewhat surprisingly, [improving compile times](https://kobzol.github.io/rust/rustc/2025/05/20/disable-debuginfo-to-improve-rust-compile-times.html).

If you are not optimizing for binary size, we recommend only stripping debug symbols using `strip = "debuginfo"`,
rather than `strip = true`, as the symbol table is used by many profiling tools.

## `wasm-opt`

Binary size is particularly important on the web, as it impacts page load times.
When working with WASM builds specifically, we have another tool: [`wasm-opt`](https://github.com/WebAssembly/binaryen).

This is a distinct step that runs *after* compilation,
and can be tuned to optimize speed or size.
This should be called *after* [`wasm-bindgen`](https://github.com/wasm-bindgen/wasm-bindgen) or similar tools.

```sh
# Optimize for size (z profile).
wasm-opt -Oz -o output.wasm input.wasm

# Optimize for size (s profile).
wasm-opt -Os -o output.wasm input.wasm

# Optimize for speed.
wasm-opt -O3 -o output.wasm input.wasm

# Optimize for both size and speed.
wasm-opt -O -ol 100 -s 100 -o output.wasm input.wasm
```

+++
title = "Troubleshooting"
insert_anchor_links = "right"
aliases = ["learn/book/troubleshooting"]
[extra]
weight = 5
+++

Is something in Bevy not working as expected? Maybe one of these will resolve your problem:

## Unable to find a GPU

```txt
thread 'main' panicked at 'Unable to find a GPU! Make sure you have installed required drivers!'
```

This error message means that bevy is unable to draw to your screen.
Causes include:

1. Vulkan-compatible drivers not installed. To fix this, install/update the drivers. On Linux this may be `vulkan-intel` or `vulkan-radeon`.
2. Trying to run an example on a headless machine. To fix this, install a GPU!

## Unable to debug dynamically linked Bevy application in VSCode on Windows

```txt
The program '[10184] my-game.exe' has exited with code -1073741515 (0xc0000135).
```

Whilst `cargo run` may load the application successfully, running via the debugging UI in VSCode may yield the above error. This error means
that the required libraries were not loaded correctly (likely due to a pathing quirk with VSCode debug extensions on Windows).

Edit your launch configurations in `.vscode/launch.json` so that the rust libraries are found correctly.

For `cppvsdbg`:

```json
"environment": [
    {"name":"PATH", "value":"%USERPROFILE%/.rustup/toolchains/nightly-x86_64-pc-windows-msvc/bin;${workspaceFolder}/target/debug/deps;%PATH%"}
    // Switch `nightly` to `stable` if you're using Rust stable
],
```

Or for `codelldb`:

```json
"env": {
    "PATH": "${env:USERPROFILE}/.rustup/toolchains/nightly-x86_64-pc-windows-msvc/bin;${workspaceFolder}/target/debug/deps;${env:PATH}",
    // Switch `nightly` to `stable` if you're using Rust stable
},
```

## Unable to compile `getrandom` when building for web

Bevy binaries tend to depend on `getrandom` through the `uuid` crate among
others, and the standard getrandom implementation does not work for (some)
web/wasm targets.

```txt
   Compiling getrandom v0.3.4
   Compiling getrandom v0.4.2
error: The wasm32-unknown-unknown targets are not supported by default; you may need to enable the "wasm_js" crate feature.
```

If you only had one tree dependency on `getrandom`, then adding
`getrandom = { version = "*", features = ["wasm_js"] }` would solve the issue
like
[`getrandom`](https://docs.rs/getrandom/latest/getrandom/#webassembly-support)
suggests.

However, if your crate depends on multiple versions of `getrandom` (which it
likely will indirectly), this will not be enough: The wildcard version pattern
still ends up reflecting only a single version of the crate you want to add a
feature for.

Going through `cargo tree --invert getrandom` you can find crates in your
dependency tree that use `getrandom` (e.g. `uuid` and `ahash`), and figure out
if they have a [solution](https://docs.rs/uuid/latest/uuid/#webassembly) that
applies the fix for you. This can feel a little indirect and fragile though,
since you might notice that a lot of crates share the same getrandom version,
and you just want that crate to change its implementation.

If you want to directly enforce a feature for multiple versions of a crate, you
have to "rename" them. Cargo does not know that you want a feature to apply to
every instance of the crate (what if a cargo feature only exists for some
versions of the dependency?), so it has to be stated multiple times without
creating a name conflict in your dependency table:

```toml
# in `Cargo.toml`:
[target.'cfg(all(target_family = "wasm", any(target_os = "unknown", target_os = "none")))'.dependencies]
getrandom_a = { package = "getrandom", version = "0.3.4", features = ["wasm_js"] }
getrandom_b = { package = "getrandom", version = "0.4.2", features = ["wasm_js"] }
```

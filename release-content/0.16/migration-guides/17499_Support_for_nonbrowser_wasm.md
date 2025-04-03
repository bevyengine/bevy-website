Bevy now has support for the [`wasm32v1-none` target], which is a barebones `no_std` version of `wasm32-unknown-unknown` that disables all features past the original [W3C WebAssembly Core 1.0 spec]. As part of this change, Bevy's browser-specific WASM features have been put behind of the `web` feature flag, which is enabled by default. If you have `default-features = false` and wish to build Bevy to run on a browser, you will need to re-enable this flag:

```toml
# 0.15
[dependencies]
bevy = { version = "0.15", default-features = false }

# 0.16
[dependencies]
bevy = { version = "0.16", default-features = false, features = ["web"] }
```

[`wasm32v1-none` target]: https://doc.rust-lang.org/rustc/platform-support/wasm32v1-none.html
[W3C WebAssembly Core 1.0 spec]: https://www.w3.org/TR/wasm-core-1/

WASM does not support dynamic libraries that can be linked to during runtime. Before, Bevy would fail to compile if you enabled the `dynamic_linking` feature.

```bash
$ cargo build --target wasm32-unknown-unknown --features bevy/dynamic_linking
error: cannot produce dylib for `bevy_dylib v0.13.2` as the target `wasm32-unknown-unknown` does not support these crate types
```

Now, Bevy will fallback to static linking for all WASM targets. If you enable `dynamic_linking` for development, you no longer need to disable it for WASM.

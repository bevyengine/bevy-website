`LogPlugin` used to silently override the panic handler on WASM targets. This functionality has now been split out into the new `PanicHandlerPlugin`, which was added to `DefaultPlugins`.

If you want nicer error messages on WASM but don't use `DefaultPlugins`, make sure to manually add `PanicHandlerPlugin` to the app.

```rust
App::new()
    .add_plugins((MinimalPlugins, PanicHandlerPlugin))
    .run()
```

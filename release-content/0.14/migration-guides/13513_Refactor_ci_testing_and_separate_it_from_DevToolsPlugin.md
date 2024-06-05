Hi maintainers! I believe `DevToolsPlugin` was added within the same release as this PR, so I don’t think a migration guide is needed.

`DevToolsPlugin` is no longer included in `MinimalPlugins`, so you will need to remove it manually.

```rust
// Before
App::new()
    .add_plugins(MinimalPlugins)
    .run();

// After
App::new()
    .add_plugins(MinimalPlugins)
    .add_plugins(DevToolsPlugin)
    .run();
```

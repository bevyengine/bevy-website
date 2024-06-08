Rust prides itself on its error handling, and Bevy has been steadily catching up. Previously, when checking if an asset was loaded using `AssetServer::load_state` (and variants), the only information returned on an error was the empty `LoadState::Failed`. Not very useful for debugging!

Now, a full `AssetLoadError` is included inside `Failed` to tell you exactly what went wrong. You may need to update your `match` and `if let` statements to handle this new value:

```rust
// Before
match asset_server.load_state(asset_id) {
    // ...
    LoadState::Failed => eprintln!("Could not load asset!"),
}

// After
match asset_server.load_state(asset_id) {
    // ...
    LoadState::Failed(error) => eprintln!("Could not load asset! Error: {}", error),
}
```

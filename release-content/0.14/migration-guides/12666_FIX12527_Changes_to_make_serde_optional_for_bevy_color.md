`serde` is now an optional dependency of `bevy_color`. If you need color types to implement `Serialize` and `Deserialize`, add the `serialize` feature to `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "0.14", features = ["serialize"] }
```

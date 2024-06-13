`bevy::diagnostic` depends on the `sysinfo` to track CPU and memory usage, but compiling and polling system information can be very slow. `sysinfo` is now behind the `sysinfo_plugin` feature flag, which is enabled by default for `bevy` for _not_ for `bevy_diagnostic`.

If you depend on `bevy_diagnostic` directly, toggle the flag in `Cargo.toml`:

```toml
[dependencies]
bevy_diagnostic = { version = "0.14", features = ["sysinfo_plugin"] }
```

If you set `default-features = false` for `bevy`, do the same in `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "0.14", default-features = false, features = ["sysinfo_plugin"] }
```

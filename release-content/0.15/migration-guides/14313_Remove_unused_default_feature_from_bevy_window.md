`bevy_window` had an empty default feature flag that did not do anything, so it was removed. You may have to remove any references to it if you specified it manually.

```toml
# 0.14
[dependencies]
bevy_window = { version = "0.14", default-features = false, features = ["default"] }

# 0.15
[dependencies]
bevy_window = { version = "0.15", default-features = false }
```

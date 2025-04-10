The main `bevy` crate now officially supports `no_std`. As part of this change, some functionality that used to always be included in `bevy` is now behind feature flags. The features of note are:

- `default_no_std`
- `bevy_log`
- `bevy_input_focus`
- `async_executor`
- `std`
- `critical-section`
- `libm`

Additionally, if you depend on `bevy_reflect` directly, its `bevy` feature flag has been split into two separate flags: `smallvec` and `smol_str` for their corresponding types.

#### For application authors

If your application has default features enabled, congratulations! You don't need to do anything extra! If your application has `default-features = false`, however, you may need to enabled the `std` and `async_executor` features:

```toml
# 0.15
[dependencies]
bevy = { version = "0.15", default-features = false }

# 0.16
[dependencies]
bevy = { version = "0.16", default-features = false, features = ["std", "async_executor"] }
```

#### For library authors

It is recommended for libraries to depend on Bevy with `default-features = false` to give developers more control over what features are enabled. Here are some recommended features that a library crate may want to expose:

```toml
[features]
# Most users will be on a platform which has `std` and can use the more-powerful `async_executor`.
default = ["std", "async_executor"]

# Features for typical platforms.
std = ["bevy/std"]
async_executor = ["bevy/async_executor"]

# Features for `no_std` platforms.
libm = ["bevy/libm"]
critical-section = ["bevy/critical-section"]

[dependencies]
# We disable default features to ensure we don't accidentally enable `std` on `no_std` targets, for
# example. 
bevy = { version = "0.16", default-features = false }
```

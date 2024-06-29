The default implementation of `System::run` will now always immediately run `System::apply_deferred`. If you were manually calling `System::apply_deferred` in this situation, you may remove it. Please note that `System::run_unsafe` still _does not_ call `apply_deferred` because it cannot guarantee it will be safe.

```rust
// 0.13
system.run(world);

// Sometime later:
system.apply_deferred(world);

// 0.14
system.run(world);

// `apply_deferred` no longer needs to be called!
```

External types are no longer registered into the type registry automatically unless they are used by other Bevy types (due to the new recursive registration). If you were depending on types from `std`, `glam`, or similar types being in the type registry you may need to manually register them.

```rust
App::new().register_type::<DMat3>();
```

`Access::grow` from `bevy::ecs::query` has been removed. Many operations now automatically grow the capacity.

```rust
// Before
let mut access = Access::new();
access.grow(1);
// Other operations...

// After
let mut access = Access::new();
// Other operations...
```

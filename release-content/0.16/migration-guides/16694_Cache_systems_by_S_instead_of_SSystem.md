As part of a bug fix for system caching, the `CachedSystemId` resource has been changed to store an `Entity` instead of a `SystemId`. `CachedSystemId` construction has also been changed to use the `new()` method.

```rust
// 0.15
let cached_id = CachedSystemId::<S::System>::(id);
assert!(id == cached_id.0);

// 0.16
let cached_id = CachedSystemId::<S>::new(id);
// You can convert a valid `Entity` into a `Systemid` with `SystemId::from_entity()`.
assert!(id == SystemId::from_entity(cached_id.entity));
```

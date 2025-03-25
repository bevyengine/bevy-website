The `CachedSystemId` resource has been changed:

```rust
// Before:
let cached_id = CachedSystemId::<S::System>(id);
assert!(id == cached_id.0);

// After:
let cached_id = CachedSystemId::<S>::new(id);
assert!(id == SystemId::from_entity(cached_id.entity));
```

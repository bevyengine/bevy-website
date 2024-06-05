Converting from a `Handle` to an `AssetId` using `Into` was removed because it was a footgun that could potentially drop the asset if the `Handle` was a strong reference. If you need the `AssetId`, please use `Handle::id()` instead.

```rust
// Before
let id: AssetId<T> = handle.into();

// After
let id = handle.id();
```

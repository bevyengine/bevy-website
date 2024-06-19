Converting from a `Handle` to an `AssetId` using `Into` was removed because it was a footgun that could potentially drop the asset if the `Handle` was a strong reference. If you need the `AssetId`, please use `Handle::id()` instead.

```rust
// 0.13
let id: AssetId<T> = handle.into();

// 0.14
let id = handle.id();
```

Added [AssetLoadError](https://docs.rs/bevy/latest/bevy/asset/enum.AssetLoadError.html) to the [LoadState::Failed](https://docs.rs/bevy/latest/bevy/asset/enum.LoadState.html) variant.

If you're comparing for equality with `LoadState::Failed`, you'll need to use pattern matching instead.

```rust
// 0.13
if state == LoadState::Failed {}
// 0.14
if matches!(state, LoadState::Failed(_)) {}
```

Removed `Copy`, `Ord` and `PartialOrd` implementations for [LoadState](https://docs.rs/bevy/latest/bevy/asset/enum.LoadState.html) enum

Added `Eq` and `PartialEq` implementations for [MissingAssetSourceError](https://docs.rs/bevy/latest/bevy/asset/io/struct.MissingAssetSourceError.html), [MissingProcessedAssetReaderError](https://docs.rs/bevy/latest/bevy/asset/io/struct.MissingProcessedAssetReaderError.html), [DeserializeMetaError](https://docs.rs/bevy/latest/bevy/asset/enum.DeserializeMetaError.html), [LoadState](https://docs.rs/bevy/latest/bevy/asset/enum.LoadState.html), [AssetLoadError](https://docs.rs/bevy/latest/bevy/asset/enum.AssetLoadError.html), [MissingAssetLoaderForTypeNameError](https://docs.rs/bevy/latest/bevy/asset/struct.MissingAssetLoaderForTypeNameError.html) and [MissingAssetLoaderForTypeIdError](https://docs.rs/bevy/latest/bevy/asset/struct.MissingAssetLoaderForTypeIdError.html)

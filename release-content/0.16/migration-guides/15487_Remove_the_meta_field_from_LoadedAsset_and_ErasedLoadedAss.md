`LoadedAsset` used to have a `meta` field for storing metadata. This field was unused and inaccessible, however, so in 0.16 it has been removed. Due to this change, several method signatures have also changed:

- `ErasedAssetLoader::load()` now takes `meta: &(dyn AssetMetaDyn + 'static)` instead of a `Box<dyn AssetMetaDyn>`.
- `LoadedAsset::new_with_dependencies()` no longer requires a `meta` argument.
- `LoadContext::finish()` no longer requires a `meta` argument.

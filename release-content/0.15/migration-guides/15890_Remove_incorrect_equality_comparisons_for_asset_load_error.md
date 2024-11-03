The types `bevy_asset::AssetLoadError` and `bevy_asset::LoadState` no longer support equality comparisons. If you need to check for an asset’s load state, consider checking for a specific variant using `LoadState::is_loaded` or the `matches!` macro. Similarly, consider using the `matches!` macro to check for specific variants of the `AssetLoadError` type if you need to inspect the value of an asset load error in your code.

`DependencyLoadState` and `RecursiveDependencyLoadState` are not released yet, so no migration needed,

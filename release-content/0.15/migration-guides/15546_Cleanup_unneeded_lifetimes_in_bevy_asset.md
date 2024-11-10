The traits `AssetLoader`, `AssetSaver` and `Process` traits from `bevy_asset` now use elided lifetimes. If you implement these then remove the named lifetime.

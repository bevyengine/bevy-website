The trait method `bevy_asset::io::AssetReader::read` (and `read_meta`) now return an opaque type instead of a boxed trait object. Implementors of these methods should change the type signatures appropriately

```rust
impl AssetReader for MyReader {
    // Before
    async fn read<'a>(&'a self, path: &'a Path) -> Result<Box<Reader<'a>>, AssetReaderError> {
        let reader = // construct a reader
        Box::new(reader) as Box<Reader<'a>>
    }

    // After
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        // create a reader
    }
}
```

`bevy::asset::io::Reader` is now a trait, rather than a type alias for a trait object. Implementors of `AssetLoader::load` will need to adjust the method signature accordingly

```rust
impl AssetLoader for MyLoader {
    async fn load<'a>(
        &'a self,
        // Before:
        reader: &'a mut bevy::asset::io::Reader,
        // After:
        reader: &'a mut dyn bevy::asset::io::Reader,
        _: &'a Self::Settings,
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
}
```

Additionally, implementors of `AssetReader` that return a type implementing `futures_io::AsyncRead` and `AsyncSeek` might need to explicitly implement `bevy::asset::io::Reader` for that type.

```rust
impl bevy::asset::io::Reader for MyAsyncReadAndSeek {}
```

In Rust 1.75, [`async fn` was stabilized for traits](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html#async-fn-and-return-position-impl-trait-in-traits). Some traits have been switched from returning `BoxedFuture` to be an `async fn`, specifically:

- `AssetReader`
- `AssetWriter`
- `AssetLoader`
- `AssetSaver`
- `Process`

Please update your trait implementations:

```rust
// 0.13
impl AssetLoader for MyAssetLoader {
    // ...

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        // Note that you had to pin the future.
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            Ok(bytes)
        })
    }
}

// 0.14
impl AssetLoader for MyAssetLoader {
    // ...

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        // No more need to pin the future, just write it!
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(bytes)
    }
}
```

Because these traits now use `async`, they are no longer object safe. If you need to receive or store `&dyn Trait`, use the `&dyn ErasedTrait` variant instead. For instance:

```rust
// 0.13
struct MyReader(Box<dyn AssetReader>);

// 0.14
struct MyReader(Box<dyn ErasedAssetReader>);
```

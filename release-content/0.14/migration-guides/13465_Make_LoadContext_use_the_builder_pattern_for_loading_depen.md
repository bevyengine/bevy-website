- Several LoadContext method calls will need to be updated:
  - `load_context.load_with_settings(path, settings)` => `load_context.loader().with_settings(settings).load(path)`
  - `load_context.load_untyped(path)` => `load_context.loader().untyped().load(path)`
  - `load_context.load_direct(path)` => `load_context.loader().direct().load(path)`
  - `load_context.load_direct_untyped(path)` => `load_context.loader().direct().untyped().load(path)`
  - `load_context.load_direct_with_settings(path, settings)` => `load_context.loader().with_settings(settings).direct().load(path)`
  - `load_context.load_direct_with_reader(reader, path)` => `load_context.loader().direct().with_reader(reader).load(path)`
  - `load_context.load_direct_with_reader_and_settings(reader, path, settings)` => `load_context.loader().with_settings(settings).direct().with_reader(reader).load(path)`
  - `load_context.load_direct_untyped_with_reader(reader, path)` => `load_context.loader().direct().with_reader(reader).untyped().load(path)`

---

CC @alice-i-cecile / @bushrat011899 

Examples:

```rust
load_context.loader()
    .with_asset_type::<A>()
    .with_asset_type_id(TypeId::of::<A>())
    .with_settings(|mut settings| { settings.key = value; })
    // Then, for a Handle<A>:
    .load::<A>()
    // Or, for a Handle<LoadedUntypedAsset>:
    .untyped()
    .load()
    // Or, to load an `A` directly:
    .direct()
    .load::<A>()
    .await
    // Or, to load an `ErasedLoadedAsset` directly:
    .direct()
    .untyped()
    .load()
    .await
```

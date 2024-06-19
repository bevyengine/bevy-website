<!-- Note: This is the migration guide for both #13465 AND #13415. -->

`LoadContext`, used by `AssetLoader`, has been updated so all of its `load_*` methods have been merged into a builder struct.

```rust
// 0.13
load_context.load_direct(path);
// 0.14
load_context.loader().direct().untyped().load(path);

// 0.13
load_context.load_direct_with_reader(reader, path);
// 0.14
load_context.loader().direct().with_reader(reader).untyped().load(path);

// 0.13
load_context.load_untyped(path);
// 0.14
load_context.loader().untyped().load(path);

// 0.13
load_context.load_with_settings(path, settings);
// 0.14
load_context.loader().with_settings(settings).load(path);
```

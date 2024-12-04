In areas where these implementations where being used, you can now add `from_static` in order to get the original specialised implementation which avoids creating an `Arc` internally.

```rust
// Before
let asset_path = AssetPath::from("my/path/to/an/asset.ext");

// After
let asset_path = AssetPath::from_static("my/path/to/an/asset.ext");
```

To be clear, this is only required if you wish to maintain the performance benefit that came with the specialisation. Existing code is _not_ broken by this change.

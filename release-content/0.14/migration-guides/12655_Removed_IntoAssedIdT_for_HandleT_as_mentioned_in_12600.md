If you use passing Handle by value as AssetId, you should pass reference or call .id() method on it
Before (0.13):
`assets.insert(handle, value);`
After (0.14):
`assets.insert(&handle, value);`
or
`assets.insert(handle.id(), value);`

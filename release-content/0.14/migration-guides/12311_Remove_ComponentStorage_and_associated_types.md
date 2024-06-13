The `Component::Storage` associated type has been replaced with the associated constant `STORAGE_TYPE`, making the `ComponentStorage` trait unnecessary. If you were manually implementing `Component` instead of using the derive macro, update your definitions:

```rust
// 0.13
impl Component for MyComponent {
    type Storage = TableStorage;
}

// 0.14
impl Component for MyComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    // ...
}
```

|Before|After|
|-|-|
|`TableStorage`|`StorageType::Table`|
|`SparseStorage`|`StorageType::SparseSet`|

`Component` is also now no longer object safe. If you were using `dyn Component`, please consider [filing an issue](https://github.com/bevyengine/bevy/issues) describing your use-case.

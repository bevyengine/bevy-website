If you were manually implementing `Component` instead of using the derive macro, replace the associated `Storage` associated type with the `STORAGE_TYPE` const:

```rust
// in Bevy 0.13
impl Component for MyComponent {
    type Storage = TableStorage;
}
// in Bevy 0.14
impl Component for MyComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}
```

Component is no longer object safe. If you were relying on `&dyn Component`, `Box<dyn Component>`, etc. please [file an issue ](https://github.com/bevyengine/bevy/issues) to get [this change](https://github.com/bevyengine/bevy/pull/12311) reverted.

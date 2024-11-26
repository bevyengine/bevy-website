- If you are implementing `EntityMapper` yourself, you can use the below as a stub implementation:

```rust
fn mappings(&self) -> impl Iterator<Item = (Entity, Entity)> {
    unimplemented!()
}
```

- If you were using `EntityMapper` as a trait object (`dyn EntityMapper`), instead use `dyn DynEntityMapper` and its associated methods.

As part of improvements to the bundle spawning API, the `DynamicBundle` trait now has a new `Effect` associated type. If you manually implemented `DynamicBundle`, you likely want to set `Effect = ()`, which retains the same behavior as 0.15 bundles:

```rust
// 0.15
impl DynamicBundle for MyBundle {
    // ...
}

// 0.16
impl DynamicBundle for MyBundle {
    type Effect = ();

    // ...
}
```

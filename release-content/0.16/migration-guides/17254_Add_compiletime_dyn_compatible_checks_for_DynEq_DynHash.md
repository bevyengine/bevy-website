`assert_object_safe` is no longer exported by `bevy_utils`. Instead, you can write a compile time check that your trait is “dyn compatible”:

```rust
/// Assert MyTrait is dyn compatible
const _: Option<Box<dyn MyTrait>> = None;
```

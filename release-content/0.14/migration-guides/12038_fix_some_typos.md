In `crates/bevy_mikktspace/src/generated.rs` 

```rs
// before
pub struct SGroup {
    pub iVertexRepresentitive: i32,
    ..
}

// after
pub struct SGroup {
    pub iVertexRepresentative: i32,
    ..
}
```

In `crates/bevy_core_pipeline/src/core_2d/mod.rs`

```rs
// before
Node2D::ConstrastAdaptiveSharpening

// after
Node2D::ContrastAdaptiveSharpening
```

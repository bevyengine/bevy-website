Closures passed as arguments to `par_chunk_map()`, `par_splat_map()`, `par_chunk_map_mut()`, and `par_splat_map_mut()` now take an additional index argument specifying which part of the slice is being processed.

```rust
// 0.13
items.par_chunk_map(&task_pool, 100, |chunk| {
    // ...
});

// 0.14
items.par_chunk_map(&task_pool, 100, |_index, chunk| {
    // ...
});
```

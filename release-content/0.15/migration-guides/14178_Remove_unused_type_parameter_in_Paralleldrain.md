The type parameter of `Parallel::drain()` was unused, so it is now removed. If you were manually specifying it, you can remove the bounds.

```rust
// 0.14
// Create a `Parallel` and give it a value.
let mut parallel: Parallel<Vec<u8>> = Parallel::default();
*parallel.borrow_local_mut() = vec![1, 2, 3];

for v in parallel.drain::<u8>() {
    // ...
}

// 0.15
let mut parallel: Parallel<Vec<u8>> = Parallel::default();
*parallel.borrow_local_mut() = vec![1, 2, 3];

// Remove the type parameter.
for v in parallel.drain() {
    // ...
}
```

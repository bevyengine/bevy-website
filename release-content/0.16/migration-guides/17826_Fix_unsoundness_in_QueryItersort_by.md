The `sort()` family of methods on `QueryIter` unsoundly gave access `L::Item<'w>` with the full world `'w` lifetime, meaning it was possible to smuggle items out of the compare closure. This has been fixed by shortening the lifetime so that items cannot escape the closure on the following methods on `QueryIter` and `QueryManyIter`:

- `sort()`
- `sort_unstable()`
- `sort_by()`
- `sort_unstable_by()`
- `sort_by_key()`
- `sort_unstable_by_key()`
- `sort_by_cached_key()`

This fix may cause your code to get lifetimes errors, such as:

```
error: implementation of `FnMut` is not general enough
```

To fix this, you will need to make the comparer generic over the new lifetime. Often this can be done by replacing named `'w` with `'_`, or by replacing the use of a function item with a closure:

```rust
// 0.15
query.iter().sort_by::<&C>(Ord::cmp);

// 0.16
query.iter().sort_by::<&C>(|l, r| Ord::cmp(l, r));
```

```rust
// 0.15
fn comparer(left: &&'w C, right: &&'w C) -> Ordering {
    // ...
}

query.iter().sort_by::<&C>(comparer);

// 0.16
fn comparer(left: &&C, right: &&C) -> Ordering {
    // ...
}

query.iter().sort_by::<&C>(comparer);
```

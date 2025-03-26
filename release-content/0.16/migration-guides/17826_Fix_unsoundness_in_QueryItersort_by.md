The `sort` family of methods on `QueryIter` unsoundly gave access `L::Item<'w>` with the full `'w` lifetime.  It has been shortened to `L::Item<'w>` so that items cannot escape the comparer.  If you get lifetime errors using these methods, you will need to make the comparer generic in the new lifetime.  Often this can be done by replacing named `'w` with `'_`, or by replacing the use of a function item with a closure.

```rust
// Before: Now fails with "error: implementation of `FnMut` is not general enough"
query.iter().sort_by::<&C>(Ord::cmp);
// After: Wrap in a closure
query.iter().sort_by::<&C>(|l, r| Ord::cmp(l, r));

query.iter().sort_by::<&C>(comparer);
// Before: Uses specific `'w` lifetime from some outer scope
// now fails with "error: implementation of `FnMut` is not general enough"
fn comparer(left: &&'w C, right: &&'w C) -> Ordering { /* ... */ }
// After: Accepts any lifetime using inferred lifetime parameter
fn comparer(left: &&C, right: &&C) -> Ordering { /* ... */ }
```

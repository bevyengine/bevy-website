There was a lifetime issue found with `QueryLens::query()` where calling `get_inner()` on the returned value would allow for multiple mutable references to the same entity. This has been fixed by shrinking the lifetime of `QueryLens::query()`'s result, however it may break existing code.

If you run into lifetime issues while calling `get_inner()` or `iter_inner()` on `QueryLens::query()`'s result, you may need to switch to the new `QueryLens::query_inner()` method that only works on immutable queries.

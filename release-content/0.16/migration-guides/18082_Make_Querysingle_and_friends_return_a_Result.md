`Query::single`, `Query::single_mut` and their `QueryState` equivalents now return a `Result`. Generally, you’ll want to:

- Use Bevy 0.16’s system error handling to return a `Result` using the `?` operator.
- Use a `let else Ok(data)` block to early return if it’s an expected failure.
- Use `unwrap()` or `Ok` destructuring inside of tests.

The old `Query::get_single` (etc) methods which did this have been deprecated.

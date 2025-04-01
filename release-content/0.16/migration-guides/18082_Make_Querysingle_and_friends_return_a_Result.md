`Query::single`, `Query::single_mut` and their `QueryState` equivalents now return a `Result`. Generally, you’ll want to:

- Use Bevy 0.16’s system error handling to return a `Result` using the `?` operator.
- Use a `let else Ok(data)` block to early return if it’s an expected failure.
- Use `unwrap()` or `Ok` destructuring inside of tests.

The old `Query::get_single` (etc) methods which did this have been deprecated.

If you are using `anyhow`, you will experience namespace clashes between Bevy's catch-all `Result` and `anyhow`'s type of the same name.
Within Bevy-specific projects, you should migrate to use the `bevy::ecs::error::Result` due to its better backtraces, and file requests for any missing functionality.
For projects that support both Bevy and non-Bevy users, you should define a feature-gated type alias and avoid glob-importing `bevy::prelude`:

```rust
#[cfg(feature="bevy")]
type Result = bevy::ecs::error::Result;

#[cfg(not(feature="bevy"))]
type Result = anyhow::Result;
```

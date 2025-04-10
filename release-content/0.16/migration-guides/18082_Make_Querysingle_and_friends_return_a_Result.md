`Query::single()`, `Query::single_mut()` and their `QueryState` equivalents now return a `Result`. Generally, you'll want to:

- Use Bevy 0.16's system error handling to return a `Result` using the `?` operator.
- Use a `let Ok(data) = result else {}` block to early return if there's an expected failure.
- Use `unwrap()` or `Ok` destructuring inside of tests.

The old `Query::get_single()` and related methods have been deprecated.

If you are using `anyhow`, you will experience namespace clashes between Bevy's catch-all `Result` and `anyhow::Result`. Within Bevy-specific projects, you should migrate to use the new `bevy::ecs::error::Result` due to its improved backtraces. (If you discover missing functionality, please feel free to open a pull request adding it!) For projects that support both Bevy and non-Bevy users, you should define a feature-gated type alias and avoid glob-importing `bevy::prelude`:

```rust
#[cfg(feature = "bevy")]
type Result = bevy::ecs::error::Result;

#[cfg(not(feature = "bevy"))]
type Result = anyhow::Result;
```

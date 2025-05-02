**Bevy 0.15** introduces a convenient new "cached" API for running one-shot systems:

```rust
// Old, uncached API:
let foo_id = commands.register_system(foo);
commands.run_system(foo_id);

// New, cached API:
commands.run_system_cached(foo);
```

This allows you to call `register_system_cached` without needing to worry about producing duplicate systems.

```rust
// Uncached API:
let id1 = world.register_system(quux);
let id2 = world.register_system(quux);
assert!(id1 != id2);

// Cached API:
let id1 = world.register_system_cached(quux);
let id2 = world.register_system_cached(quux);
assert!(id1 == id2);
```

### Comparison to `run_system_once`

`run_system_once` sets up a system, runs it once, and tears it down. This means system parameters like `Local` and `EventReader` that rely on persistent state between runs will be lost. Any system parameters like `Query` that rely on cached computations to improve performance will have to rebuild their cache each time, which can be costly. As a consequence, `run_system_once` is only recommended for diagnostic use (e.g. unit tests), and `run_system` or `run_system_cached` should be preferred for "real" code.

### Limitations

With the cached API, different systems cannot be cached under the same `CachedSystemId<S>`. There can be no more than one distinct system of type `S`. This is true when `size_of::<S>() == 0`, which is almost always true in practice. To enforce correctness, the new API will give you a compile-time error if you try to use a non-zero-sized function (like a function pointer or a capturing closure).

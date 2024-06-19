`WorldCell` has been removed due to its incomplete, less performant, and potentially confusing nature. If you were using it to fetch multiple distinct values, consider using a `SystemState` instead with the `SystemState::get()` method.

If `SystemState` does not fit your use-case and unsafe is tolerable, you can use `UnsafeWorldCell`. It is more performant and featureful, but lacks the runtime checks.

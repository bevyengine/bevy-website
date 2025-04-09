As a result of changes to [Never type fallback] in Rust 2024, closures which panic now have an inferred return type of `!`, rather than `()`. This might occur if you're using `panic!`, `todo!`, `unimplemented!`, `unreachable!` or other means of panicking.

This affects Bevy users as our blanket implementations for `Command`, `IntoSystem` and `IntoObserverSystem`, which operate on all matching functions with the correct type signature, no longer cover closures of this sort.

While this may be fixable on Bevy's end (see [#18778]), for now, you can work around this issue by either:

1. Converting your closures to ordinary named functions, whose return type is not inferred.
2. Explicitly specifying the return type of your closure as `()`.

[Never type fallback]: https://doc.rust-lang.org/edition-guide/rust-2024/never-type-fallback.html
[#18778]: https://github.com/bevyengine/bevy/issues/18778

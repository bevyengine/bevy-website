As part of Bevy's migration to [Rust 2024], the lifetimes of several functions that use return-position impl-trait (RPIT) syntax may have been changed to be slightly more conservative. If you run into lifetime issues with functions that return `impl Trait`, please [create a new issue].

[Rust 2024]: https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html#rust-2024
[create a new issue]: https://github.com/bevyengine/bevy/issues

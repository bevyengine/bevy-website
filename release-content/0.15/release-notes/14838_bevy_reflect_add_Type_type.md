Rust's [`TypeId`] is a unique identifier for a type, making it a perfect candidate for use as a
key in mappings and for checking whether two types are the same at runtime.
And since it's essentially just two `u64` values, it's extremely cheap to copy, compare, and hash.

One of the downsides to using `TypeId`, though, is that it doesn't contain any other information
about the type, including its name. This can make debugging somewhat frustrating as you can't
easily tell which type a `TypeId` corresponds to.

Since [`bevy_reflect`] makes heavy use of `TypeId`, 0.15 introduces a new type to help alleviate
the debugging issue while still maintaining the benefits of `TypeId`: [`Type`].

[`Type`] is a simple wrapper around `TypeId` that also stores the [`TypePathTable`].
Like `TypeId` it's `Copy`, `Eq`, and `Hash`, delegating to the underlying `TypeId` for the latter
two. But unlike `TypeId`, its `Debug` implementation will print the [type path] of the type it represents.
This debuggability comes at the cost of an extra 32 bytes, but may often be well worth it,
especially if that data would have been stored elsewhere anyway.

It can be constructed from any type that implements [`TypePath`]:

```rust
let ty = Type::of::<String>();

let mut map = HashMap::<Type, i32>::new();
map.insert(ty, 25);

let debug = format!("{:?}", map);
assert_eq!(debug, "{alloc::string::String: 25}");
```

[`TypeId`]: https://doc.rust-lang.org/std/any/struct.TypeId.html
[`bevy_reflect`]: https://docs.rs/bevy_reflect/0.15/
[`Type`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/struct.Type.html
[`TypePathTable`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/struct.TypePathTable.html
[type path]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.TypePath.html#tymethod.type_path
[`TypePath`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.TypePath.html
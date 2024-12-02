The [`bevy_reflect`] crate relies on types implementing [`Reflect`] in order to make them reflectable.
Fields of structs and enums that don't implement `Reflect` must be specifically ignored with `#[reflect(ignore)]`.
And due to Rust's [orphan rule], this is often the case for types not owned by the current crate.

Following [`serde`'s example](https://serde.rs/remote-derive.html), Bevy 0.15 introduces a way to reflect remote types
using a new [`#[reflect_remote(...)]`][reflect_remote] attribute macro.
This allows users to define a model for reflection to base its behavior on,
while still operating with the actual type.

```rust
// Pretend this type is defined in a crate called `external_crate`
#[derive(Default)]
struct Name {
    pub value: String,
}

// We can define our model, including other derives and reflection attributes
#[reflect_remote(external_crate::Name)]
#[derive(Default)]
#[reflect(Default)]
struct NameWrapper {
    pub value: String,
}

// Now we can use `Name` as a field in a reflected type without having to ignore it
#[derive(Reflect)]
struct Player {
    #[reflect(remote = NameWrapper)]
    name: external_crate::Name,
}
```

Under the hood, this works by transforming our model into a transparent wrapper around the actual type:

```rust
#[repr(transparent)]
struct NameWrapper(pub external_crate::Name);
```

The macro then uses the model to generate all the reflection trait implementations,
driven by a new [`ReflectRemote`] trait for swapping between our wrapper and the remote type.
Compile-time assertions are also generated to help ensure the model and the actual type stay in sync.

While this feature has many aspects complete, including generic support, enum support, and nesting,
there are still some limitations we hope to address in future releases,
including support for reflecting a remote type with private fields.

[`bevy_reflect`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/
[`Reflect`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.Reflect.html
[orphan rule]: https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type
[reflect_remote]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/attr.reflect_remote.html
[`ReflectRemote`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.ReflectRemote.html

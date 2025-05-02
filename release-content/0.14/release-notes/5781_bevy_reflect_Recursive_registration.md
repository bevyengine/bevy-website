Bevy uses [reflection](https://docs.rs/bevy_reflect/latest/bevy_reflect/) in order to dynamically process data for things like serialization and deserialization.
A Bevy app has a `TypeRegistry` to keep track of which types exist.
Users can register their custom types when initializing the app or plugin.

```rust
#[derive(Reflect)]
struct Data<T> {
  value: T,
}

#[derive(Reflect)]
struct Blob {
  contents: Vec<u8>,
}

app
  .register_type::<Data<Blob>>()
  .register_type::<Blob>()
  .register_type::<Vec<u8>>()
```

In the code above, `Data<Blob>` depends on `Blob` which depends on `Vec<u8>`,
which means that all three types need to be manually registered—
even if we only care about `Data<Blob>`.

This is both tedious and error-prone, especially when these type dependencies are only
used in the context of other types (i.e. they aren't used as standalone types).

In 0.14, any type that derives `Reflect` will automatically register all of its type dependencies.
So when we register `Data<Blob>`, `Blob` will be registered as well (which will register `Vec<u8>`),
thus simplifying our registration down to a single line:

```rust
app.register_type::<Data<Blob>>()
```

Note that removing the registration for `Data<Blob>` now also means that `Blob` and `Vec<u8>` may
not be registered either, unless they were registered some other way.
If those types are needed as standalone types, they should be registered separately.

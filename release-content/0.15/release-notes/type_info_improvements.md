With [`bevy_reflect`], compile-time type information can be retrieved from a reflected type as [`TypeInfo`].

Bevy 0.15 adds many improvements and convenience methods for working with `TypeInfo`.

#### Generic Parameter Info

The first addition is the ability to get information about a type's generic parameters.
This not includes the parameter's type, but also its name and—if it's a const parameter—its default value.

```rust
#[derive(Reflect)]
struct MyStruct<T>(T);

let generics = MyStruct::<f32>::type_info().generics();

let t = generics.get(0).unwrap();
assert_eq!(t.name(), "T");
assert!(t.ty().is::<f32>());
assert!(!t.is_const());
```

#### Nested `TypeInfo`

Pretty much every type in Rust is made up of other types.
Structs, maps, lists—they all contain other types.

In previous versions of Bevy, `TypeInfo` granted you limited access to type information of
these nested types. It mostly just provided the type's [`TypeId`] and [`TypePath`].

However, in Bevy 0.15, you can now directly access the `TypeInfo` of these nested types.

```rust
#[derive(Reflect)]
struct Row {
  id: usize
}

let struct_info: StructInfo = Row::type_info().as_struct();

let field: NamedField = struct_info.field("id").unwrap();

// `NamedField` now exposes a way to fetch the `TypeInfo` of the field's type
let field_info: TypeInfo = field.type_info().unwrap();
assert!(field_info.is::<usize>());
```

#### `TypeInfo` Convenience Casts

In most cases, `TypeInfo` needs to first be pattern matched to the correct variant in order to gain full access
to the type's compile-time information.
This can be mildly annoying when you already know the variant ahead of time.
This often occurs when writing tests, but also shows up when trying to get the type's [`ReflectRef`] data
along with its `TypeInfo`. It tends to looks something like:

```rust
// We have to pattern match on `ReflectRef`...
let ReflectRef::List(list) = reflected_value.reflect_ref() else {
    panic!("expected a list");
};

// ...and still need to pattern match on `TypeInfo`
let TypeInfo::List(list_info) = reflected_value.get_represented_type_info().unwrap() else {
    panic!("expected a list info");
};
```

In such cases, the variant is already verified via the `ReflectRef` but the
`TypeInfo` must still be pattern matched regardless.

In Bevy 0.15, convenience methods have been added to `TypeInfo`, `ReflectRef`, [`ReflectMut`], and [`ReflectOwned`]
to conveniently cast to the expected variant or return an error upon failure.

```rust
// We can simply verify the kind of our reflected value once...
let ReflectRef::List(list) = reflected_value.reflect_ref() else {
    panic!("expected a list");
};

// ...and just assert the `TypeInfo`
let list_info = reflected_value.get_represented_type_info().unwrap().as_list().unwrap();
```

If the `.as_list()` cast fails in the snippet above, it will return an error detailing what [kind]
we expected (i.e. `List`) and what we actually got (e.g. `Array`, `Struct`, etc.).

And this works in the opposite direction as well:

```rust
let TypeInfo::List(list_info) = reflected_value.get_represented_type_info().unwrap() else {
    panic!("expected a list info");
};

let list = reflected_value.reflect_ref().as_list().unwrap();
```

[`bevy_reflect`]: https://docs.rs/bevy_reflect/0.15/
[`TypeInfo`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/enum.TypeInfo.html
[`ReflectRef`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/enum.ReflectRef.html
[`ReflectMut`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/enum.ReflectMut.html
[`ReflectOwned`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/enum.ReflectOwned.html
[`TypeId`]: https://doc.rust-lang.org/std/any/struct.TypeId.html
[`TypePath`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.TypePath.html
[kind]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/enum.ReflectKind.html

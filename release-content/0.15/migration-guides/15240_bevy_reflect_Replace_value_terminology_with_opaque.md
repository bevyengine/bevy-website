The reflection concept of “value type” has been replaced with a clearer “opaque type”. The following renames have been made to account for this:

- `ReflectKind::Value` → `ReflectKind::Opaque`
- `ReflectRef::Value` → `ReflectRef::Opaque`
- `ReflectMut::Value` → `ReflectMut::Opaque`
- `ReflectOwned::Value` → `ReflectOwned::Opaque`
- `TypeInfo::Value` → `TypeInfo::Opaque`
- `ValueInfo` → `OpaqueInfo`
- `impl_reflect_value!` → `impl_reflect_opaque!`
- `impl_from_reflect_value!` → `impl_from_reflect_opaque!`

Additionally, declaring your own opaque types no longer uses `#[reflect_value]`. This attribute has been replaced by `#[reflect(opaque)]`:

```rust
// BEFORE
#[derive(Reflect)]
#[reflect_value(Default)]
struct MyOpaqueType(u32);

// AFTER
#[derive(Reflect)]
#[reflect(opaque)]
#[reflect(Default)]
struct MyOpaqueType(u32);
```

Note that the order in which `#[reflect(opaque)]` appears does not matter.

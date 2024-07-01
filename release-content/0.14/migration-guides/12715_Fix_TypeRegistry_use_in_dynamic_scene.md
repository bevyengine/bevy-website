`SceneSerializer` and all related serialization helpers now take `&TypeRegistry` instead of `&TypeRegistryArc`. You can access the former from the latter with `TypeRegistryArc::read()`.

Furthermore, `DynamicScene::serialize_ron()` has been renamed to `serialize()`. This has been done to highlight that this function is not about serializing into RON specifically, but rather the official Bevy scene format (`.scn` / `.scn.ron`). This leaves room to change the format in the future, if need be.

```rust
// 0.13
let world = World::new();
let scene = DynamicScene::from_world(&world);

let type_registry_arc: &TypeRegistryArc = &**world.resource::<AppTypeRegistry>();

let serialized_scene = scene.serialize_ron(type_registry_arc).unwrap();

// 0.14
let world = World::new();
let scene = DynamicScene::from_world(&world);

let type_registry_arc: &TypeRegistryArc = &**world.resource::<AppTypeRegistry>();

// We now need to retrieve the inner `TypeRegistry`.
let type_registry = type_registry_arc.read();

// `serialize_ron` has been renamed to `serialize`, and now takes a reference to `TypeRegistry`.
let serialized_scene = scene.serialize(&type_registry).unwrap();
```

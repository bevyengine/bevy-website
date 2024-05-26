- `SceneSerializer` and all related serializing helper types now take a `&TypeRegistry` instead of a `&TypeRegistryArc`. You can upgrade by getting the former from the latter with `TypeRegistryArc::read()`, _e.g._
```diff
  let registry_arc: TypeRegistryArc = [...];
- let serializer = SceneSerializer(&scene, &registry_arc);
+ let registry = registry_arc.read();
+ let serializer = SceneSerializer(&scene, &registry);
```

- Rename `DynamicScene::serialize_ron()` to `serialize()`.

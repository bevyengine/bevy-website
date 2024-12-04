`Handle` can no longer be used as a `Component`. All existing Bevy types using this pattern have been wrapped in their own semantically meaningful type. You should do the same for any custom `Handle` components your project needs.

The `Handle<MeshletMesh>` component is now `MeshletMesh3d`.

The `WithMeshletMesh` type alias has been removed. Use `With<MeshletMesh3d>` instead.

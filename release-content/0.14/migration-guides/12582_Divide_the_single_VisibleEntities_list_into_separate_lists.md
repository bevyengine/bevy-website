`check_visibility` and `VisibleEntities` now store the four types of renderable entities–2D meshes, 3D meshes, lights, and UI elements–separately. If your custom rendering code examines `VisibleEntities`, it will now need to specify which type of entity it’s interested in using the `WithMesh2d`, `WithMesh`, `WithLight`, and `WithNode` types respectively. If your app introduces a new type of renderable entity, you’ll need to add an instance of the `check_visibility` system with the appropriate query filter to the main world schedule to accommodate your new component or components. For example:

```rust
app
    .add_systems(
        PostUpdate,
        check_visibility::<With<MyCustomRenderable>>
            .in_set(VisibilitySystems::CheckVisibility)
    );
```

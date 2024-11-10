Asset handles for scenes and dynamic scenes must now be wrapped in the `SceneRoot` and `DynamicSceneRoot` components. Raw handles as components no longer spawn scenes.

Additionally, `SceneBundle` and `DynamicSceneBundle` have been deprecated. Instead, use the scene components directly.

Previously:

```rust
let model_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("model.gltf"));

commands.spawn(SceneBundle {
    scene: model_scene,
    transform: Transform::from_xyz(-4.0, 0.0, -3.0),
    ..default()
});
```

Now:

```rust
let model_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("model.gltf"));

commands.spawn((
    SceneRoot(model_scene),
    Transform::from_xyz(-4.0, 0.0, -3.0),
));
```

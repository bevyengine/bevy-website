`UiPickingPlugin` and `SpritePickingPlugin` are no longer included in `DefaultPlugins`. They must be explicitly added.

`RayCastPickable` has been replaced in favor of the `MeshPickingCamera` and `Pickable` components. You should add them to cameras and entities, respectively, if you have `MeshPickingSettings::require_markers` set to `true`.

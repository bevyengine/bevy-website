All matrices have been renamed to follow the convention `x_from_y` in order to decrease confusion while increasing readability.

- `Frustum`'s `from_view_projection`, `from_view_projection_custom_far` and `from_view_projection_no_far` methods were renamed to `from_clip_from_world`, `from_clip_from_world_custom_far` and `from_clip_from_world_no_far`.
- `ComputedCameraValues::projection_matrix` was renamed to `clip_from_view`.
- `CameraProjection::get_projection_matrix` was renamed to `get_clip_from_view` (this affects implementations on `Projection`, `PerspectiveProjection` and `OrthographicProjection`).
- `ViewRangefinder3d::from_view_matrix` was renamed to `from_world_from_view`.
- `PreviousViewData`'s members were renamed to `view_from_world` and `clip_from_world`.
- `ExtractedView`'s `projection`, `transform` and `view_projection` were renamed to `clip_from_view`, `world_from_view` and `clip_from_world`.
- `ViewUniform`'s `view_proj`, `unjittered_view_proj`, `inverse_view_proj`, `view`, `inverse_view`, `projection` and `inverse_projection` were renamed to `clip_from_world`, `unjittered_clip_from_world`, `world_from_clip`, `world_from_view`, `view_from_world`, `clip_from_view` and `view_from_clip`.
- `GpuDirectionalCascade::view_projection` was renamed to `clip_from_world`.
- `MeshTransforms`' `transform` and `previous_transform` were renamed to `world_from_local` and `previous_world_from_local`.
- `MeshUniform`'s `transform`, `previous_transform`, `inverse_transpose_model_a` and `inverse_transpose_model_b` were renamed to `world_from_local`, `previous_world_from_local`, `local_from_world_transpose_a` and `local_from_world_transpose_b` (the `Mesh` type in WGSL mirrors this, however `transform` and `previous_transform` were named `model` and `previous_model`).
- `Mesh2dTransforms::transform` was renamed to `world_from_local`.
- `Mesh2dUniform`'s `transform`, `inverse_transpose_model_a` and `inverse_transpose_model_b` were renamed to `world_from_local`, `local_from_world_transpose_a` and `local_from_world_transpose_b` (the `Mesh2d` type in WGSL mirrors this).
- In WGSL, `bevy_pbr::mesh_functions`, `get_model_matrix` and `get_previous_model_matrix` were renamed to `get_world_from_local` and `get_previous_world_from_local`.
- In WGSL, `bevy_sprite::mesh2d_functions::get_model_matrix` was renamed to `get_world_from_local`.

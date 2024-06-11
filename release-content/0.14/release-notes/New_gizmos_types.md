Gizmos in Bevy allow to easily draw arbitrary shapes to help debugging or authoring content, but also to visualize specific properties of your scene, such has the AABB of your meshes.

In 0.14, several new gizmos have been added to [`bevy::gizmos`]:

#### Grid Gizmos

New grid gizmo types were added with [`Gizmos::grid_2d`] and [`Gizmos::grid`] to draw a plane grid in either 2D or 3D, alongside [`Gizmos::grid_3d`] to draw a 3D grid.

Each grid type can be skewed, scaled and subdivided along its axis, and you can separately control which outer edges to draw.

![Grid gizmos screenshot](grid_gizmos.png)

#### Coordinate Axes Gizmo

The new [`Gizmos::axes`] add a simple way to show the position, orientation and scale of any object from its [`Transform`] plus a base size.
The size of each axis arrow is proportional to the corresponding axis scale in the provided [`Transform`].

![Axes gizmo screenshot](axes_gizmo.png)

#### Light Gizmos

The new [`ShowLightGizmo`] component implements a retained gizmo to visualize lights for [`SpotLight`], [`PointLight`] and [`DirectionalLight`].
Most light properties are visually represented by the gizmos, and the gizmo color can be set to match the light instance or use a variety of other behaviors.

Similar to other retained gizmos, [`ShowLightGizmo`] can be configured per-instance or globally with [`LightGizmoConfigGroup`].

![Light gizmos screenshot](light_gizmos.png)

[`bevy::gizmos`]: https://docs.rs/bevy/0.14.0/bevy/gizmos/index.html
[`Gizmos::grid_2d`]: https://docs.rs/bevy/0.14.0/bevy/gizmos/prelude/struct.Gizmos.html#method.grid_2d
[`Gizmos::grid`]: https://docs.rs/bevy/0.14.0/bevy/gizmos/prelude/struct.Gizmos.html#method.grid
[`Gizmos::grid_3d`]: https://docs.rs/bevy/0.14.0/bevy/gizmos/prelude/struct.Gizmos.html#method.grid_3d
[`Gizmos::axes`]: https://docs.rs/bevy/0.14.0/bevy/gizmos/prelude/struct.Gizmos.html#method.axes
[`Transform`]: https://docs.rs/bevy/0.14.0/bevy/prelude/struct.Transform.html
[`ShowLightGizmo`]: https://docs.rs/bevy/0.14.0/bevy/gizmos/light/struct.ShowLightGizmo.html
[`SpotLight`]: https://docs.rs/bevy/0.14.0/bevy/pbr/struct.SpotLight.html
[`PointLight`]: https://docs.rs/bevy/0.14.0/bevy/pbr/struct.PointLight.html
[`DirectionalLight`]: https://docs.rs/bevy/0.14.0/bevy/pbr/struct.DirectionalLight.html
[`LightGizmoConfigGroup`]: https://docs.rs/bevy/0.14.0/bevy/gizmos/light/struct.LightGizmoConfigGroup.html

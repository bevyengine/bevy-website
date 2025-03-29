In Bevy, "assets" are any heavy data that we only want to hold a single copy of, despite being used by many entities: things like sounds, images and 3D models.
Assets of type `T` are stored in the [`Assets<T>`] resource, and then each entity that wants to reference them holds a component like [`Sprite`] with a [`Handle<T>`] inside of it, identifying which asset to use for that entity.

While this works great for avoiding storing ten thousands ogre meshes in memory when you're making a tower defense,
this relatively indirect pattern makes it hard to figure out when an asset that you're relying on has changed.
That's because the [`Handle<T>`] could change, pointing to a new asset, or the underlying asset in [`Assets<T>`] could change,
modifying the underlying data.

While querying for [`Changed<Sprite>`] will catch changes to the handle, it *won't* catch changes to the underlying asset,
resulting in frustrating bugs that are hard to detect, as they only occur when things change in an unusual way.

To solve this, we've added an [`AssetChanged`] query filter, which works for any type (like [`Sprite`])
which implements the new [`AsAssetId`] trait.
Something like `Query<&mut Aabb, With<AssetChanged<Mesh3d>>>` now Just Works™️, allowing you to recompute data whenever the underlying asset is changed for any reason.

[`Assets<T>`]: https://dev-docs.bevyengine.org/bevy/asset/struct.Assets.html
[`Sprite`]: https://dev-docs.bevyengine.org/bevy/prelude/struct.Sprite.html
[`Handle<T>`]: https://dev-docs.bevyengine.org/bevy/asset/enum.Handle.html
[`AssetChanged`]: https://dev-docs.bevyengine.org/bevy/asset/prelude/struct.AssetChanged.html
[`AsAssetId`]: https://dev-docs.bevyengine.org/bevy/asset/trait.AsAssetId.html

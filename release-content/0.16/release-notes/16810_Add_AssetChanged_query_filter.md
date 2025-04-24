In Bevy, "assets" are data we want to hold a single copy of, even when used by many entities: things like sounds, images and 3D models.
Assets like `Image` are stored in the [`Assets<Image>`] resource. Entities then have components like [`Sprite`], which hold a [`Handle<Image>`] inside of it, identifying which asset to use for that entity.

While this works great for avoiding storing ten thousands ogre meshes in memory when you're making a tower defense,
this relatively indirect pattern makes it hard to figure out when an asset that you're relying on has changed.
That's because the [`Handle<T>`] could change, pointing to a new asset, or the underlying asset in [`Assets<T>`] could change,
modifying the underlying data.

While querying for `Changed<Sprite>` will catch changes to the handle, it *won't* catch changes to the underlying asset,
resulting in frustrating bugs that are hard to detect, as they only occur when things change in an unusual way.

To solve this, we've added an [`AssetChanged`] query filter, which works for any type (like [`Sprite`])
which implements the new [`AsAssetId`] trait.
Something like `Query<&mut Aabb, With<AssetChanged<Mesh3d>>>` now Just Works™, allowing you to recompute data whenever the underlying asset is changed for any reason.

[`Assets<T>`]: https://docs.rs/bevy/0.16/bevy/asset/struct.Assets.html
[`Sprite`]: https://docs.rs/bevy/0.16/bevy/prelude/struct.Sprite.html
[`Handle<T>`]: https://docs.rs/bevy/0.16/bevy/asset/enum.Handle.html
[`AssetChanged`]: https://docs.rs/bevy/0.16/bevy/asset/prelude/struct.AssetChanged.html
[`AsAssetId`]: https://docs.rs/bevy/0.16/bevy/asset/trait.AsAssetId.html

- `ImageLoader` can no longer be initialized directly through `init_asset_loader`. Now you must use `app.register_asset_loader(ImageLoader::new(supported_compressed_formats))` (check out the implementation of `bevy_render::ImagePlugin`). This only affects you if you are initializing the loader manually and does not affect users of `bevy_render::ImagePlugin`.
- The asset loader name must be updated in `.meta` files for images.
Change: `loader: "bevy_render::texture::image_loader::ImageLoader",`
to: `loader: "bevy_image::image_loader::ImageLoader",`

This will fix the following error:

> `no `AssetLoader` found with the name 'bevy_render::texture::image_loader::ImageLoader`

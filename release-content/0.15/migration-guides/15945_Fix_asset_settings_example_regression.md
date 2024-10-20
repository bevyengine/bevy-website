This PR obviously requires no migration guide as this is just a bug-fix, but I believe that #15812 should mention that meta files needs updating. Proposal:

- Asset loader name must be updated in `.meta` files for images.
Change: `loader: "bevy_render::texture::image_loader::ImageLoader",`
to: `loader: "bevy_image::image_loader::ImageLoader",`
It will fix the following error: `no `AssetLoader` found with the name 'bevy_render::texture::image_loader::ImageLoader`

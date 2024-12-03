Various types and traits are no longer re-exported from `bevy_image` in `bevy::render::texture`. Import them directly from `bevy::image` instead.

```rust
// 0.14
use bevy::render::texture::BevyDefault;
// 0.15
use bevy::image::BevyDefault;
```

This is an incomprehensive list of other types may be affected for searchability: `CompressedImageFormats`, `ExrTextureLoader`, `HdrTextureLoader`, `Image`, `ImageAddressMode`, `ImageFilterMode`, `ImageLoader`, `ImageLoaderSettings`, `ImageSampler`, `ImageSamplerDescriptor`, `ImageType`, `TextureError`, `TextureFormatPixelInfo`.

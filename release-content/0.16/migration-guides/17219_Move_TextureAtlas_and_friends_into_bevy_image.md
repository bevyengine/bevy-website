The following types have been moved from `bevy_sprite` to `bevy_image`: `TextureAtlas`, `TextureAtlasBuilder`, `TextureAtlasSources`, `TextureAtlasLayout` and `DynamicTextureAtlasBuilder`.

If you are using the `bevy` crate, and were importing these types directly (e.g. before `use bevy::sprite::TextureAtlas`), be sure to update your import paths (e.g. after `use bevy::image::TextureAtlas`)

If you are using the `bevy` prelude to import these types (e.g. `use bevy::prelude::*`), you don’t need to change anything.

If you are using the `bevy_sprite` subcrate, be sure to add `bevy_image` as a dependency if you do not already have it, and be sure to update your import paths.

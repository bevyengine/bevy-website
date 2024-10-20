Replace all uses of `SpriteBundle` with `Sprite`. There are several new convenience constructors: `Sprite::from_image`, `Sprite::from_atlas_image`, `Sprite::from_color`. 

WARNING: use of `Handle<Image>` and `TextureAtlas` as components on sprite entities will NO LONGER WORK. Use the fields on `Sprite` instead. I would have removed the `Component` impls from `TextureAtlas` and `Handle<Image>` except it is still used within ui. We should fix this moving forward with the migration.

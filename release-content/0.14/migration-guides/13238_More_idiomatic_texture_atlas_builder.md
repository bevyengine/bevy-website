```diff
- let mut texture_atlas_builder = TextureAtlasBuilder::default().padding(UVec2::default()).format(..);
+ let mut texture_atlas_builder = TextureAtlasBuilder::default();
+ texture_atlas_builder.padding(UVec2::default()).format(..);

- let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
+ let (texture_atlas_layout, texture) = texture_atlas_builder.build().unwrap();
```

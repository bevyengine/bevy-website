- `ExtractedSprite` has a new `kind: ExtractedSpriteKind` field with variants `Single` and `Slices`. 
  - `Single` represents a single sprite. `ExtractedSprite`’s `anchor`, `rect`, `scaling_mode` and `custom_size` fields have been moved into `Single`. 
  - `Slices` contains a range that indexes into a new resource `ExtractedSlices`. Slices are used to draw elements composed from multiple sprites such as text or nine-patched borders.

- `ComputedTextureSlices::extract_sprites` has been renamed to `extract_slices`. Its `transform` and `original_entity` parameters have been removed.

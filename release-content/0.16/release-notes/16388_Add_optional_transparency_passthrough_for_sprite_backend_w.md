<!-- Add optional transparency passthrough for sprite backend with bevy_picking -->
<!-- https://github.com/bevyengine/bevy/pull/16388 -->
In most cases when working on a game, you don't want clicks over the transparent part of sprites to count as a click on that sprite. This is especially true if you're working on a 2D game with a lot of overlapping sprites which have transparent regions.

In previous versions of bevy, sprite interactions were handled as simple bounding box checks. If your cursor was within the boundary of the sprite's box, it would block interactions with the sprites behind it, even if the area of the sprite is fully transparent.

In **Bevy 0.16**, when interacting with transparent sections of an entity sprite, those interactions will pass through to the entities beneath it. No changes required to your codebase!

By default passthrough will occur for any part of a sprite which has an alpha value of `0.1` or lower. If you wish to revert back to rect-checking for sprites or change the cutoff, you can do so by overwriting `SpritePickingSettings`.

```rust
// Change the alpha value cutoff to 0.2 instead of the default 0.1
app.insert_resource(SpritePickingSettings {
  picking_mode: SpritePickingMode::AlphaThreshold(0.2)
});

// Revert to Bounding Box picking mode
app.insert_resource(SpritePickingSettings {
  picking_mode: SpritePickingMode::BoundingBox
});
```

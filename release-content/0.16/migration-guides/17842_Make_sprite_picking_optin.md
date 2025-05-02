- Sprite picking are now opt-in, make sure you insert `Pickable` component when using sprite picking.

```diff
-commands.spawn(Sprite { .. } );
+commands.spawn((Sprite { .. }, Pickable::default());
```

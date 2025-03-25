If you were previously implementing `VisitEntities` or `VisitEntitiesMut` (likely via a derive), instead use `MapEntities`. Those were almost certainly used in the context of Bevy Scenes or reflection via `ReflectMapEntities`. If you have a case that uses `VisitEntities` or `VisitEntitiesMut` directly, where `MapEntities` is not a viable replacement, please let us know!

```rust
// before
#[derive(VisitEntities, VisitEntitiesMut)]
struct Inventory {
  items: Vec<Entity>,
  #[visit_entities(ignore)]
  label: String,
}

// after
#[derive(MapEntities)]
struct Inventory {
  #[entities]
  items: Vec<Entity>,
  label: String,
}
```

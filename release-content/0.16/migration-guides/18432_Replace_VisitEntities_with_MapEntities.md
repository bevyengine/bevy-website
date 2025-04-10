`VisitEntities` and `VisitEntitiesMut` have been removed in favor of `MapEntities`, as the prior is less generally applicable (doesn't work on collections like `HashSet`s). If you previously derived `VisitEntities` and family, you can now derive `MapEntities` and use the `#[entities]` attribute to annotate the list of `Entity`s.

```rust
// 0.15
#[derive(VisitEntities, VisitEntitiesMut)]
struct Inventory {
    items: Vec<Entity>,
    // Opt-out of mapping this field, as its a string.
    #[visit_entities(ignore)]
    label: String,
}

// 0.16
#[derive(MapEntities)]
struct Inventory {
    // Opt-in to mapping this field.
    #[entities]
    items: Vec<Entity>,
    label: String,
}
```

Note `Component::visit_entities()` and `Component::visit_entities_mut()` have also been removed in favor of the new `Component::map_entities()` method. When deriving `Component`, you may also use `#[entities]` to specify which `Entity`s may be mapped.

Finally, entity mapping is no longer implemented for all types that implement `IntoIterator<Item = &Entity>`. If you previously depended on a custom data type to support the `#[entities]` attribute, please manually derive / implement `MapEntities` for it.

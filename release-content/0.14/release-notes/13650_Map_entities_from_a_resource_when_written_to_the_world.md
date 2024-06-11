Bevy's `DynamicScene` is a collection of resources and entities that can be serialized to create collections like prefabs or savegame data. When a DynamicScene is deserialized and written into a World - such as when a saved game is loaded - the dynamic entity identifiers inside the scene must be mapped to their newly spawned counterparts.

Previously, this mapping was only available to Entity identifiers stored on Components. In Bevy 0.14, Resources can reflect `MapEntitiesResource` and implement the `MapEntities` trait to get access to the `EntityMapper`.

```rust
    // This resource reflects MapEntitiesResource and implements the MapEntities trait.
    #[derive(Resource, Reflect, Debug)]
    #[reflect(Resource, MapEntitiesResource)]
    struct TestResource {
        entity_a: Entity,
        entity_b: Entity,
    }

    // A simple and common use is a straight mapping of the old entity to the new.
    impl MapEntities for TestResource {
        fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
            self.entity_a = entity_mapper.map_entity(self.entity_a);
            self.entity_b = entity_mapper.map_entity(self.entity_b);
        }
    }
```

Any mention or import of types in the affected modules have to add the respective module name to the import path.
F.e.:
`bevy::ecs::entity::EntityIndexSet` -> `bevy::ecs::entity::index_set::EntityIndexSet`

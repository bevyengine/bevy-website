<!-- Retained Rendering -->
<!-- https://github.com/bevyengine/bevy/pull/14449 Original PR -->
<!-- https://github.com/bevyengine/bevy/pull/15320 Adopted PR -->
<!-- https://github.com/bevyengine/bevy/pull/15582 Synchronized Removed Components -->
<!-- https://github.com/bevyengine/bevy/pull/15756 Type Safe Retained Render World -->

With the advent of the retained render world, entities are no longer despawned at the end of every frame in the render world.
Extracted entities with the `TemporaryRenderEntity` component will be despawned at the end of every frame like before.

In order to make this possible, the `Entity` identifiers in the main and the extracted version in render world are no longer guaranteed to line up. As a result:

- all tools to spawn entities with a precise `Entity` id are in the process of being deprecated and will be removed
- collections that contain references to `Entity` that are extracted into the render world have been changed to contain `MainEntity` in order to prevent errors where a render world entity id is used to look up an item by accident. Custom rendering code may need to be changed to query for `&MainEntity` in order to look up the correct item from such a collection
  - users who implement their own extraction logic for collections of main world entity should strongly consider extracting into a different collection that uses `MainEntity` as a key.
- render phases now require specifying both the `Entity` and `MainEntity` for a given `PhaseItem`. Custom render phases should ensure `MainEntity` is available when queuing a phase item

Renderers can now check `RenderVisibleEntities` to avoid rendering items that are not visible from a view. `RenderVisibleMeshEntities`, `RenderCubemapVisibleEntities`, and `RenderCascadeVisibleEntities` are also available for more fine-grained control.

To guide you further, let's take a look at a few common patterns.
For every example, we specify in which world the code is run.

#### Spawning entities in the render world

Previously, if you spawned an entity with `world.spawn(...)`, `commands.spawn(...)` or some other method in the rendering world, it would be despawned at the end of each frame. In 0.15, this is no longer the case and so your old code could leak entities. This can be mitigated by either re-architecting your code to no longer continuously spawn entities (like you're used to in the main world), or by adding the `bevy_render::world_sync::TemporaryRenderEntity` component to the entity you're spawning. Entities tagged with `TemporaryRenderEntity` will be removed at the end of each frame (like before).

#### Extract components with `ExtractComponentPlugin`

```rust
// main world
app.add_plugins(ExtractComponentPlugin::<ComponentToExtract>::default());
```

`ExtractComponentPlugin` has been changed to automatically sync entities with `ComponentToExtract`. This is done via the new `WorldSyncPlugin`. Any code using `ExtractComponentPlugin` will not require any changes.

#### Manual extraction using `Extract<Query<(Entity, ...)>>`

```rust
// in render world, inspired by bevy_pbr/src/cluster/mod.rs
pub fn extract_clusters(
    mut commands: Commands,
    views: Extract<Query<(Entity, &Clusters, &Camera)>>,
) {
    for (entity, clusters, camera) in &views {
        // some code
        commands.get_or_spawn(entity).insert(...);
    }
}
```

An extract query in the render world queries for entities and components in the main world. Here `entity` is a main world entity and `get_or_spawn(main_world_entity).insert(...)` potentially inserts components on the wrong entity. Remember, there is no longer a one-to-one correspondence between the main and render world entities. Moreover `get_or_spawn` has been deprecated.

In 0.15, you should use `RenderEntity` in place of `Entity` to get the correct entity in the render world. For entities to have a `RenderEntity` they need to be synced first. This can be done either via `WorldSyncPlugin` or adding the `SyncToRenderWorld` to the main world entity.

This results in the following code:

```rust
// in render world, inspired by bevy_pbr/src/cluster/mod.rs
pub fn extract_clusters(
    mut commands: Commands,
    views: Extract<Query<(RenderEntity, &Clusters, &Camera)>>,
) {
    for (render_entity, clusters, camera) in &views {
        // some code
        // After the sync step, all main world entities with a &RenderEntity have a corresponding (empty) render world entity. This should never panic.
        commands.entity(render_entity).insert(...);
    }
}

// in main world, when spawning
world.spawn((Clusters::default(), Camera::default(), SyncToRenderWorld))
```

#### Looking up main world entities in the render world

In order to get the main world entity from a render world entity. It works much the same. Every synced render world entity has a `MainEntity` component you can query for that returns the correct main world entity.

```rust
// in the render world
pub fn inspect_clusters(
    views: Query<(MainEntity, &Clusters, &Camera)>
) {
    for (main_entity, clusters, camera in &views) {
        // do something
    }
}
```

#### General advice for working with main and render world entities

When working with entities from both worlds it can be confusing. If you are every in a scenario where this isn't entirely clear (for example, when working on custom extraction code in the render world), we advise that you use `RenderEntity` and `MainEntity` as simple wrappers around `Entity`. Mixing these up can become a real headache and lead to some non-obvious errors.

```rust
// render world 0.14
pub instances: Vec<(Entity, RenderLayers, bool)>,

// render world 0.15
pub instances: Vec<(MainEntity, RenderLayers, bool)>,
```

There are also other ways to disambiguate between the two worlds.

```rust
// render world 0.14
pub(crate) render_lightmaps: EntityHashMap<RenderLightmap>,

// render world 0.15
pub(crate) render_lightmaps: MainEntityHashMap<RenderLightmap>,
```

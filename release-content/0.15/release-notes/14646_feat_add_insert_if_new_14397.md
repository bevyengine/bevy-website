<!-- feat: add `insert_if_new` (#14397) -->
<!-- https://github.com/bevyengine/bevy/pull/14646 -->
Often there are reasons to insert some components (e.g. `Transform`) separately from the rest of a bundle (e.g. `SpriteBundle` or `PbrBundle`). However `insert` overwrites existing components, making this difficult.

For example, say you have a simulation that can run headless. In headless mode, entities still get a `TransformBundle`, but don't need any rendering info. In normal mode, they get a full `PbrBundle`.

A nice way to implement this is to have the simulation code add the `TransformBundle` with positions etc., and have a rendering system with `Query<Entity, Added<MyThing>>` which inserts a PbrBundle when active.

But `PbrBundle` contains `Transform`, so if you do this, it will clobber the existing transform.

The new method `insert_if_new` (on `EntityCommands` and `EntityMut`) addresses this. It's the same as `insert`, except if a component is a duplicate, it keeps the *old* value instead of the new one. This makes it easier to add bundles incrementally to an entity, without clobbering existing components.

```rust
fn setup(mut commands: Commands) {
    ...
    commands.spawn((
        MySimObject { ... },
        TransformBundle { transform: { ... }, ..default() },
    ))
}

// This system is only added when rendering is enabled
fn setup_rendering(mut commands: Commands, q: Query<(Entity, MySimObject), Without<Sprite>) {
    for (e, simobj) in &q {
      let mesh = ...; // this might be relatively complex and unrelated to the simulation
      let material = ...;
      commands.entity(e).insert_if_new(PbrBundle { mesh, material, ..default() });
    }
}
```

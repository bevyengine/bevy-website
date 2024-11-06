<!-- Dedicated `Reflect` implementation for `Set`-like things -->
<!-- https://github.com/bevyengine/bevy/pull/13014 -->

Up until version 0.14, our `std::collections::HashSet` and `hashbrown::HashSet` didn't enjoy the same level of reflectability as, for example, `std::collections::HashMap`. Now, with version 0.15, these set-like types are fully reflectable!

### Why does this matter?

If you're a fan of the excellent `bevy-inspector-egui` plugin crate, you'll now be able to see the contents of your `HashSet`s after updating both the crate to the version compatible with bevy 0.15. This enhancement allows for deeper introspection and easier debugging in your game development workflow.

Here is an example of entity connections where this improvement has a good impact:

```rust 
#[derive(Component)]
pub struct Person;

/// previously inspecting this would only show 
/// 
/// "HashSet<Entity, EntityHash> is #[reflect_value], but has no
/// InspectorEguiImpl registered in the TypeRegistry. Try calling
/// .register_type::<HashSet<Entity, EntityHash>> or add the
/// DefaultInspectorConfigPlugin for builtin types."
/// 
/// which either didn't work or would need a lot of extra code just to get it
/// to work
#[derive(Component, Reflect)]
pub struct Friendships(EntityHashSet<Entity>);

fn spawn_friends(mut commands: Commands) {
    let adelia = commands.spawn(Name::new("Adelia")).id();
    let diego = commands.spawn(Name::new("Diego")).id();
    let reyna = commands.spawn(Name::new("Reyna")).id();

    commands
        .entity(adelia)
        .insert(Friendships(EntityHashSet::from_iter([diego, reyna])));
    commands
        .entity(diego)
        .insert(Friendships(EntityHashSet::from_iter([adelia])));
}
```

[`std::collections::HashSet`]: https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html
[`hashbrown::HashSet`]: https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html
[`Reflect`]: https://docs.rs/bevy/0.15.0-rc.3/bevy/reflect/trait.Reflect.html
[`std::collections::HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
[`bevy-inspector-egui`]: https://github.com/jakobhellermann/bevy-inspector-egui

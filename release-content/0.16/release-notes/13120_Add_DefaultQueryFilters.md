**Bevy 0.16** adds the ability to disable entities by adding the [`Disabled`] component. This (by default) will hide the entity (and all of its components) from systems and queries that are looking for it.

This is implemented using the newly added **default query filters**. These do what they say on the tin: every query will act as if they have a `Without<Disabled>` filter, unless they explicitly mention [`Disabled`] (generally via a `With<Disabled>` or `Option<&Disabled>` argument).

Because this is generic, developers can also define additional disabling components, which can be registered via [`App::register_disabling_component`]. Having multiple distinct disabling components can be useful if you want each form of disabling to have its own semantics / behaviors: you might use this feature for hiding networked entities, freezing entities in an off-screen chunk, creating a collection of prefab entities loaded and ready to spawn, or something else entirely.

Note that for maximum control and explicitness, only the entities that you directly add disabling components to are disabled. Their children or other related entities are not automatically disabled!

To disable an entity _and_ its children, consider the new [`Commands::insert_recursive`] and [`Commands::remove_recursive`].

[`Disabled`]: https://docs.rs/bevy/0.16/bevy/ecs/entity_disabling/struct.Disabled.html
[`Commands::insert_recursive`]: https://docs.rs/bevy/0.16/bevy/prelude/struct.EntityCommands.html#method.insert_recursive
[`Commands::remove_recursive`]: https://docs.rs/bevy/0.16/bevy/prelude/struct.EntityCommands.html#method.remove_recursive

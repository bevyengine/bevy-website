How would go about marking an entity as disabled in an ECS: unseen-by-default and uninteractable?
Simply deleting it is definitely reliable, but it's relatively expensive, throws away the data, and makes it hard to enable the entity again.
You could move it into a shadow [`World`], but that's esoteric, expensive, and makes it challenging to fetch any data off of it and pass it back into the main [`World`].

What about adding a simple marker component? That seems easy enough: make a unit struct, insert it, and then use `Without<Disabled>` in all of the relevant queries!

While that approach works at first, it quickly becomes frustrating. Did you forget to add the boilerplate somewhere?
Do you vendor all of your dependencies and change their queries too? What if you're making a library? Do all of *your* users need to remember to filter out disabled entities too?

Conceptually though, this seems like the right idea: simply hide the entity from queries and systems that aren't looking for it.
To make this pattern less frustrating Bevy 0.16 introduces the idea of **default query filters**.

These do what they say on the tin: every query will act as if they have a `Without<Disabled>` filter, unless they explicitly mention [`Disabled`] (generally via a `With<Disabled>` or `Option<&Disabled>` argument).
Because this machinery was already built, Bevy allows users (and libraries) to define their own disabling components,
which can be registered via [`App::register_disabling_component`].
Having multiple distinct disabling components can be useful if you want each form of disabling to have its own semantics (or custom behavior!): you might use this feature for hiding networked entities, freezing entities in an off-screen chunk, creating a collection of prefab entities loaded and ready to spawn or something else entirely.

To disable or enable an entity, simply remove or add the disabling component of your choice. It's that easy!
Note that for maximum control and explicitness, only the entities that you directly add disabling components to are disabled: their children or other related entities are not automatically disabled!
This can lead to strange bugs, so in most cases, you should either be careful to call [`Commands::insert_recursive`] and [`Commands::remove_recursive`] or add a hook or observer to get automatic hierarchy-aware disabling.

[`World`]: https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.World.html
[`Disabled`]: https://dev-docs.bevyengine.org/bevy/ecs/entity_disabling/struct.Disabled.html
[`Commands::insert_recursive`]: https://dev-docs.bevyengine.org/bevy/prelude/struct.EntityCommands.html#method.insert_recursive
[`Commands::remove_recursive`]: https://dev-docs.bevyengine.org/bevy/prelude/struct.EntityCommands.html#method.remove_recursive

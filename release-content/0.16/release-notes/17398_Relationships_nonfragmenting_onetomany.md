When building Bevy apps, it is often useful to "link" entities together. The most common case in Bevy is connecting parent and child entities together. In previous Bevy versions, a child would have a `Parent` component, which stored a reference to the parent entity, and the parent entity would have a `Children` component, which stored a list of all of its children entities. To ensure these connections remained valid, developers were not allowed to modify these components directly. Instead, all changes had to be done via specialized commands.

This worked, but it had some pretty glaring downsides:

1. Maintaining hierarchies was "separate" from the core ECS data model. This made it hard to improve our spawn APIs, and made interacting with hierarchies less natural.
2. The system was specialized and not reusable. Developers wanting to define their own relationship types had to reinvent the wheel.
3. To ensure data integrity, expensive scans were required to avoid duplicates.

In **Bevy 0.16** we have added initial support for Relationships: a generalized and efficient component-driven system for linking entities together bidirectionally. This is what defining a new [`Relationship`] looks like:

```rust
/// This is a "relationship" component.
/// Add it to an entity that "likes" another entity.
#[derive(Component)]
#[relationship(relationship_target = LikedBy)]
struct Likes(pub Entity);

/// This is the "relationship target" component.
/// It will be automatically inserted and updated to contain
/// all entities that currently "like" this entity.
#[derive(Component, Deref)]
#[relationship_target(relationship = Likes)]
struct LikedBy(Vec<Entity>);

// Later in your app
let e1 = world.spawn_empty().id();
let e2 = world.spawn(Likes(e1)).id();
let e3 = world.spawn(Likes(e1)).id();

// e1 is liked by e2 and e3 
let liked_by = world.entity(e1).get::<LikedBy>().unwrap();
assert_eq!(&**liked_by, &[e2, e3]);
```

The [`Relationship`] component is the "source of truth", and the [`RelationshipTarget`] component is updated to reflect that source of truth. This means that adding/removing relationships should always be done via the [`Relationship`] component.

We use this "source of truth" model instead of allowing both components to "drive" for performance reasons. Allowing writes to both sides would require expensive scanning during inserts to ensure they are in sync and have no duplicates. The "relationships as the source of truth" approach allows us to make adding relationships constant-time (which is an improvement over previous Bevy versions!).

Relationships are built on top of Bevy's [Component Hooks](/news/bevy-0-14/#ecs-hooks-and-observers), which immediately and efficiently maintains the connection between the [`Relationship`] and the [`RelationshipTarget`] by plugging directly into the component add/remove/update lifecycle. In combination with the new Immutable Components feature (relationship components are immutable), this ensures data integrity is maintained no matter what developers do!

Bevy's existing hierarchy system has been fully replaced by the new [`ChildOf`] [`Relationship`] and [`Children`] [`RelationshipTarget`]. Adding a child is now as simple as:

```rust
commands.spawn(ChildOf(some_parent));
```

Likewise reparenting an entity is as simple as:

```rust
commands.entity(some_entity).insert(ChildOf(new_parent));
```

We also took this chance to improve our spawn APIs more generally. Read the next section for details!

Note that this is just the first step for relationships. We have plans to expand their capabilities:

1. Many-To-Many Relationships: The current system is one-to-many (ex: The `ChildOf` Relationship points to "one" target entity and the `RelationshipTarget` can be targeted by "many" child entities). Some relationships could benefit from supporting many relationship targets.
2. Fragmenting Relationships: The current system does not "fragment" ECS archetypes (Ex: `(Player, ChildOf(e1))`, and `(Player, ChildOf(e2))` exist in the same archetype). In some cases, for performance or logic reasons, a relationship might want to fragment the archetype based on its _value_ rather than just its type. Fragmenting Relationships would enable these cases to opt-in to fragmenting behavior.

[`Relationship`]: https://dev-docs.bevyengine.org/bevy/ecs/relationship/trait.Relationship.html
[`RelationshipTarget`]: https://dev-docs.bevyengine.org/bevy/prelude/trait.RelationshipTarget.html
[`ChildOf`]: https://dev-docs.bevyengine.org/bevy/prelude/struct.ChildOf.html
[`Children`]: https://dev-docs.bevyengine.org/bevy/ecs/hierarchy/struct.Children.html

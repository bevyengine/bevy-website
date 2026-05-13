+++
title = "Relations"
insert_anchor_links = "right"
[extra]
weight = 6
status = 'hidden'
+++

By themselves, entities exist in a flat data structure.
**Relations** makes it possible to organize entities into groups or hierarchies.
This has numerous potential uses:

- A 3D scene may be made up of hierarchies of game objects.
- A game user interface may be comprised of hierarchical 2D elements.
- A container or character inventory might use relations to associate an entity with its contents.
- Relations can be used to track abstract concepts such as ownership.

There is one particular type of relation that gets used a lot in Bevy: the [`ChildOf`] relation.
This is a [`Component`] which is inserted into a child entity, holding a reference to the child's parent entity.
The parent has a corresponding [`Children`] component which contains a reference to all of the parent's children.

Like all relations, the `ChildOf` / `Children` relation is dual-ended and directed, with separate components maintaining each end of the relationship.
It represents a "one-to-many" relationship, with the parent being the one, and the children being the many.

[`Component`]: https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html
[`ChildOf`]: https://docs.rs/bevy/latest/bevy/ecs/hierarchy/struct.ChildOf.html
[`Children`]: https://docs.rs/bevy/latest/bevy/ecs/hierarchy/struct.Children.html

## Adding Children

[`Commands`] can be used to add children to an entity directly:

```rs
fn spawn_entity_with_children(mut commands: Commands) {
    // Spawn a car
    let vehicle = commands.spawn((Camaro, Color::Red)).id();
    // Spawn 4 wheels and attach to the car
    commands.entity(vehicle).add_children(&[
        commands.spawn((Wheel, Color::Black)).id(),
        commands.spawn((Wheel, Color::Black)).id(),
        commands.spawn((Wheel, Color::Black)).id(),
        commands.spawn((Wheel, Color::Black)).id(),
    ]);

    // You can also use `with_children`:
    commands.entity(vehicle).with_children(|parent| {
        parent.spawn((BumperSticker::new("I brake for Bevy")))
    });
}
```

Note that `add_children` and `with_children` only set up the `ChildOf` and `Children` components, and nothing else.
If you want any other components, you'll need to add them yourself.

The `ChildOf` relation uses the [`linked_spawn`] option, which means that when a parent is despawned, it's children (and their children, and so on) are also despawned automatically.

```rs
fn despawn_vehicle(mut commands: Commands, vehicle: Entity) {
    // Despawn the vehicle and all its children.
    commands.entity(vehicle).despawn();
}
```

If you just wanted to despawn the children, and not the parent, you can use `despawn_children`:

```rs
fn despawn_wheels(mut commands: Commands, vehicle: Entity) {
    // Despawn only the children of the vehicle
    commands.entity(vehicle).despawn_children();
}
```

There are many more methods for adding and removing children, which you can check out in the [API docs].

[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html
[`linked_spawn`]: https://docs.rs/bevy/latest/bevy/ecs/relationship/trait.Relationship.html#:~:text=linked%5Fspawn
[API docs]: https://docs.rs/bevy/latest/bevy/prelude/struct.EntityCommands.html

### Adding Children Declaratively

There's another way to add children to an entity, one which lets you create an entire hierarchy in a single `spawn()` call:

```rs
fn spawn_entity_with_children(mut commands: Commands) {
    // Spawn a car and its wheels
    commands.spawn((
        Camaro,
        Color::Red,
        Children::spawn((
            Spawn((Wheel, Color::Black)),
            Spawn((Wheel, Color::Black)),
            Spawn((Wheel, Color::Black)),
            Spawn((Wheel, Color::Black)),
            Spawn((BumperSticker::new("I brake for no one!"))),
        ))
    ));
}
```

{% callout(type="info") %}
The `Children::spawn` call uses an advanced feature called a "bundle effect".
This is an additional side-effect that happens after the `Children` component is inserted.
In this case, the child entities are actually created once the component is inserted.

Bundle effects also work for `insert()` as well as `spawn()`.
{% end %}

The [`children!`] macro can make this code even more concise:

```rs
fn spawn_entity_with_children(mut commands: Commands) {
    // Spawn a car and its wheels
    commands.spawn((
        Camaro,
        Color::Red,
        children!([
            (Wheel, Color::Black),
            (Wheel, Color::Black),
            (Wheel, Color::Black),
            (Wheel, Color::Black),
            (BumperSticker::new("I brake for no one!")),
        ])
    ));
}
```

{% callout(type="warn") %}
**Caution**

In this shorter syntax, the parentheses around each child entity can trip you up if you are not careful.
The expression `(Wheel, Color::Black)` adds a single child entity with two components.
If you instead wrote `Wheel, Color::Black` (without the parentheses) that would add two separate entities, each with just one component.

There are also some advanced scenarios where using `children!` may not be appropriate, as the longer `Children::spawn()` syntax gives you greater flexibility.
{% end %}

[`children!`]: https://docs.rs/bevy/latest/bevy/ecs/macro.children.html

## Querying Relations

You can query for `Children` just like any other component:

```rust
// Query children
fn scan_children(mut query: Query<(&Color, &Children)>) {
    for (color, children) in query.iter() {
        // But...think of the children!
    }
}

// Query parents
fn scan_parents(mut query: Query<(&Color, &ChildOf)>) {
    for (color, parent) in query.iter() {
        // But...also think of the parents!
    }
}
```

## Traversing Hierarchies

The `bevy_ecs` crate has a bunch of handy methods for traversing hierarchies.
You can travel upwards through ancestors (using [`Query::iter_ancestors`]), or downwards through descendants (using [`Query::iter_descendants`]).
All you need to kick off the process is a query able to access the relationship components:

```rust
// Query descendants
fn scan_descendants(mut query: Query<&Children>, start: Entity) {
    for entity in query.iter_descendants(start) {
        // But...what about the descendants!
    }
}
```

[`Query::iter_ancestors`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Query.html#method.iter_ancestors
[`Query::iter_descendants`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Query.html#method.iter_descendants

## Defining Other Relations

You can also create new _kinds_ of relations.
Let's say we want to define a relation that represents the contents of a treasure chest or character inventory.
Since we don't want the container's contents to be drawn in the scene, we don't want to use the normal `ChildOf` relation.
Instead, we'll define a new `ContainedBy` relation.
The other end of the relation (called the "relationship target") will be called `Contents`.

```rust
/// A relation representing the container of an item.
#[derive(Component, Clone, PartialEq, Eq, Debug)]
#[relationship(relationship_target = Contents)]
pub struct ContainedBy(pub Entity);

impl Default for ContainedBy {
    fn default() -> Self {
        ContainedBy(Entity::PLACEHOLDER)
    }
}

/// A collection representing the contents of a container.
#[derive(Component, Default)]
#[relationship_target(relationship = ContainedBy, linked_spawn)]
pub struct Contents(Vec<Entity>);
```

{% callout(type="info") %}
**Note on naming**:
When speaking of relationships, names are important because otherwise it's easy for language to get confused.
For example, if a component is named `Parent`, does that mean that the entity _is_ a parent, or that it _has_ a parent?

The convention in Bevy is to try and pick names that are unambiguous.
For example, `ChildOf` means that this entity _is_ a child, not that it _has_ a child.
Similarly, `ContainedBy` makes it clear that this is an item in a container, not a container itself.
{% end %}

Now that we've defined our new relation, we can start using it!
Remember in the previous sections all of the various methods for adding children?
Well, those methods are just special cases of more general ones.
Instead of `add_children()`, you could use `add_related::<ContainedBy>()`, and instead of `Children::spawn()` you can call `Contents::spawn()`.

### Defining Self-Relationships

As your game develops further, you might find that there are cases where you want a relationship between an `Entity` and _itself_.
Maybe your game is inspired by JRPGs and you want to include the player's character in a custom `Party` relationship collection, or perhaps the enemy boss should include themselves in a `EnemyCombatants` relationship collection.
However you wish to implement it, including an `Entity` in a relationship collection that the same `Entity` owns can potentially help simplify the logic required to build your game.

However, Bevy does not allow this by default: attempting to create a relationship between an `Entity` and itself will simply remove the relationship component from the `Entity` and a warning will be logged.
To get around the default behavior, we have to include the `allow_self_referential` attribute when defining our relationship struct:

```rust
#[derive(Component)]
#[relationship(relationship_target = PeopleILike, allow_self_referential)]
pub struct LikedBy(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = LikedBy)]
pub struct PeopleILike(Vec<Entity>);
```

The `allow_self_referential` attribute will set an internal bool value to `true`, which will allow the relationship to point to its own `Entity`.
Now if we want to create a new `Entity` with our self-relationship, the setup can be as simple as this:

```rust
// Create an empty Entity.
let entity = world.spawn_empty().id();
// Insert a LikedBy relationship on the Entity pointing towards itself.
world.entity_mut(entity).insert(LikedBy(entity));
```

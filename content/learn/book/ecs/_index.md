+++
title = "Entities, components and systems"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

In Bevy, game objects are stored as **entities**, whose data is stored as **components**.
**Systems** operate on this data, modifying the **world** to carry out the behavior that brings your game to life.
Together, these these form the basis of Bevy's **ECS**, which unsurprisingly stands for ["Entity-Component-System"](https://en.wikipedia.org/wiki/Entity_component_system).
Let's go over the most important definitions:

- **Entities:** Game objects (either abstract, like a camera, or tangible, like a player character), whose data is stored as components.
  - The [`Entity`] type is just a simple identifier (like a URL address, a unique name, or a row number in a database): any combination of components can be added to each entity.
- **Components:** Data stored on an entity, that can be manipulated in systems.
  - Each component has a different Rust type that implements the [`Component`] trait, and only one component of each type can exist for each entity.
- **World:** A unifying collection of all of the data stored in the ECS.
- **Systems:** Special functions that read and write data stored in the [`World`].
  - Any function whose parameters all implement the [`SystemParam`] type can be converted into a [`System`].

Suppose we wanted to make a Breakout game in Bevy.
Let's think about what entities we might want, what components they might have, and what systems we might create:

- Paddle entities
  - a dataless `Paddle` **marker component**, to allow us to uniquely identify the paddle
  - a [`Sprite`] component, which describes how to draw these bundles
    - in reality, this is a bit more complex, and requires a [`SpriteBundle`] collection of components
  - a [`Transform`] component, to let us know the translation (position), rotation (orientation) and scale (size) of our paddles
  - a `Velocity` component, giving us more realistic movement
  - a `Collidable` component, to let us know that the ball can bounce off of it
- Ball entity
  - a `Ball` marker component, so we can uniquely identify our ball
  - a [`Sprite`] component
  - a [`Transform`] component
  - a `Velocity` component, to ensure the ball keeps moving
- Brick entities
  - a [`Brick`] marker component
  - a [`Sprite] component
  - a [`Transform] component
  - a `Collidable` component
- Wall entity
  - a `Collidable` component, to make sure our ball bounces off the walls
  - a `Transform` component, so we know where the boundaries are

As you can see, each component implies specific behavior, but does not provide it on its own: they're just data, and cannot act on their own.
The components are quite small, allowing us to reuse these types and share behavior across entities using systems that operate on all entities.
For our simple Breakout game, we may have:

- `setup`: a simple **startup system** that runs once when our game is launched, **spawning** our paddles, ball and walls
- `apply_velocity`: a system that operates on all entities with a `Transform` and `Velocity`, and moves the entity according to its velocity
- `handle_collisions`: a system that operates on the `Ball` entity, and any entity with both a `Collidable` component and a `Transform` component, to bounce the ball appropriately
- `destroy_bricks`: a system that **despawns** entities with the `Brick` marker component when they are collided with

If you'd like to see what a basic but complete Breakout game looks like in Bevy, check out the [Breakout example]!

In order to start working with Bevy, you should know a few other critical pieces of ECS vocabulary:

- **Resources:** Globally unique stores of data that live in the [`World`], but are not associated with a specific entity.
  - Events, configuration and global game state are all commonly stored as resources, which can be accessed with the [`Res`] system parameter.
- **Queries:** Requests for specific entity-component data from the [`World`].
  - The [`Query`] type has two type parameters: the first describes what component data should be fetched, and the second filters down which entities with that data should be returned.
- **Commands:** Instructions to modify the [`World`] at a later point in time.
  - Most commonly, this is used to spawn and despawn entities, or insert and removed components.
  - [`Commands`] require [exclusive world access](./exclusive-world-access/), and so are deferred until there are no other systems running.

While there's much more to learn about Bevy's ECS, this basic overview should give you the vocabulary you need to start exploring the rest of this chapter.
Don't worry if some concepts are too abstract or impractical for you at this point:
this book is intended to be skimmed on the first read.
Refer back to it later for more detailed explanations as you start building your own awesome projects in Bevy!

[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`Component`]: https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
[`SystemParam`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html
[`System`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.System.html
[`Sprite`]: https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html
[`SpriteBundle`]: https://docs.rs/bevy/latest/bevy/sprite/struct.SpriteBundle.html
[`Res`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Res.html
[`Query`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html
[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html

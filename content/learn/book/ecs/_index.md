+++
title = "Entities, Components and Systems"
template = "docs-section.html"
insert_anchor_links = "right"
[extra]
weight = 2
status = 'hidden'
+++

In Bevy, game objects are stored as **entities**, whose data is stored as **components**.
**Systems** operate on this data, modifying the **world** to carry out the behavior that brings your game to life.
Together, these these form the basis of Bevy's **ECS**, which unsurprisingly stands for ["Entity-Component-System"](https://en.wikipedia.org/wiki/Entity_component_system).
Let's go over the most important definitions:

- **World:** A unifying collection of all of the data stored in the ECS.
  - Access to the [`World`] follows Rust's borrow checker rules: you can read from the same data any number of times, but you must have exclusive access to modify a piece of data.
- **Entities:** Game objects (either abstract, like a camera, or tangible, like a player character), whose data is stored as components.
  - The [`Entity`] type is just a simple identifier (like a URL address, a unique name, or a row number in a database).
- **Components:** Data stored on an entity, that can be manipulated in systems.
  - Each component has a different Rust type that implements the [`Component`] trait, and only one component of each type can exist for each entity.
  - Components without data are called **marker components**, and can be used to efficiently select entities that have a specific property (like being `Poisoned`, or defining a `Player`).
  - Any combination of components can be added to each entity, allowing us to extend and share behavior through composition.
- **Systems:** Special functions that operate on data from the [`World`]: most commonly modifying the data stored in components on entities.
  - Any function whose parameters all implement the [`SystemParam`] type can be converted into a [`System`].

Suppose we wanted to make a [Breakout game](https://github.com/bevyengine/bevy/blob/latest/examples/games/breakout.rs) in Bevy.
Let's think about what entities we might want, what components they might have, and what systems we might create:

- A paddle entity
  - a `Paddle` marker component, to allow us to uniquely identify the paddle
  - a [`Sprite`] component, which describes how to draw our paddle
    - in reality, this is a bit more complex, and requires a [`SpriteBundle`] collection of components
  - a [`Transform`] component, to let us know the translation (position), rotation (orientation) and scale (size) of our paddles
  - a `Velocity` component, giving us more realistic movement
  - a `Collidable` component, to let us know that the ball can bounce off of it
- A ball entity
  - a `Ball` marker component, so we can uniquely identify our ball
  - a [`Sprite`] component
  - a [`Transform`] component
  - a `Velocity` component, to ensure the ball keeps moving
- Brick entities
  - a [`Brick`] marker component
  - a [`Sprite`] component
  - a [`Transform`] component
  - a `Collidable` component
- Wall entities
  - a `Collidable` component, to make sure our ball bounces off the walls
  - a `Transform` component, so we know where the boundaries are

As you can see, each component implies specific behavior, but does not provide it on it. Components are just data (although they often have simple methods), and only act when systems use them.
Each of our components is quite small, allowing us to reuse these types and share behavior across entities using systems.
For our simple Breakout game, we may have:

- `setup`: a simple **startup system** that runs a single time when our game is launched, **spawning** our paddles, ball and walls
- `apply_velocity`: a system that operates on all entities with a `Transform` and `Velocity`, and moves the entity according to its velocity
- `handle_collisions`: a system that operates on the `Ball` entity, and any entity with both a `Collidable` component and a `Transform` component, to bounce the ball appropriately
- `destroy_bricks`: a system that **despawns** entities with the `Brick` marker component when they are collided with

In order to start working with Bevy, you should know a few other critical pieces of ECS vocabulary:

- **Resources:** Globally unique stores of data that live in the [`World`], but are not associated with a specific entity.
  - Events, configuration and global game state are all commonly stored as resources, which can be accessed with the [`Res`] system parameter.
- **Queries:** Requests for specific entity-component data from the [`World`].
  - The [`Query`] type has two type parameters: the first describes what component data should be fetched, and the second filters down which entities with that data should be returned when looping over the query.
- **Commands:** Instructions to modify the [`World`] at a later point in time.
  - Most commonly, this is used to spawn and despawn entities, or insert and remove components.
  - [`Commands`] require [exclusive world access](./exclusive-world-access/), and so are deferred until there are no other systems running.

While there's much more to learn about Bevy's ECS, this basic overview should give you the vocabulary you need to start exploring the rest of this chapter.
Don't worry if some concepts are too abstract, advanced or impractical for you at this point:
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

<!-- Hooks: https://github.com/bevyengine/bevy/pull/10756 -->
<!-- Observers: https://github.com/bevyengine/bevy/pull/10839 -->

As much as we love churning through homogeneous blocks of data in a tight loop here at Bevy; not every task is a perfect fit for the straightforward ECS model.
Responding to changes and/or processing events are vital tasks in any application, and games are no exception.

Bevy has accumulated a number of subtly distinct tools to handle this over the years, and in 0.14 is picking up two more.
Let's quickly go over the existing tools to contextualize why adding more is useful:

- Buffered [`Event`]s:multiple-producer, multiple-consumer queues. Flexible and efficient, but requires polling. Events are droped after two frames.
- Change detection via [`Added`] and [`Changed`]: Granular, automatic, great integration with queries. Linear time checks, no added data.
- [`RemovedComponents`]: A special form of event that is triggered when a component is removed from an entity, or an entity with that component is despawned.

All of these (and systems themselves!) use a ["pull"-style mechanism]: metaphorical events are sent regardless of whether or not anyone is listening, and listeners must periodically poll to ask if anything has changed.
This is a useful pattern, and one we intend to keep around!
By polling, we can process events in batch, getting more context and improving data locality (which makes the CPU go brr).

But it comes with some limitations:

- polling has a small but non-zero overhead every frame
- there is an unavoidable delay between an event being triggered and the response being processed

This delay is the critical problem:

- data (like indexes or hierarchies) exists, even momentarily, in an invalid state
- we can't process arbitrary chains of recursive logic within a single cycle, no matter how carefully we order our event writers and readers

To overcome these limitations, Bevy 0.14 introduces two complementary "push"-style mechanisms inspired by the ever-wonderful C ECS, [flecs]:

1. [**Component Lifecyle Hooks:**](https://dev-docs.bevyengine.org/bevy/ecs/component/struct.ComponentHooks.html) Mutations to the world that automatically occur in response to **lifecycle events**: whenever a component of the given type is added, overwritten or removed.
2. [**Observers:**](https://dev-docs.bevyengine.org/bevy/ecs/observer/struct.Observer.html) On-demand systems that listen to [`Trigger`] events, which commonly target specific entities. These systems are run sequentially and recursively at the next command flush point in response to a trigger being sent.

Only one hook per component type can be registered, and it cannot be overwritten.
It is intended for use by library authors, to strictly enforce invariants (such as hierarchy correctness) required to use that type.
Hooks always run before observers.

By contrast, observers are a flexible tool intended for gamplay logic.
They can listen to the same lifecycle events as hooks, but can also respond to custom, user-defined triggers.
Observers can be attached to a single entity, listening only to triggers targeted at that entity (callbacks anyone?), but they can also be used to listen for triggers without an associated entity.
Their advantages over buffered events are clearest when combined with commands that emit triggers (to avoid ever entering a bad state),
or when you're taking advantage of observers' ability to emit triggers which are then immediately processed, chaining recursively.

In the future, we intend to use hooks and observers to [replace `RemovedComponents`], make our hierarchy management more robust, create a first-party replacement for [`bevy_eventlistener`] as part of our UI work and [build out relations].
These are powerful, abtract tools: we can't wait to see the mad science the community cooks up!

[`Event`]: https://dev-docs.bevyengine.org/bevy/ecs/event/trait.Event.html
[`Added`]: https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.Added.html
[`Changed`]: https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.Changed.html
[`RemovedComponents`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.RemovedComponents.html
["pull"-style mechanism]: https://dev.to/anubhavitis/push-vs-pull-api-architecture-1djo
[flecs]: https://www.flecs.dev/flecs/
[`Trigger`]: https://dev-docs.bevyengine.org/bevy/ecs/observer/struct.Trigger.html
[replace `RemovedComponents`]: https://github.com/bevyengine/bevy/issues/13928
[`bevy_eventlistener`]: https://github.com/aevyrie/bevy_eventlistener
[build out relations]: https://github.com/bevyengine/rfcs/pull/79

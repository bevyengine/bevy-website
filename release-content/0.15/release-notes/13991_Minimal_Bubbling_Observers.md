<!-- Minimal Bubbling Observers -->
<!-- https://github.com/bevyengine/bevy/pull/13991 -->

Virtually every pointer interaction (like mouse click) is rare (humans are slow!), and often requires a complex response.

This pattern is particularly useful in UI, where unhandled interactions are often intended for the pane that *contains* the entity that's on top, but is also valuable for in-game interactions: clicking on a unit's sword should select the unit!

To support this, we've extended the [`Event`] trait to include an associated `Traversal` type and an associated `AUTO_PROPAGATE` constant.
This behavior is opt-in: when you derive the `Event` type, these are set to `()` and `false` respectively.

For the [`Pointer<E>`] event type, we've chosen to implement this as:

```rust
impl <E> Event for Pointer<E>{
    type Traversal = &Parent;
    const AUTO_PROPAGATE: bool = true;
}
```

This means that, unless you call [`Trigger::propagate(false)`], pointer events will be bubbled up the hierarchy (accessing the `Entity` stored in the [`Parent`] component) until it reaches the entity root.

Any type that implements the [`Traversal`] trait can be used as the associated type and can access arbitrary read-only query data from the world.
While using the standard entity hierarchy is a sensible choice for *many* applications, bubbling can be used for arbitrary event propagation using your own [proto-relations].
Let us know what you cook up: user feedback is indispensable for building a better Bevy!

[`Event`]: https://docs.rs/bevy/0.15.0/bevy/ecs/event/trait.Event.html
[`Pointer<E>`]: https://docs.rs/bevy/0.15.0/bevy/picking/events/struct.Pointer.html
[`Trigger::propagate(false)`]: https://docs.rs/bevy/0.15.0/bevy/ecs/prelude/struct.Trigger.html#method.propagate
[`Parent`]: https://docs.rs/bevy/0.15.0/bevy/hierarchy/struct.Parent.html
[proto-relations]: https://github.com/bevyengine/bevy/issues/3742
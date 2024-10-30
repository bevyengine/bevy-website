<!-- Add `EntityRefExcept` and `EntityMutExcept` world queries, in preparation for generalized animation. -->
<!-- https://github.com/bevyengine/bevy/pull/15207 -->

<!-- TODO -->

[`EntityMut`] and [`EntityRef`] are powerful tools for interacting with all components of a given entity at once in arbitrary ways.
These types implement `QueryData`, so you can add them to any `Query` you'd like!

However, because they can access *any* component information, Rust's prohibition against mutable aliasing prevent you from simultaneously accessing other component information,
even if you pinky promise not to read any data that's being written to.

```rust
// This system is forbidden!
// 
// Inside the body of the function, we could choose to mutate the `AnimationPlayer` itself
// while reading its value!
fn animate_anything(query: Query<(&AnimationPlayer, EntityMut)> ){}
```

To let you work around this limitation, we've introduced a matching pair of tools: [`EntityMutExcept`] and [`EntityRefExcept`],
which work just like the [`EntityMut`] and [`EntityRef`] but *don't* provide access to a bundle of components that you declare off-limits.

```rust
/// Look mom, no mutable aliasing!
fn animate_anything(query: Query<(&AnimationPlayer, EntityMutExcept<AnimationPlayer>)> ){}
```

[`EntityMut`]: https://docs.rs/bevy/0.15/bevy/ecs/prelude/struct.EntityMut.html
[`EntityRef`]:https://docs.rs/bevy/0.15/bevy/ecs/prelude/struct.EntityRef.html
[`EntityMutExcept`]: https://docs.rs/bevy/0.15/bevy/ecs/world/struct.EntityMutExcept.html
[`EntityRefExcept`]: https://docs.rs/bevy/0.15/bevy/ecs/world/struct.EntityRefExcept.html

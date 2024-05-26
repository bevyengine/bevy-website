- `AnimationClip` now uses UUIDs instead of hierarchical paths based on the `Name` component to refer to bones. This has several consequences:
  - A new component, `AnimationTarget`, should be placed on each bone that you wish to animate, in order to specify its UUID and the associated `AnimationPlayer`. The glTF loader automatically creates these components as necessary, so most uses of glTF rigs shouldn’t need to change.
  - Moving a bone around the tree, or renaming it, no longer prevents an `AnimationPlayer` from affecting it.
  - Dynamically changing the `AnimationPlayer` component will likely require manual updating of the `AnimationTarget` components.

- Entities with `AnimationPlayer` components may now possess descendants that also have `AnimationPlayer` components. They may not, however, animate the same bones.
- As they aren’t specific to `TypeId`s, `bevy_reflect::utility::NoOpTypeIdHash` and `bevy_reflect::utility::NoOpTypeIdHasher` have been renamed to `bevy_reflect::utility::NoOpHash` and `bevy_reflect::utility::NoOpHasher` respectively.

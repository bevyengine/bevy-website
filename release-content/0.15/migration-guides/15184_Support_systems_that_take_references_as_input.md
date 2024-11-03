- All current explicit usages of the following types must be changed in the way specified:
  - `SystemId<I, O>` to `SystemId<In<I>, O>`
  - `System<In = T>` to `System<In = In<T>>`
  - `IntoSystem<I, O, M>` to `IntoSystem<In<I>, O, M>`
  - `Condition<M, T>` to `Condition<M, In<T>>`

- `In<Trigger<E, B>>` is no longer a valid input parameter type. Use `Trigger<E, B>` directly, instead.

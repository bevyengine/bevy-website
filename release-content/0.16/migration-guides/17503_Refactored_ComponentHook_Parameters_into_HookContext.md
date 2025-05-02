The function signature of component hooks (`ComponentHook`) has been simplified so that all arguments beyond the `DeferredWorld` is passed in a `HookContext`. Note that because `HookContext` is plain data with all public fields, you can use de-structuring to simplify migration.

```rust
// 0.15
fn my_hook(
    mut world: DeferredWorld,
    entity: Entity,
    component_id: ComponentId,
) {
    // ...
}

// 0.16
fn my_hook(
    mut world: DeferredWorld,
    HookContext { entity, component_id, caller }: HookContext,
) {
    // ...
}
```

Likewise, if you were discarding certain parameters, you can use `..` in the de-structuring:

```rust
// 0.15
fn my_hook(
    mut world: DeferredWorld,
    entity: Entity,
    _: ComponentId,
) {
    // ...
}

// 0.16
fn my_hook(
    mut world: DeferredWorld,
    HookContext { entity, .. }: HookContext,
) {
    // ...
}
```

Update the function signatures for your component hooks to only take 2 arguments, `world` and `context`. Note that because `HookContext` is plain data with all members public, you can use de-structuring to simplify migration.

```rust
// Before
fn my_hook(
    mut world: DeferredWorld,
    entity: Entity,
    component_id: ComponentId,
) { ... }

// After
fn my_hook(
    mut world: DeferredWorld,
    HookContext { entity, component_id, caller }: HookContext,
) { ... }
```

Likewise, if you were discarding certain parameters, you can use `..` in the de-structuring:

```rust
// Before
fn my_hook(
    mut world: DeferredWorld,
    entity: Entity,
    _: ComponentId,
) { ... }

// After
fn my_hook(
    mut world: DeferredWorld,
    HookContext { entity, .. }: HookContext,
) { ... }
```

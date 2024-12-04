Previously `World::get_resource_ref::<T>` and `World::resource_ref::<T>` would return a `Res<T>` which was inconsistent with the rest of the `World` API (notably `resource_scope`). This has been fixed and the methods now return `Ref<T>`. 

This means it is no longer possible to get `Res<T>` from `World`. If you were relying on this, you should try using `Ref<T>` instead since it has the same functionality.

__Before__

```rust
let my_resource: Res<MyResource> = world.resource_ref();
function_taking_resource(my_resource);

fn function_taking_resource(resource: Res<MyResource>) { /* ... */ }
```

__After__

```rust
let my_resource: Ref<MyResource> = world.resource_ref();
function_taking_resource(my_resource);

fn function_taking_resource(resource: Ref<MyResource>) { /* ... */ }
```

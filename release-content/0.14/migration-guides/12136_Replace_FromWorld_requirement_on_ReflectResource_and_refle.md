`#[reflect(Resource)]` now requires the `FromReflect` trait to be implemented for your resource. This is done by default if you use `#[derive(Reflect)]`, but you structs that opt-out of this behavior will have to write their own implementation. `FromReflect` was added to replace the `FromWorld` requirement, though `FromReflect` is fallible. You may wish to add `#[reflect(FromWorld)]` to your resources to maintain an infallible variant.

Finally, if you use the `ReflectResource` struct you will need to pass a `&TypeRegistry` to its `insert`, `apply_or_insert`, and `copy` methods.

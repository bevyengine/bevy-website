`Result`'s `Reflect` implementation has been changed to make it a `ReflectKind::Enum` instead of a `ReflectKind::Value`. This increases its consistency with `Option` and allows for inspection of its contents.

Now, `Result<T, E>` no longer requires both `T` and `E` to be `Clone`, but instead requires them to implement `FromReflect`. Additionally, `<Result<T, E> as Reflect>::reflect_*` now returns the `Enum` variant instead of `Value`.

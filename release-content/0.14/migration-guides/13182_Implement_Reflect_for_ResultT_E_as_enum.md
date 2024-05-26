`Result<T, E>` has had its `Reflect` implementation changed to align it with `Option<T>` and its intended semantics: A carrier of either an `Ok` or `Err` value, and the ability to access it. To achieve this it is no longer a `ReflectKind::Value` but rather a `ReflectKind::Enum` and as such carries these changes with it:

For `Result<T, E>`

- Both `T` and `E` no longer require to be `Clone` and now require to be `FromReflect`
- `<Result<T, E> as Reflect>::reflect_*` now returns a `ReflectKind::Enum`, so any code that previously relied on it being a `Value` kind will have to be adapted.
- `Result<T, E>` now implements `Enum`

Since the migration is highly dependent on the previous usage, no automatic upgrade path can be given.

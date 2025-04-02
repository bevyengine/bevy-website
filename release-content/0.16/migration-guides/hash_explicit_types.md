`hashbrown`'s default hasher [has changed in `hashbrown 0.15`](https://github.com/rust-lang/hashbrown/pull/563), resulting in serious regressions in type inference and method availability for Bevy users of `HashSet` and `HashMap`.
We have updated versions to avoid compiling duplicate copies of the crate, as Bevy's own dependencies have already updated.

Bevy uses a different (faster on average, but slower in the face of malicious inputs) than `hashbrown`'s new default hasher.
Unfortunately, many methods are only implemented for the slower `HashMap` and `HashSet` types with the slower `RandomState` type as a generic argument, rather than the Bevy's preferred `FixedHasher` type.

For example, the `HashMap::new` method will produce a type that uses the incorrect hasher, leading to errors like:

```sh
error[E0308]: mismatched types
   --> crates/bevy_plugin/src/line_provider/asset_provider.rs:160:14
    |
160 |         Self(HashMap::new())
    |         ---- ^^^^^^^^^^^^^^ expected `HashMap<&str, UntypedHandle, FixedHasher>`, found `HashMap<_, _>`
    |         |
    |         arguments to this function are incorrect
    |
    = note: expected struct `hashbrown::map::HashMap<&'static str, bevy::prelude::UntypedHandle, FixedHasher>`
               found struct `hashbrown::map::HashMap<_, _, bevy::bevy_platform_support::hash::RandomState>`
```

`HashMap::new` can be replaced with `HashMap::default`, which is flexible in the correct way.
Other methods, such as `with_capacity` may be replaced with the `with_capacity_and_hasher` method.

We are understand your frustrations with this regression, and are actively discussing solutions to improve this going forward.

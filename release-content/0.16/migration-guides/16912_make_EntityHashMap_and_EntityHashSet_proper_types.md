Users of `with_hasher` and `with_capacity_and_hasher` on `EntityHashMap`/`Set` must now use `new` and `with_capacity` respectively.
If the non-newtyped versions are required, they can be obtained via `Deref`, `DerefMut` or `into_inner` calls.

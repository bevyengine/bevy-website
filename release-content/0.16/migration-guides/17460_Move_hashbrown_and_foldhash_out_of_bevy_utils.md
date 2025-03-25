- The following items were moved out of `bevy_utils` and into `bevy_platform_support::hash`:
  - `FixedState`
  - `DefaultHasher`
  - `RandomState`
  - `FixedHasher`
  - `Hashed`
  - `PassHash`
  - `PassHasher`
  - `NoOpHash`

- The following items were moved out of `bevy_utils` and into `bevy_platform_support::collections`:
  - `HashMap`
  - `HashSet`

- `bevy_utils::hashbrown` has been removed. Instead, import from `bevy_platform_support::collections` _or_ take a dependency on `hashbrown` directly.
- `bevy_utils::Entry` has been removed. Instead, import from `bevy_platform_support::collections::hash_map` or `bevy_platform_support::collections::hash_set` as appropriate.
- All of the above equally apply to `bevy::utils` and `bevy::platform_support`.

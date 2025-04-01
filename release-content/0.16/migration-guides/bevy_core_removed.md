`bevy_core` has been removed and its items moved into more appropriate locations.
Below are some tables showing where items have been moved to

#### Structs

| Item                             | 0.15 Path   | 0.16 Path         |
| -------------------------------- | ----------- | ----------------- |
| `FrameCount`                     | `bevy_core` | `bevy_diagnostic` |
| `FrameCountPlugin`               | `bevy_core` | `bevy_diagnostic` |
| `Name`                           | `bevy_core` | `bevy_ecs::name`  |
| `NameOrEntity`                   | `bevy_core` | `bevy_ecs::name`  |
| `NameOrEntityItem`               | `bevy_core` | `bevy_ecs::name`  |
| `NonSendMarker`                  | `bevy_core` | `bevy_app`        |
| `TaskPoolOptions`                | `bevy_core` | `bevy_app`        |
| `TaskPoolPlugin`                 | `bevy_core` | `bevy_app`        |
| `TaskPoolThreadAssignmentPolicy` | `bevy_core` | `bevy_app`        |
| `TypeRegistrationPlugin`         | `bevy_core` | _Removed_         |

#### Functions

| Item                             | 0.15 Path   | 0.16 Path         |
| -------------------------------- | ----------- | ----------------- |
| `update_frame_count`             | `bevy_core` | `bevy_diagnostic` |

#### Removed

`TypeRegistrationPlugin` no longer exists. If you can’t use a default `App` but still need `Name` registered, do so manually.
  
```rust
// Before
app.add_plugins(TypeRegistrationPlugin);

// After
app.register_type::<Name>();
```

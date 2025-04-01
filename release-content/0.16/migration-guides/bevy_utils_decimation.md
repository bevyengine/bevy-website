In 0.16 `bevy_utils` (and by extension `bevy::utils`) was significantly reduced with many of its items either being removed, spun-out into their own crates, or just moved into more appropriate existing crates.
Below is a series of tables for all items that were in `bevy_utils` 0.15 that have since been moved or removed in 0.16.

Note that certain items have been completely removed, see below for further details.

**Re-Exports**

| Item        | 0.15 Path    | 0.16 Path  |
| ----------- | ------------ | ---------- |
| `hashbrown` | `bevy_utils` | _Removed_  |
| `tracing`   | `bevy_utils` | `bevy_log` |

**Structs**

| Item                    | 0.15 Path    | 0.16 Path                     |
| ----------------------- | ------------ | ----------------------------- |
| `AHasher`               | `bevy_utils` | `ahash`                       |
| `Duration`              | `bevy_utils` | `core::time`                  |
| `FixedState`            | `bevy_utils` | `bevy_platform_support::hash` |
| `Hashed`                | `bevy_utils` | `bevy_platform_support::hash` |
| `Instant`               | `bevy_utils` | `bevy_platform_support::time` |
| `NoOpHash`              | `bevy_utils` | `bevy_platform_support::time` |
| `PassHash`              | `bevy_utils` | `bevy_platform_support::time` |
| `PassHasher`            | `bevy_utils` | `bevy_platform_support::time` |
| `RandomState`           | `bevy_utils` | `bevy_platform_support::time` |
| `SystemTime`            | `bevy_utils` | `std::time`                   |
| `SystemTimeError`       | `bevy_utils` | `std::time`                   |
| `TryFromFloatSecsError` | `bevy_utils` | `core::time`                  |

**Traits**

| Item                    | 0.15 Path    | 0.16 Path    |
| ----------------------- | ------------ | ------------ |
| `ConditionalSend`       | `bevy_utils` | `bevy_tasks` |
| `ConditionalSendFuture` | `bevy_utils` | `bevy_tasks` |

**Macros**

| Item                   | 0.15 Path    | 0.16 Path          |
| ---------------------- | ------------ | ------------------ |
| `assert_object_safe`   | `bevy_utils` | _Removed_          |
| `dbg`                  | `bevy_utils` | `bevy_log`         |
| `error`                | `bevy_utils` | `bevy_log`         |
| `info`                 | `bevy_utils` | `bevy_log`         |
| `warn`                 | `bevy_utils` | `bevy_log`         |
| `all_tuples`           | `bevy_utils` | `variadics_please` |
| `all_tuples_with_size` | `bevy_utils` | `variadics_please` |
| `debug_once`           | `bevy_utils` | `bevy_log`         |
| `detailed_trace`       | `bevy_utils` | _Removed_          |
| `error_once`           | `bevy_utils` | `bevy_log`         |
| `info_once`            | `bevy_utils` | `bevy_log`         |
| `trace_once`           | `bevy_utils` | `bevy_log`         |
| `warn_once`            | `bevy_utils` | `bevy_log`         |

**Functions**

| Item           | 0.15 Path             | 0.16 Path             |
| -------------- | --------------------- | --------------------- |
| `check_ready`  | `bevy_utils::futures` | `bevy_tasks::futures` |
| `now_or_never` | `bevy_utils::futures` | `bevy_tasks::futures` |

**Type Aliases**

| Item            | 0.15 Path    | 0.16 Path                                      |
| --------------- | ------------ | ---------------------------------------------- |
| `BoxedFuture`   | `bevy_utils` | `bevy_tasks`                                   |
| `Entry`         | `bevy_utils` | `bevy_platform_support::collections::hash_map` |
| `HashMap`       | `bevy_utils` | `bevy_platform_support::collections`           |
| `HashSet`       | `bevy_utils` | `bevy_platform_support::collections`           |
| `StableHashMap` | `bevy_utils` | _Removed_                                      |
| `StableHashSet` | `bevy_utils` | _Removed_                                      |

**Removed Items**

- `assert_object_safe` was removed in part because the term is now outdated (replaced with _dyn compatibility_) and otherwise because it is trivial to inline.
  
  ```rust
  // Before
  const _: () = assert_object_safe::<dyn MyTrait>();

  // After
  const _: Option<Box<dyn MyTrait>> = None;
  ```
  
- `hashbrown` was removed from `bevy_utils` as a re-export due to its significant API change from `hashbrown` 0.14 to 0.15.
  Instead of exposing a large public API out of our direct control, we've taken a more explicit subset and moved it into `bevy_platform_support::collections`, mimicking the layout of the standard library.
  If you need access to `hashbrown`, take a direct dependency instead.

- `detailed_trace` was removed due to its minimal use within the engine.
  If you still wish to use it, make sure you have taken a direct dependency on `tracing` and have a feature name `detailed_trace` defined in your `Cargo.toml`.
  You can use the below as a replacement:

  ```rust
  macro_rules! detailed_trace {
      ($($tts:tt)*) => {
          if cfg!(feature = "detailed_trace") {
              ::tracing::trace!($($tts)*);
          }
      }
  }
  ```
  
- `dbg`, `info`, `warn`, and `error` were all removed due to minimal use within the engine.
  If you still wish to use them, make sure you have taken a direct dependency on `tracing`.
  You can use the below as a replacement:

  ```rust
  /// Calls the [`tracing::info!`] macro on a value.
  pub fn info<T: core::fmt::Debug>(data: T) {
      ::tracing::info!("{:?}", data);
  }

  /// Calls the [`tracing::debug!`] macro on a value.
  pub fn dbg<T: core::fmt::Debug>(data: T) {
      ::tracing::debug!("{:?}", data);
  }

  /// Processes a [`Result`] by calling the [`tracing::warn!`] macro in case of an [`Err`] value.
  pub fn warn<E: core::fmt::Debug>(result: Result<(), E>) {
      if let Err(warn) = result {
          ::tracing::warn!("{:?}", warn);
      }
  }

  /// Processes a [`Result`] by calling the [`tracing::error!`] macro in case of an [`Err`] value.
  pub fn error<E: core::fmt::Debug>(result: Result<(), E>) {
      if let Err(error) = result {
          ::tracing::error!("{:?}", error);
      }
  }
  ```
  
- `StableHashMap` and `StableHashSet` were removed due to minimal use within the engine.
  You can use the below as a replacement:
  
  ```rust
  /// A stable hash-map.
  pub type StableHashMap<K, V> = bevy::platform_support::collections::HashMap<K, V, bevy::platform_support::hash::FixedState>;

  /// A stable hash-set.
  pub type StableHashSet<K> = bevy::platform_support::collections::HashSet<K, bevy::platform_support::hash::FixedState>;
  ```

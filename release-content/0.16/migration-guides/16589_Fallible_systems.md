If you've written a custom executor, there are a few changes you will need to make in order to support fallible systems.

1. Many uses of `BoxedSystem<(), ()>` have been replaced with `ScheduleSystem`, which is a type alias to `BoxedSystem<(), Result>`.
2. Executors should obey the `SystemParamValidationError` returned by `SystemParam::validate_param()` in order to determine whether to raise an error or skip the system.
3. When an executor encounters an error, it should pass that error to `default_error_handler()`, whose behavior can be configured with the `GLOBAL_ERROR_HANDLER` static.

For more information on fallible systems, please read the module docs for `bevy::ecs::error`.

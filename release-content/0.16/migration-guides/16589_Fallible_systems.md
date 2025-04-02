Users who have implemented their own custom executor should use `ScheduleSystem` in place of `BoxedSystem<(), ()>` and import the `System` trait where needed. 

Custom executors should:

- obey the `SystemParamValidationError` returned by `SystemParam::validate_param`, skipping systems as requested
- use the `default_error_handler` for both validation errors and results returned from systems

See the source code of the first-party Bevy schedule executors for more details.

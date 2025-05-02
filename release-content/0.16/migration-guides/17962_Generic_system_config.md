In order to reduce internal duplication between scheduling systems and system sets, the new generic `ScheduleConfigs<T>` type and `IntoScheduleConfigs<T>` trait have been added. These take a generic parameter, `T`, that may be `ScheduleSystem` (for systems) or `InternedSystemSet` (for system sets).

|0.15 Item|0.16 Item|
|-|-|
|`SystemConfigs`|`ScheduleConfigs<ScheduleSystem>`|
|`SystemSetConfigs`|`ScheduleConfigs<InternedSystemSet>`|
|`IntoSystemConfigs`|`IntoScheduleConfigs<ScheduleSystem, M>`|
|`IntoSystemSetConfigs`|`IntoScheduleConfigs<InternedSystemSet, M>`|

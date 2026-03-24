+++
title = "Skipping Systems"
insert_anchor_links = "right"
[extra]
weight = 1
status = 'hidden'
+++

Instead of returning early from a system, you could choose to skip it entirely.
This can reduce the complexity of using your systems by separating concerns of _"what a system does"_ from _"should it run"_.
Skipping a system entirely can also reduce the overhead involved since it would be cancelled before being dispatched.
Systems are run in parallel by default, so skipping a system means your application would do less work overall if a system is skipped.

Bevy offers two complementary ways to skip systems: **run conditions** and **fallible system parameters**.

## Run Conditions

Run conditions are functions that can be attached to systems via the [`.run_if()`] method at the time that they are added to a schedule.
More specifically, any read-only system which returns `bool` can be used as a run condition.
This allows you to access information about the [`World`] and even store local state when determining if a system should be run.

For your convenience, Bevy comes with a set of premade run conditions, which can be found by searching for [common conditions] in the Bevy docs.
These run conditions are domain-specific, thus they are defined as needed by various Bevy subcrates.
They might allow you to [only run a system based on input being received], [run a system on a timer], or [run a system when in a certain game state].

Most of these premade run conditions won't run on their own.
Instead you'll have to provide them with a value which will then return a function that can be then be used as a run condition.

Run conditions can also be composed.
You can chain multiple `.run_if()` calls using AND logic, but there are also various boolean operations provided by the [`SystemCondition`] trait.

Run conditions can also be applied to multiple systems at once by using a [`SystemSet`].
If a system set is given a run condition, it will be evaluated once per frame for that system set and the systems inside of it will be skipped if it returns true.

As a result, run conditions can be useful when working with systems that you do not own.
They can be used to configure if (or under what conditions) these foreign systems should run as long as the crate author has publicly exposed either the system function or a system set that contains it.

[`.run_if()`]: https://docs.rs/bevy/latest/bevy/prelude/trait.IntoScheduleConfigs.html#method.run_if
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
[common conditions]: https://docs.rs/bevy/latest/bevy/prelude/index.html?search=common_conditions
[only run a system based on input being received]: https://docs.rs/bevy/latest/bevy/input/common_conditions/index.html
[run a system on a timer]: https://docs.rs/bevy/latest/bevy/time/common_conditions/index.html
[run a system when in a certain game state]: https://docs.rs/bevy/latest/bevy/ecs/schedule/common_conditions/index.html
[`SystemCondition`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/trait.SystemCondition.html
[`SystemSet`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/trait.SystemSet.html

## Fallible System Parameters

Every system parameter requires that some specific data is available in the `World`.
For example, `Res<T>` requires that `T` has been initialized as a resource.
If that data is not found, [`SystemParam`] authors can choose how to proceed:

- The system can fail and cause a panic,
- Or the system can be gracefully skipped.

You aren't locked into either panicking or skipping though.
When system params fail the resulting behavior can be configured (as detailed in the [handling errors] section), but by default failed system parameters will cause a panic.

Alternatively, system params can cause the system that they are part of to be skipped when validation fails.
This represents an unusual but expected state of the application, which should be handled silently without issue.

For example, the [`Single`] system parameter works like [`Query`] except that it only succeeds if the query matches exactly one entity.
If only a single entity matches, the returned value is provided and conveniently unwrapped.
If either zero or more than one entity matches, the system is not run.

This logic can be useful when, for example, dealing with a player character who may have died.
We don't need to update their statistics while waiting for them to respawn, but we certainly don't want to panic!

If you are writing your own [`SystemParam`], this behavior can be configured via the [`SystemParam::validate_param`] method.

[Handling Errors]: ../handling-errors
[`SystemParam`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html
[`Single`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Single.html
[`Query`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Query.html
[`SystemParam::validate_param`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html#method.validate_param

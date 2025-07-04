+++
title = "Skipping systems"
insert_anchor_links = "right"
[extra]
weight = 1
status = 'hidden'
+++

Instead of simply returning early from a system, you can simply skip it entirely.
This can simplify the logic of your systems, by separating concerns of "what a system does" from "should it run".

In addition, because systems are run in parallel, skipping a system entirely can reduce the overhead involved,
as they are cancelled before they are dispatched.

Bevy offers two complementary ways to skip systems: **run conditions** and **fallible system parameters**.

## Run conditions

Run conditions are functions that can be attached to systems via `.run_if` at the time that they are added to a schedule.
Any read-only system which returns `bool` can be used as a run condition,
allowing you to access information about the [`World`] and even store local state.

For your convenience, Bevy comes with a set of premade run conditions,
which can be found by searching for [common conditions] in the Bevy docs.
Because these are domain-specific, they are defined as needed by various Bevy subcrates.
They might allow you to [only run a system when a button is pressed],
[run a system on a timer], or [run a system only when in a certain game state].

Most of these methods are not run conditions themselves:
instead, they work by accepting a value, and then returning a function that can be used as a run condition.

Run conditions can be composed: by default, chaining multiple `.run_if` calls are composed via AND logic,
but various boolean operations are provided by the [`Condition`] trait.

Run conditions can also be applied to multiple systems at once, using [system sets].
If a system set is given a run condition, it will be evaluated once per frame for that system set,
and the systems inside of it will be skipped if it returns true.

As a result, run conditions can be useful when working with systems that you do not own:
they can be used to configure if (or under what conditions) these systems run
as long as the crate author has publicly exposed either the system function or a system set that contains it.

## Fallible system parameters

Every system parameter requires specific data:
for example, `Res<T>` requires that `T` has been initialized as a resource.

If that data is not found, [`SystemParam`] authors have a choice for how to proceed:
they can either fail, or be gracefully skipped.

The behavior when system params fail can be configured,
as detailed in the [handling errors] section of this chapter,
but by default, failed system parameters will cause a panic.

Alternatively, system params can cause the system that they are part of to be skipped when validation fails.
This represents an unusual but expected state of the application, which should be handled silently without issue.

For example, the [`Single`] system parameter works like [`Query`],
except that it only succeeds if the query matches exactly one entity,
conveniently unwrapping it for you.
If either zero or more than one entity matches, the system is not run.

This logic can be useful when, for example, dealing with a player character,
who may have died.
We don't need to update their statistics while waiting for them to respawn,
but we certainly don't want to panic!

If you are writing your own [`SystemParam`], this behavior can be configured via the [`SystemParam::validate_param`] method.

+++
title = "Hooks"
insert_anchor_links = "right"
[extra]
weight = 11
status = 'hidden'
+++

<!-- TBW -->

In the previous chapter we learned about `Events` and how they allow us to run code in the `World` or on a specific `Entity` in response to a trigger condition. We can extend this concept by using *lifecycle events* to run code whenever we add, remove, or alter a `Component` from or on an `Entity`. These are ran in response to a `Component` being modified in some way, and can be accessed mainly through [`ComponentHooks`] (simply referred to as **Hooks** from here on out).



[`ComponentHooks`]: https://docs.rs/bevy/latest/bevy/ecs/lifecycle/struct.ComponentHooks.html

## `ComponentHooks`

`ComponentHooks` are *lifecycle events*, events that occur when a `Component` is *added to*, *inserted in*, *replaced on*, or *removed from* an `Entity`, or when a `Component` is *despawned*, like when an `Entity` is removed from the `World`. Specifically these are all `EntityEvents`, meaning that they will carry an `entity` field which holds the `EntityID` of the `Entity` being targetted. 

We can split these five lifecycle events into two categories: lifecycle events that trigger when a `Component` is *added* to an `Entity`, and lifecycle events that trigger when a `Component` is *removed* from an `Entity`.

Adding a `Component`:
- [`Add`] triggers when a component is added to an `Entity` *that did not already have it*.
- [`Insert`] triggers when a component is added to an `Entity`, *regardless of whether it already had it or not*.

When both `Add` and `Insert` occur, `Add` hooks are evaluated before `Insert` hooks.

Removing a `Component`:
- [`Replace`] triggers when a component is removed from an `Entity`, *regardless of if it is replaced with a new value*.
- [`Remove`] triggers when a component is removed from an `Entity` *and not replaced*. (This also happens before the component is actually removed.)
- [`Despawn`] triggered on *each* component on an `Entity` when the `Entity` is *despawned*.

`Replace` hooks are evaluated before `Remove` hooks, and `Despawn` hooks are evaluated last.

[`Add`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Add.html
[`Insert`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Insert.html
[`Replace`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Replace.html
[`Remove`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Remove.html
[`Despawn`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Despawn.html

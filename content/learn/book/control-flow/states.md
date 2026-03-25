+++
title = "States"
insert_anchor_links = "right"
[extra]
weight = 2
+++

Virtually all games are presented as a series of interactive "modes".
Take for example a classic arcade game such as _Space Invaders_ or _Pac-Man_:

- First you have an "intro" mode, which catches the player's interest and teaches them about how the game works.
- Then we enter the "play" mode, where the game actually begins and the user is presented with various interactive challenges.
- Finally at some point we'll have a "game end" mode, where the game level ends and the player will be presented with a "Congratulations, you Won!" or "Sorry, you Lost!" screen.

Each of these modes entails a different set of graphics as well as a different configuration of input controls.
During the intro mode (for example) all you can do is start the game, or perhaps get help, or maybe even view the game's credits screen.
You can't start controlling your ship or character until after you have left the "intro" mode and entered "play" mode.

How should we organize all of these different modal states in code?
We could have separate variables for each of the modes and write a bunch of logic for all the various entities involved, but there's an easier way: **game states**.

Bevy's [`States`] trait lets you define one or more [finite-state machines](https://en.wikipedia.org/wiki/Finite-state_machine) (FSMs) that can be used to orchestrate changes to the Bevy world.
You can configure your ECS systems to only run during certain states, or only when transitioning between certain states.
You can create entities that will only exist within specific states as well.
For example, you might have a HUD (heads-up display) entity which only exists during "play" mode.

[`States`]: https://docs.rs/bevy/latest/bevy/state/state/trait.States.html

## Setting Up States

Let's say that we're building a retro arcade game like [Galaga](https://en.wikipedia.org/wiki/Galaga).
We'll want at least three states:

```rust,hide_lines=1-2
# use bevy::ecs::prelude::*;
#
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    /// The introductory screen
    #[default]
    Intro,
    /// The main play mode
    Playing,
    /// Finished a level; display the accumulated score
    LevelComplete,
}
```

Here we derive from the `States` trait to define an enum with three variants, representing our three states.
The `Default` defines the starting state.

{% callout(type="info") %}
The `States` trait is used to define _global_ states: that is, states that have influence over the entire Bevy `World`.
However, while `States` models a finite-state machine, it is not the only possible use for FSMs within games.
Animation and character behavior are also frequently modeled as FSMs, but since these only control a single entity, they are outside of the scope of what the `States` trait is intended for.
{% end %}

To use `GameState` we'll first need to register it with the `App`:

```rust
app.init_state::<GameState>();
```

You can define more than one set of states and have multiple state sets running in your application at a given time.
The different state sets will be independent of each other, however we can also indicate a set of states that exist within another set of states. These are sub-states, and we'll dive into them in next.

## Switching Between States

To trigger a transition between states, update the [`NextState<T>`] resource with the state you want to transition to.

In our example arcade game, we start in the `Intro` state.
When the player indicates they are ready to begin, we transition to the `Playing` state.
If the player clears the level, we go to the `LevelComplete` state.
This state only lasts a few seconds, at which point the next level begins (which means we go back to the `Playing` state).
We continue to alternate between `Playing` and `LevelComplete` states until the player runs out of lives.

[`NextState<T>`]: https://docs.rs/bevy/latest/bevy/state/state/enum.NextState.html

## States Control When Systems Run

While you can check the value of the [`State<T>`] resource during a system, their primary value lies in controlling when and if systems are run.

For example, say we only want enemy units to move and attack while in the `Playing` state:

```rs
// Only run the enemy behavior while playing
app.add_systems(Update, update_enemies.run_if(in_state(GameState::Playing)));
```

The [`in_state`] function is a run condition which evaluates to `true` if we are in that state.

We can also configure systems to run when entering or exiting a state, using the [`OnEnter`] and [`OnExit`] schedules.
For example, we might want to play a sound when we begin a new level:

```rs
// Runs the `start_level_sound` system when we enter the `Playing` state.
app.add_systems(OnEnter(GameState::Playing), start_level_sound);
```

[`State<T>`]: https://docs.rs/bevy/latest/bevy/state/state/struct.State.html
[`in_state`]: https://docs.rs/bevy/latest/bevy/prelude/fn.in_state.html
[`OnEnter`]: https://docs.rs/bevy/latest/bevy/prelude/struct.OnEnter.html
[`OnExit`]: https://docs.rs/bevy/latest/bevy/prelude/struct.OnExit.html

## Digging in to State Transitions

All state transitions occur during the [`StateTransition`] schedule.
[`OnExit`] schedules run as the given state is left, and [`OnEnter`] schedules run just as they are entered.

The `StateTransition` schedule itself runs at two points:

1. **During app startup**, after [`PreStartup`] but before [`Startup`]. This is when your initial states' [`OnEnter`] systems run.
2. **Each tick of the game loop**, after [`PreUpdate`] but before the fixed update loop and [`Update`].

When you set a new state with [`NextState<T>`], the transition doesn't happen immediately.
Instead the change is queued and applied the next time `StateTransition` runs.
This means that systems which will run later in the _same_ tick will still see the _old_ state.

When a transition does occur, the schedules run in this order:

1. [`OnExit`] for the old state
2. [`OnTransition`] for the transition
3. [`OnEnter`] for the new state

Every transition also emits a [`StateTransitionEvent<S>`], which you can read via a [`MessageReader`] to respond to specific transition edges.

For more details on where `StateTransition` fits into the broader game loop, see the [schedules] chapter.

[`StateTransitionEvent<S>`]: https://docs.rs/bevy/latest/bevy/state/state/struct.StateTransitionEvent.html
[`MessageReader`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.MessageReader.html
[`StateTransition`]: https://docs.rs/bevy/latest/bevy/state/state/struct.StateTransition.html
[`OnTransition`]: https://docs.rs/bevy/latest/bevy/prelude/struct.OnTransition.html
[`PreStartup`]: https://docs.rs/bevy/latest/bevy/app/struct.PreStartup.html
[`Startup`]: https://docs.rs/bevy/latest/bevy/app/struct.Startup.html
[`PreUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PreUpdate.html
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html
[schedules]: /learn/book/the-game-loop/schedules

## Cleaning Up Between States

One of the most common patterns when working with states is to run some setup logic upon entering a state, and some tear-down or clean-up logic when exiting a state.
The most common form of clean-up is despawning entities when the state they're associated with ends. 

To make this easier, the [`DespawnOnExit`] component can be added to an entity to indicate that the entity should only exist in a particular state.
When we exit that state, the entity will automatically be despawned.

For example, if we wanted to despawn all remaining enemies at the end of a level:

```rs
fn spawn_enemy(mut commands: Commands) {
    commands.spawn((
        Enemy,
        DespawnOnExit(GameState::Playing),
    ));
}
```

The same pattern can be very helpful for UI as well.
We can automatically close menus by despawning them when their associated state ends.
This allows us to automatically couple the "remember to clean this up" logic with the creation of our objects, rather than needing to remember all of the things we might have spawned in a single monolithic cleanup system.

Similar helper components exist: [`DespawnOnEnter`], for when you want to clean up when entering a specific state, and [`DespawnWhen`], for when you want to perform more complex state-matching logic.

[`DespawnOnExit`]: https://docs.rs/bevy/latest/bevy/prelude/struct.DespawnOnExit.html
[`DespawnOnEnter`]: https://docs.rs/bevy/latest/bevy/prelude/struct.DespawnOnExit.html
[`DespawnWhen`]: https://docs.rs/bevy/latest/bevy/prelude/struct.DespawnWhen.html

## SubStates: States Within States

For our arcade game, the `GameState::Playing` state is actually more complex than it appears.
When a level starts, the action doesn't begin immediately - instead there is a brief interval where we display an animation of the ship arriving, or "warping in".
Similarly, when the player's ship is destroyed, we show an animation of the ship exploding in all its glory.
During these times, the player is unable to move their ship or fire, but other entities, such as enemies, terrain, or the HUD are able to function normally.
We may also want to have a special "pause" state which freezes both the player and enemies, but which does not interrupt other aspects of the game like background music.

We can model this as a **sub-state** of `GameState::Playing`, meaning that these states only exist while we are in the "playing" state.

```rust,hide_lines=1-2
# use bevy::ecs::prelude::*;
#
#[derive(SubStates, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::Playing)]
pub enum ActionState {
    /// Player ship is warping in (also happens after respawn).
    #[default]
    Arriving,
    /// Player ship is under player control
    Running,
    /// User has paused the game
    Paused,
    /// Death animation, followed by either respawn or game over.
    Dying,
}
```

Sub-states are initialized just like top-level states:

```rust
app
    .init_state::<GameState>()
    .init_state::<ActionState>();
```

When state transitions occur, sub-states and computed states are recomputed accordingly, causing these transitions to cascade.
If we change `GameState`, causing `ActionState` to be removed, the `OnExit` schedule for the old `ActionState` value will run as well.

{% callout(type="info") %}
It might be reasonable to ask why we chose to model these different modes as sub-states: why not just have a single flat list of all the states?
One reason is that all of these action states have a lot in common: they all have a HUD, they all have enemies which need to be spawned, and so on.
By making them sub-states, we can tie the existence of, say, the HUD to the top-level state, while using the sub-states to control things like enemy and player movement.
{% end %}

## Computed States: Automatically-Derived States

Sometimes you want a state that you can't directly change, but that is instead *derived* from one or more other states.

In our arcade game, many systems — the HUD, the score display, maybe even player movement — need to run during *both* `Playing` and `LevelComplete`, but not during `Intro`.
We could add `.run_if(in_state(GameState::Playing).or(in_state(GameState::LevelComplete)))` to each of those systems, but that's repetitive and fragile: every time we add a new `GameState` variant, we'd have to update every run condition.

Instead, we can define a **computed state** called `InGame` that automatically exists whenever the game is in any "active" state.
To do so, we must implement the [`ComputedStates`] trait:

```rust,hide_lines=1
# use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState { 
    #[default]
    Intro,
    Playing,
    LevelComplete
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct InGame;

impl ComputedStates for InGame {
    type SourceStates = GameState;

    fn compute(sources: GameState) -> Option<Self> {
        match sources {
            GameState::Intro => None,
            _ => Some(InGame),
        }
    }
}
```

The `compute` function receives the current value of the source state and returns `Option<Self>`:

- Returning `Some` means the computed state should be active with that value.
- Returning `None` means the computed state should not exist at all.

This means computed states can appear and disappear automatically.
`InGame` exists during `Playing` and `LevelComplete`, but during `Intro` the [`State<InGame>`][`State<T>`] resource simply isn't present in the world.

Computed states are registered with [`add_computed_state`]:

```rust
app
    .init_state::<GameState>()
    .add_computed_state::<InGame>();
```

Now all of our shared in-game systems can use a single, clear run condition:

```rs
app.add_systems(Update, (update_hud, update_score, spawn_enemies).run_if(in_state(InGame)));
```

If we later add new `GameState` variants like `Cutscene` or `Tutorial`, we only need to update the `compute` function in one place.

{% callout(type="info") %}

Unlike sub-states, computed states are **read-only**: you cannot set them via [`NextState<T>`].
Their value is entirely determined by their `compute` function.
If you need a derived state that you can also manually override, use a sub-state instead.

{% end %}

[`add_computed_state`]: https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html#method.add_computed_state
[`ComputedStates`]: https://docs.rs/bevy/latest/bevy/state/state/trait.ComputedStates.html

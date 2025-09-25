+++
title = "States"
insert_anchor_links = "right"
[extra]
weight = 2
+++

Virtually all games are presented as a series of interactive "modes". Take for example a classic
arcade game such as _Space Invaders_ or _Pac-Man_: First you have an "intro" mode, which catches the
player's interest and teaches them about how the game works. The competition doesn't actually start until
we enter the "play" mode, where the user is presented with various interactive challenges. At some
point the game level will end, and the player will be presented with a "Congratulations, you Won!"
or "Sorry, you Lost!" screen.

Each of these modes entails a different set of graphics as well as a different configuration
of input controls. During the intro mode, for example, all you can do is start the game, or perhaps
get help or view the game's credits screen. You can't start controlling your ship or character
until after you have left the "intro" mode and entered "play" mode.

How shall we organize all of these different modal states in code? We could have separate variables for each
of the modes, and write a bunch of logic for all the various entities involved, but there's an
easier way: _game states_.

Bevy's [`States`] trait lets you define one or more [finite-state
machines](https://en.wikipedia.org/wiki/Finite-state_machine) (FSMs) that can be used to orchestrate
changes to the Bevy world. You can configure your ECS systems to only run during certain states, or
only on when transitioning between certain states. You can create entities that only exist
within particular states as well. For example, you might have a HUD (heads-up display) entity which
only exists during "play" mode.

## Defining States

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

Here we derive from the `States` trait to define an enum with three variants, representing our
three states. The `Default` defines the starting state.

{% callout(type="info") %}
The `States` trait is used to define _global_ states: that is, states that have influence over
the entire Bevy `World`. However, while `States` models a finite-state machine, it is not the
only possible use for FSMs within games. Animation and character behavior are also frequently
modeled as FSMs, but since these only control a single entity, they are outside of the scope of what
the `States` trait is intended for.
{% end %}

To use `GameState` we'll first need to register it with the `App`:

```rust
app.init_state::<GameState>();
```

You can define more than one set of states; these are independent (but see sub-states, below).

## SubStates: States within States

For our arcade game, the `GameState::Playing` state is actually more complex than it appears.
When a level starts, the action doesn't begin immediately - instead there is a brief
interval where we display an animation of the ship arriving, or "warping in". Similarly, when
the player's ship is destroyed, we show an animation of the ship exploding in all its glory.
During these times, the player is unable to move their ship or fire, but other entities, such
as enemies, terrain, or the HUD are able to function normally. We may also want to have a special
"pause" state which freezes both the player and enemies, but which does not interrupt other
aspects of the game like background music.

We can model this as a _sub-state_ of `GameState::Playing`, meaning that these states only exist
while we are in the "playing" state.

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

{% callout(type="info") %} It might be reasonable to ask why we chose to model these different modes
as sub-states: why not just have a single flat list of all the states? One reason is that all of
these action states have a lot in common: they all have a HUD, they all have enemies which need to
be spawned, and so on. By making them sub-states, we can tie the existence of, say, the HUD to the
top-level state, while using the sub-states to control things like enemy and player movement. {% end
%}

## Switching between States

You can access the current state of type `T` with the `State<T>` resource. To trigger a transition
between states, update the `NextState<T>` resource with the state you want to transition to.

In our example arcade game, we start in the `Intro` state. When the player indicates they are ready to
begin, we transition to the `Playing` state. If the player clears the level, we go to the
`LevelComplete` state. This state only lasts a few seconds, at which point the next level begins -
which means we go back to the `Playing` state. We continue to alternate between `Playing` and
`LevelComplete` states until the player runs out of lives.

(TBW: Example that transitions to "Playing" when a button is pressed).

## States and Systems

States can be used to determine which ECS systems get run. For example, say we only want enemy units
to move and attack while in the `Playing` state:

```rs
// Only run the enemy behavior while playing
app.add_systems(Update, update_enemies.run_if(in_state(GameState::Playing)));
```

The `in_state` function is a run condition which evaluates to `true` if we are in that state.

We can also configure systems to run when entering or exiting a state, using `OnEnter` and
`OnExit`. For example, we might want to play a sound when we begin a new level:

```rs
// Runs the `start_level_sound` system when we enter the `Playing` state.
app.add_systems(OnEnter(GameState::Playing), start_level_sound);
```

## States and Entities

The `StateScoped` component can be added to an entity to indicate that the entity should only
exist in a particular state. When we exit that state, the entity will automatically be despawned.

For example, if we wanted to despawn all remaining enemies at the end of a level:

```rs
fn spawn_enemy(mut commands: Commands) {
    commands.spawn((
        Enemy,
        StateScoped(GameState::Playing),
    ));
}
```

## ComputedState

<!-- TBW -->

## States and Schedules

<!-- TBW -->

Bevy's `States` are a simple but powerful abstraction for managing the control flow of your app.

```rust
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
 #[default]
  Menu,
  InGame,
}

fn handle_escape_pressed(mut next_state: ResMut<NextState<GameState>>) {
    // Get keyboard input...

    if escape_pressed {
        next_state.set(GameState::Menu);
    }
}

fn open_menu() {
    // Spawn the settings menu...
}

App::new()
    .init_state::<GameState>()
    .add_systems(Update, handle_escape_pressed.run_if(in_state(GameState::InGame)));
    .add_systems(OnEnter(GameState::Menu), open_settings_menu);
```

But as users' games (and non-game applications!) grew in complexity, their limitations became more apparent.
What happens if we want to capture the notion of "in a menu", but then have different states corresponding to which submenu should be open?
What if we want to ask questions like "is the game paused", but that question only makes sense while we're within a game?

Finding a good abstraction for this required [several](https://github.com/bevyengine/bevy/pull/9957) [attempts](https://github.com/bevyengine/bevy/pull/10088) and a great deal of both experimentation and discussion.

While your existing states code will work exactly as before, there are now two additional tools you can reach for if you're looking for more expressiveness: **computed states** and **sub states**.

```rust
/// Let's begin with our simple state above.
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum GameState {
    #[default]
    Menu,
    // The addition of `pause` field means that simply checking for `GameState::InGame` doesn't work:
    // the states are different depending on its value and we may want to distinguish between game systems
    // that run when the game is paused or not!
    InGame {
        paused: bool
    },
}

// While we can simply do `OnEnter(GameState::InGame{paused: true})`,
// we need to be able to reason about "while we're in the game, paused or not".
// To this end, we define the `InGame` computed state.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct InGame;

impl ComputedStates for InGame {
    // Computed states can be calculated from one or many source states.
    type SourceStates = GameState;

    // Now, we define the rule that determines the value of our computed state.
    fn compute(sources: GameState) -> Option<InGame> {
        match sources {
            // We can use pattern matching to express the
            //"I don't care whether or not the game is paused" logic!
            GameState::InGame {..} => Some(InGame),
            _ => None,
        }
    }
}

// In contrast, substates should be used when you want to keep manual
// control over the value through `NextState`, but still bind their
// existence to some parent state.
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
// This macro means that `GamePhase` will only exist when we're in the `InGame` computed state.
// The intermediate computed state is helpful for clarity here, but isn't required:
// you can manually `impl SubStates` for more control, multiple parent states and non-default initial value!
#[source(InGame = InGame)]
enum GamePhase {
    #[default]
    Setup,
    Battle,
    Conclusion
}

// Initializing our states is easy: just call the appropriate method on `App`
// and all of the required machinery will be set up for you.
App::new()
   .init_state::<GameState>()
   .add_computed_state::<InGame>()
   .add_substate::<GamePhase>()
```

Just like any other state, computed states and substates work with all of the tools you're used to:
the `State` and `NextState` resources, `OnEnter`, `OnExit` and `OnTransition` schedules and the `in_state` run condition.
Make sure to visit [both](https://github.com/bevyengine/bevy/blob/main/examples/state/computed_states.rs) [examples](https://github.com/bevyengine/bevy/blob/main/examples/state/sub_states.rs) for more information!

The only exception is that, for correctness, computed states *cannot* be mutated through `NextState`.
Instead, they are strictly derived from their parent states; added, removed and updated automatically during state transitions based on the provided `compute` method.

All of Bevy's state tools are now found in a dedicated `bevy_state` crate, which can be controlled via a feature flag.
Yearning for the days of state stacks? Wish that there was a method for re-entering states?
All of the state machinery relies *only* on public ECS tools: resources, schedules and run conditions.
We know that state machines are very much a matter of taste; so if our design isn't to your taste consider taking advantage of Bevy's modularity and writing your own abstraction or using one supplied by the community!

In Bevy 0.13, the [`OnEnter`] schedules for states initialized via [`app.init_state`] would run after any systems in the `Startup` schedules.
This is because [`apply_state_transitions`] was only run during the [`StateTransition`] schedule.

This was a subtle bug: it was possible for the game to be in a particular state without having first *entered* it.
Now, [`OnEnter`] state transition logic is handled immediately.
See [bevy#13968](https://github.com/bevyengine/bevy/issues/13968) for more context on this decision.

To migrate, choose one of the following options:

1. Moving your startup systems to a state, as a variant of the state you're waiting for (e.g. `AppState::Setup`), and then transition out of it once the setup is complete.
2. Moving your startup systems to a state, and making the other state a [sub state](https://github.com/bevyengine/bevy/blob/v0.14.0-rc.4/examples/state/sub_states.rs) that depends on the startup state's completion (e.g. `SetupState::SetupComplete`).

Bevy 0.13:

```rust
#[derive(States, Default)]
enum AppState {
    #[default]
    InMenu,
    InGame,
}

app
   .init_state::<AppState>()
   .add_systems(Startup, initial_setup)
   .add_systems(OnEnter(AppState::InMenu), relies_on_initial_setup);
```

Bevy 0.14 (solution 1):

```rust
#[derive(States, Default)]
enum AppState {
    #[default]
    Setup
    InMenu,
    InGame,
}

fn transition_to_in_menu(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InMenu);
}

app
    .init_state::<AppState>()
    .add_systems(OnEnter(AppState::Setup), initial_setup)
    .add_system(Update, transition_to_in_menu.run_if(in_state(AppState::Setup)))
    .add_systems(OnEnter(AppState::InMenu), relies_on_initial_setup);
```

Bevy 0.14 (solution 2):

```rust
#[derive(States, Default)]
enum SetupState {
    #[default]
    SettingUp,
    SetupComplete,
}

#[derive(SubStates, Default)]
#[source(SetupState = SetupState::SetupComplete)]
enum AppState {
    #[default]
    InMenu,
    InGame,
}

fn finish_setup(mut app_state: ResMut<NextState<SetupState>>) {
    app_state.set(SetupState::SetupComplete);
}


app
    .init_state::<SetupState>()
    // Note that we don't call `init_state` for substates!
    .add_sub_state::<AppState>()
    .add_systems(OnEnter(AppState::InitialSetup), initial_setup)
    .add_system(Update, finish_setup.run_if(in_state(AppState::Setup)))
    .add_systems(OnEnter(AppState::InMenu), relies_on_initial_setup);
```

State scoped entities is a pattern that naturally emerged in community projects and has finally been integrated into Bevy codebase.

```rust
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
 #[default]
  Menu,
  InGame,
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        // We mark out entity with the `StateScoped` component.
        // When the provided state is exited, the entity will be deleted recursively with all chilren.
        StateScoped(GameState::InGame)
        SpriteBundle { ... }
    ))
}

App::new()
    .init_state::<GameState>()
    .enable_state_scoped_entities::<GameState>()
    .add_systems(OnEnter(GameState::InGame), spawn_player);
```

By binding entity lifetime to a state during setup, we can dramatically reduce the amount of cleanup code we have to write!

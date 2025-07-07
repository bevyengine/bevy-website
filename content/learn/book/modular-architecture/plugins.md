+++
title = "Plugins"
insert_anchor_links = "right"
[extra]
weight = 0
+++

**Plugins** are the most important tool for organizing your Bevy projects,
allowing you to split logic across files, easily toggle behavior for testing,
and reuse functionality between projects.

At its heart, the `Plugin` trait is extremely simple.
It looks a little something like this:

```rust
trait Plugin {
    fn build(&self, app: &mut App) {}
}
```

Let's break that down:

- this is a trait, so we need to implement it for a user-defined type
- `build` takes `&self`, allowing you to change behavior based on the value of our type
- it takes a`&mut App` reference, allowing you to mutate the [`App`] state freely
  - adding systems via [`App::add_systems`] is the most common and important method
  - as discussed in our section on [apps], [`App`] holds a [`World`], allowing you to add resources, make queries, spawn entities and more
- because of how simple this is, we can just cut-and-paste code from our `main.rs` into plugins when we're ready to clean up

Let's see how this works in practice:

```rust
# use bevy::prelude::*;
#
# fn check_if_player_is_dead() {}
#
// We don't have any config, so a unit struct works fine
struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) -> &mut App {
        app.add_systems(Update, check_if_player_is_dead);
    }
}

App::new().add_plugins(PlayerPlugin);
```

Because our plugin is so simple (it doesn't take any config, and only uses the `build` method),
we could instead choose to use [the blanket for all functions that take a &mut App] instead:

```rust
# use bevy::prelude::*;
#
# fn check_if_player_is_dead() {}
#
fn player_plugin(app: &mut App){
    app.add_systems(Update, check_if_player_is_dead);
}

App::new().add_plugins(player_plugin);
```

The choice is mostly stylistic: the "fn plugin" style is a bit terser,
but the "struct plugin" style is easier to change if you find that your plugin needs
extra features in the future.

## Plugin groups

## The `Plugin` lifecycle

When a plugin is added though `App::add_plugins`, the app calls `Plugin::build`, and the plugin typically accesses and configures the world.  Then, when the app is run, a few other plugin life-cycle functions are called, and finally we enter the run loop:

- The app polls `Plugin::finished` until all the added plugins return `true`.
- The app calls `Plugin::finish` on all plugins.
- The app calls `Plugin::cleanup` an all plugins.
- The app calls the run loop function on itself.

[apps]: [../the-game-loop/app]
[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`App::add_systems`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html?search=add#method.add_systems
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html
[the blanket for all functions that take a &mut App]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html#impl-Plugin-for-T

+++
title = "Modularity through plugins"
weight = 3
template = "book-section.html"
page_template = "book-section.html"
+++

In Bevy, virtually all of the functionality you might want is added via **plugins**, which are added to your app.
These might provide basic windowing functionality, handle input or sound, calculate physics or provide game-specific logic like a tile map system.

By combining Bevy's first-party plugins with third-party alternatives, you can modularly customize the behaviour of the Bevy game engine, and then add your own plugins to contain your own game-specific code in a well-organized way.

## Writing your own plugins

Plugins are collections of code that modify the `AppBuilder` (which controls all of the data and logic of our game) using the builder pattern.
Any code in a plugin could be directly substituted directly on the base `AppBuilder`.
There's no magic to be found here; they're just a straightforward tool for code organization.

You can write your own to organize your own code by implementing the `Plugin` trait on a struct of your own creation.

```rust
use bevy::prelude::*;

fn main(){
 App::build()
  // As discussed below, DefaultPlugins provide the standard scaffolding for Bevy games
   .add_plugins(DefaultPlugins)
   // Plugins merely organize code: this could just be replaced directly with 
   // the init_resource and add_system calls from below
   .add_plugin(ScorePlugin)
   .run();
}

struct ScorePlugin;

impl Plugin for ScorePlugin {
  fn build(&self, app: &mut AppBuilder) {
     app
       // The Score struct is addded as a resource (global singleton) to the world, 
       // beginning at the default value of 0
       .init_resource::<Score>()
       // Increments the score by 1 every pass of the game loop
       .add_system(increment_score.system())
       // Prints the current value of the score
       .add_system(report_score.system());
  }
}

#[derive(Default, Debug)]
struct Score(u8);

fn increment_score(score: ResMut<Score>){
  score.0 += 1;
}

fn report_score(score: Res<Score>){
  dbg!(score);
}
```

## `DefaultPlugins`

Bevy's `DefaultPlugins` are intended to get up and running with a "standard game" as quickly as possible. Let's take a look at the [source](https://github.com/bevyengine/bevy/blob/latest/crates/bevy_internal/src/default_plugins.rs) again.

As you can see, there are some added niceties (logging and diagnostics, scenes, input and windowing) as well as a number of plugins controlled by feature flags.
These are heavier dependencies that you may want to disable for some games (or platforms), and replace with your own alternatives.
To do so, we can use [cargo features](https://doc.rust-lang.org/cargo/reference/features.html) to disable them in our game.

In your `Cargo.toml` file, disable default features and opt-in to the [features you want](https://github.com/bevyengine/bevy/blob/main/docs/cargo_features.md):

```toml
[dependencies]
bevy = {"0.5", , default-features = false, features = ["..."]}
```

As shown in the [plugin_group.rs](https://github.com/bevyengine/bevy/blob/latest/examples/app/plugin_group.rs) example, you can also configure plugin groups from within Bevy itself.

## `MinimalPlugins`

If you're looking to structure your Bevy app in an unusual way and don't want to use most of the functionality provided by the engine, you can choose to use  Bevy's [`MinimalPlugins`](https://docs.rs/bevy/latest/bevy/struct.MinimalPlugins.html) instead.

We can click through to the [source]((https://github.com/bevyengine/bevy/blob/latest/crates/bevy_internal/src/default_plugins.rs)) for the `impl PluginGroup for MinimalPlugins` to see that this adds [`CorePlugin`](https://docs.rs/bevy/latest/bevy/core/struct.CorePlugin.html) and [`ScheduleRunnerPlugin`](https://docs.rs/bevy/latest/bevy/app/struct.ScheduleRunnerPlugin.html).

The `CorePlugin` handles low-level fundamentals such as updating app time, while the `ScheduleRunnerPlugin` sets up the main game loop to run repeatedly over time.
This functionality is essential: starting with these plugins is virtually always going to be a safe bet.

## Third-party plugins

Importing 3rd-party plugins is easy; they're just ordinary Rust code that can be managed with `cargo`.
Bevy's modular nature tends to result in simple plug-and-play interoperability and easy extensibility, so don't be afraid to try out plugins that seem interesting or useful for your game.

1. Find a Bevy plugin (such as from our [collection of assets](https://bevyengine.org/assets/)).
2. Add it to your `Cargo.toml` as a crate under `[dependencies]`.
3. Import the code definitions from the crate (i.e. `using bevy_third_party::prelude::*`) to add the appropriate items to your workspace.
4. Add the plugin to your app!

Follow the documentation and examples from the crates you're importing to make sure you have everything configured properly.

If you plan on releasing a plugin yourself, please refer to [Bevy's Plugin Guidelines](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md) for guidance, [release a crate](https://doc.rust-lang.org/cargo/reference/publishing.html), and then [add it to Bevy Assets](https://github.com/bevyengine/bevy-assets/) to share it with the community!

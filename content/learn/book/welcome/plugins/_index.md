+++
title = "Plugins"
weight = 3
template = "book-section.html"
page_template = "book-section.html"
+++

One of Bevy's core principles is modularity. In Bevy, all functionality is implemented via [`Plugins`], which are added to an [`App`]. Game logic like player movement, core engine logic like rendering and sound, and third party extensions like tile maps are all implemented the same way using [`Plugins`].

This empowers Bevy developers to modularly "build their own engine" using official, third party, and custom [`Plugins`]. Bevy intentionally blurs the lines between engine developers and app developers.

## Writing your own plugins

Plugins are collections of code that modify [`Apps`].
Any code in a plugin could be directly applied to the base [`App`].
There's no magic to be found here; they're just a straightforward tool for code organization.

Plugins are types that implement the [`Plugin`] trait:

```no_run,rust
use bevy::prelude::*;

fn main(){
  App::new()
    // Adds the "default" Bevy Engine plugins.
    // We'll cover this in the next section.
    .add_plugins(DefaultPlugins)
    // Adds our new `Plugin` to the `App`
    .add_plugin(ScorePlugin)
    .run();
}

struct ScorePlugin;

impl Plugin for ScorePlugin {
  fn build(&self, app: &mut App) {
    app
      // The Score struct is added as a resource (global singleton) to the world, 
      // beginning at the default value of 0
      .init_resource::<Score>()
      // Increments the score by 1 every pass of the game loop
      .add_system(increment_score)
      // Prints the current value of the score
      .add_system(report_score);
  }
}

#[derive(Default, Debug)]
struct Score(u8);

fn increment_score(mut score: ResMut<Score>) {
  score.0 += 1;
}

fn report_score(score: Res<Score>) {
  info!("{}", score.0);
}
```

[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`Apps`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`Plugin`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html
[`Plugins`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html

## Plugin groups

Bevy's [`DefaultPlugins`] is a [`PluginGroup`] that adds the "core engine features" like rendering, windowing, and sound that most developers want when building an app.

You can add [`DefaultPlugins`] to your app like this:

```no_run,hide-lines=1-2,rust
use bevy::prelude::*;

App::new().add_plugins(DefaultPlugins);
```

Take a look at the [source](https://github.com/bevyengine/bevy/blob/latest/crates/bevy_internal/src/default_plugins.rs) to see a full list of what's included.

If you're looking to structure your Bevy app in an unusual way (for example, if you want to run it in a [headless fashion](https://github.com/bevyengine/bevy/blob/latest/examples/app/headless.rs)) and don't want to use most of the functionality provided by the engine, you can choose to use  Bevy's [`MinimalPlugins`] instead.

[`DefaultPlugins`]: https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html
[`PluginGroup`]: https://docs.rs/bevy/latest/bevy/app/trait.PluginGroup.html
[`MinimalPlugins`]: https://docs.rs/bevy/latest/bevy/struct.MinimalPlugins.html

## Third-party plugins

Importing 3rd-party plugins is easy; they're just ordinary Rust code that can be managed with [`cargo`](https://doc.rust-lang.org/cargo/).
Bevy's modular nature tends to result in simple plug-and-play interoperability and easy extensibility, so don't be afraid to try out plugins that seem interesting or useful for your game.

1. Find a Bevy plugin (such as from our [collection of assets](https://bevyengine.org/assets/)).
2. Add it to your `Cargo.toml` as a crate under `[dependencies]`.
3. Import the code definitions from the crate (i.e. `using bevy_third_party::prelude::*`) to add the appropriate items to your workspace.
4. Add the plugin to your app (i.e. `app.add_plugin(bevy_third_party_plugin)`)!

Follow the documentation and examples from the crates you're importing to make sure you have everything configured properly.

If you plan on releasing a plugin yourself, please refer to [Bevy's Plugin Guidelines](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md), [release a crate](https://doc.rust-lang.org/cargo/reference/publishing.html), and then [add it to Bevy Assets](https://github.com/bevyengine/bevy-assets/) to share it with the community!

## Pick-and-choose features

Some apps won't need all of the features provided by [`DefaultPlugins`]. Other features must be opted into. We can use [cargo features](https://doc.rust-lang.org/cargo/reference/features.html) to enable and disable what features are compiled into our game.

In your `Cargo.toml` file, you can disable default features and opt-in to the [features you want](https://github.com/bevyengine/bevy/blob/main/docs/cargo_features.md):

```toml
[dependencies]
bevy = { version = "0.5", default-features = false, features = ["feature_name"] }
```

As shown in the [`plugin_group.rs`](https://github.com/bevyengine/bevy/blob/latest/examples/app/plugin_group.rs) example, you can also configure plugin groups from within Bevy itself.

Many of Bevy's subcrates can also be used directly on their own and integrated with other engines or your own framework.
[`bevy_ecs`](https://crates.io/crates/bevy_ecs) is a particularly popular choice for this, allowing you to use our fast, featureful ECS in more unusual projects.

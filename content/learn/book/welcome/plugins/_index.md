+++
title = "Modularity through plugins"
weight = 2
template = "book-section.html"
page_template = "book-section.html"
+++

In Bevy, virtually all of the functionality you might want is added via *plugins*, which are added to your app.
These might provide basic windowing functionality, handle input or sound, calculate physics or provide game-specific logic like a tile map system.

By combining Bevy's first-party plugins with third-party alternatives, you can modularly customize the behaviour of the Bevy game engine, and then add your own plugins to contain your own game-specific code in a well-organized way.

## `MinimalPlugins`

Unless you're doing something very unusual, you will *always* want to include the plugins provided by Bevy's [`MinimalPlugins`](https://docs.rs/bevy/latest/bevy/struct.MinimalPlugins.html). We can click through to the source for the `impl PluginGroup for MinimalPlugins` to see what this adds:

```rust
use bevy_app::{PluginGroup, PluginGroupBuilder};

pub struct MinimalPlugins;

impl PluginGroup for MinimalPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy_core::CorePlugin::default());
        group.add(bevy_app::ScheduleRunnerPlugin::default());
    }
}
```

The `CorePlugin` handles low-level fundamentals such as updating app time, while the `ScheduleRunnerPlugin` sets up the main game loop to run repeatedly over time.

## `DefaultPlugins`

Bevy's `DefaultPlugins` are intended to get up and running with a "standard game" as quickly as possible. Let's take a look at the source again:

```rust
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy_log::LogPlugin::default());
        group.add(bevy_core::CorePlugin::default());
        group.add(bevy_transform::TransformPlugin::default());
        group.add(bevy_diagnostic::DiagnosticsPlugin::default());
        group.add(bevy_input::InputPlugin::default());
        group.add(bevy_window::WindowPlugin::default());
        group.add(bevy_asset::AssetPlugin::default());
        group.add(bevy_scene::ScenePlugin::default());

        #[cfg(feature = "bevy_render")]
        group.add(bevy_render::RenderPlugin::default());

        #[cfg(feature = "bevy_sprite")]
        group.add(bevy_sprite::SpritePlugin::default());

        #[cfg(feature = "bevy_pbr")]
        group.add(bevy_pbr::PbrPlugin::default());

        #[cfg(feature = "bevy_ui")]
        group.add(bevy_ui::UiPlugin::default());

        #[cfg(feature = "bevy_text")]
        group.add(bevy_text::TextPlugin::default());

        #[cfg(feature = "bevy_audio")]
        group.add(bevy_audio::AudioPlugin::default());

        #[cfg(feature = "bevy_gilrs")]
        group.add(bevy_gilrs::GilrsPlugin::default());

        #[cfg(feature = "bevy_gltf")]
        group.add(bevy_gltf::GltfPlugin::default());

        #[cfg(feature = "bevy_winit")]
        group.add(bevy_winit::WinitPlugin::default());

        #[cfg(feature = "bevy_wgpu")]
        group.add(bevy_wgpu::WgpuPlugin::default());
    }
}
```

As you can see, there are some added niceties (logging and diagnostics, scenes, input and windowing) as well as a number of plugins controlled by feature flags.
These are heavier dependencies that you may want to disable for some games (or platforms), and replace with your own alternatives.
To do so:

TODO: explain how to disable plugins.

## Writing your own plugins

Plugins are simply collections of code that modify the `AppBuilder`.

You can write your own to organize your own code by implementing the `Plugin` trait on a struct of your own creation.

```rust
use bevy::prelude::*;

fn main(){
 App::build()
   .add_plugins(DefaultPlugins)
   // Plugins merely organize code: this could just be replaced directly with 
   // the add_system and init_resource calls from below
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

## Third-party plugins

Importing 3rd-party plugins is easy; they're just Rust code!

TODO: show how to import another plugin.
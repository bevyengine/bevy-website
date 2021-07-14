+++
title = "Modularity through plugins"
weight = 4
template = "book-section.html"
page_template = "book-section.html"
+++

Writing all your Bevy code in one file would be a pain. Even a game as simple as snake could take up more than 300 lines of code. That's what plugins are for.

Plugins are essentially collections of systems, components and resources that can be added to the main app. For example, if you want a window to display, you can add the `WindowPlugin` and the `RenderPlugin`.

You can also define your own plugins to seperate related pieces of information into dedicated areas. Finally, there are also plugin groups that group plugins together. 

## `MinimalPlugins`

One of the aformentioned plugin groups is the `MinimalPlugins` group. This plugin is for the most bare-bones of games or tools. It only includes bevy-core and bevy-app. If you are making a text-based game, this is the option for you.

You can add it like so:

```rust
App::build()
    .add_plugins(MinimalPlugins)
    .run()
```

**NOTE: The Bevy Prelude must be included in `main.rs`**

## `DefaultPlugins`

Another plugin group (the one that you will most likely be using) is the `DefaultPlugins` group. This plugin group includes all the bells and whistles for a Bevy project (rendering, audio, loggings, assets) and is generally recommended for larger scale projects.

You can add it like so:

```rust
App::build()
    .add_plugins(DefaultPlugins)
    .run()
```

**NOTE: The Bevy Prelude must be included in `main.rs`**

## Writing your own plugins

Let's seperate our `hello_bevy` system into a plugin.

Open `main.rs` from the last project and define a public struct:

```rust
pub struct HelloPlugin;
```

Then, implement the `Plugin` trait for the struct:

```rust
impl Plugin for HelloPlugin {

}
```

Next, define a `build` function with a reference to `self` (immutable) and `AppBuilder` (mutable) and add any systems or resources you want:

```rust
fn build(&self, app: &mut AppBuilder) {
    app.add_system(hello_world.system());
}
```

Finally, add the plugin in the main function:

```rust
App::build()
    .add_plugin(HelloPlugin)
    .run();
```

## Third-party plugins

You can also install third-party plugins from [crates.io](https://crates.io).

You can find a list of currently available plugins [here](https://bevyengine.org/assets)

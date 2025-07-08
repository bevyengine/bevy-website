+++
title = "Plugins"
insert_anchor_links = "right"
[extra]
weight = 0
+++

**Plugins** are the most important tool for organizing your Bevy projects,
allowing you to split logic across files, easily toggle behavior for testing,
and reuse functionality between projects.

Plugins are best used to organize your code into functional units:
`WorldGenPlugin`, `InventoryPlugin` or `AudioPlugin` are all plausible.
Inside of each plugin, you can initialize resources,
add systems, setup observers, register types and generally handle the setup needed
to make your subsystem function.
Once all of this setup is complete, a plugin's job is done:
vanishing into thin air as the systems and resources it added live on to carry out its job.

At its heart, the `Plugin` trait is extremely simple.
It looks a little something like this:

```rust
trait Plugin {
    fn build(&self, app: &mut App) {}
}
```

Let's break that down:

- this is a trait, so we need to implement it for a user-defined type
- `build` takes `&self`, allowing you to change the behavior based on the value.
- it takes a`&mut App` reference, allowing you to mutate the [`App`] state freely
  - adding systems via [`App::add_systems`] is the most common and important method
  - as discussed in our section on [apps], [`App`] holds a [`World`], allowing you to add resources, make queries, spawn entities and more
- because of how simple this is, we can just cut-and-paste code from our `main.rs` into plugins when we're ready to clean up

Let's see how this works in practice:

```rust,hide_lines=1-4
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

```rust,hide_lines=1-4
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

## Configuring plugins

As alluded to above, you can use the `&self` argument in [`Plugin::build`] to change
the behavior of the plugin based on the passed in configuration.
But *should* you use this pattern?

Whenever possible, you should prefer using resources for configuration,
alerting users to their presence via documentation,
and responding dynamically to changes in their value.

This is inherently more flexible, allowing your user to tweak the settings at runtime
to account for changing needs.
It *also* plays nicer with various tooling, allowing users
to easily inspect and dynamically modify the values.

But sometimes, the value passed in *only* make sense
to be provided once, at the time of plugin definition.
A relatively common example of this is allowing the user
to configure which schedule your plugins' systems should be added to.
In that case, using a struct with fields as your `Plugin` type
is the best and only option.

## Plugin groups

You may have noticed that [`App::add_plugins`] is a method that takes `&mut App`.
Does that mean you can add plugins via other plugins?
Yes, yes it does.

For game code, this can be really convenient,
allowing you to readily group functionality into a nice hierarchy.

If you're writing a library though, you probably shouldn't do nest plugins like this.
It can make it harder to follow what's going on,
and it becomes challenging to selectively disable plugins for testing or things like "making a server build".

Instead, you should use a [`PluginGroup`].
You've likely already encountered these: [`DefaultPlugins`] is a [`PluginGroup`]!

[`PluginGroup`] comes with a few extra niceties over simply recursively adding plugins:

1. You can write dedicated docs for the collection of plugins.
2. You can overwrite the values of contained plugins via [`PluginGroup::set`], changing the default config.
3. With the help of [`PluginGroupBuilder`] you can enable and disable contained plugins cleanly.

## Plugin ordering and dependencies

When working with multiple plugins, be mindful that they're effectively just functions that immediately mutate the [`App`].
As a result, plugins are [evaluated in the order that they are added to the `App`].
This can lead to very annoying bugs for your users as they try and do seemingly innocuous things
like alphabetizing their plugin list, so please try to be robust to this behavior.

Currently, there is [no official solution for declaring that one plugin relies on the existence of another].
Using a [duck typing] approach, where you check for the existence of required resources,
is the least bad solution for now.

Similarly, to avoid tricky bugs, adding the same plugin to your app multiple times will panic by default.
This behavior can be overridden by overwriting the default [`Plugin::is_unique`] method.
The same duck-typing solution can be used to check if the plugin already exists,
and avoid re-adding it if another dependency has already pulled it in.

## The `Plugin` lifecycle

When a plugin is added though [`App::add_plugins`], the app calls `Plugin::build`, and the plugin typically accesses and configures the world.  
Then, when the app is run, a few other plugin life-cycle functions are called, and finally we enter the run loop:

- The app polls `Plugin::finished` until all the added plugins return `true`.
- The app calls `Plugin::finish` on all plugins.
- The app calls `Plugin::cleanup` an all plugins.
- The app calls the run loop function on itself.

In most cases, this complexity can be completely ignored, but when working with subsystems that require deferred initialization it can be helpful.

[apps]: [../the-game-loop/app]
[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`App::add_systems`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html?search=add#method.add_systems
[`App::add_plugins`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html?search=add#method.add_plugins
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html
[the blanket for all functions that take a &mut App]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html#impl-Plugin-for-T
[`PluginGroup`]: https://docs.rs/bevy/latest/bevy/app/trait.PluginGroup.html
[`DefaultPlugins`]: https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html
[`PluginGroup::set`]: https://docs.rs/bevy/latest/bevy/prelude/trait.PluginGroup.html#method.set
[`PluginGroupBuilder`]: https://docs.rs/bevy/latest/bevy/app/struct.PluginGroupBuilder.html
[evaluated in the order that they are added to the `App`]: https://github.com/bevyengine/bevy/issues/1255
[no official solution for declaring that one plugin relies on the existence of another]: https://github.com/bevyengine/bevy/issues/69
[duck typing]: https://en.wikipedia.org/wiki/Duck_typing
[`Plugin::is_unique`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html#method.is_unique

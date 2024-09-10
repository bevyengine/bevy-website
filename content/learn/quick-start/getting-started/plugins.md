+++
title = "Plugins"
insert_anchor_links = "right"
aliases = ["learn/book/getting-started/plugins"]
[extra]
weight = 4
+++

One of Bevy's core principles is modularity. All Bevy engine features are implemented as plugins---collections of code that modify an [`App`]. This includes internal features like the renderer, but games themselves are also implemented as plugins! This empowers developers to pick and choose which features they want. Don't need a UI? Don't register the [`UiPlugin`]. Want to build a headless server? Don't register the [`RenderPlugin`].

This also means you are free to replace any components you don't like. If you feel the need, you are welcome to build your own [`UiPlugin`], but consider [contributing it back to Bevy](https://github.com/bevyengine/bevy/blob/main/CONTRIBUTING.md) if you think it would be useful!

Those not contributed back into Bevy and instead released separately are third-party plugins. These are useful and easy to use additions created by fellow developers that can help you avoid re-inventing the wheel. To use them all you have to do is:

1. Find a third party Bevy plugin (like those at the [Assets page](/assets)).
2. Add it to your `Cargo.toml` as a crate under `[dependencies]`.
3. Import the code definitions (like `use third_party::prelude::*;`) from the crate to add the items to your workspace.
4. Add the plugin to your app (like `app.add_plugins(third_party_plugin)`).

However, most developers don't need a custom experience and just want the "full engine" experience with no hassle. For this, Bevy provides a set of [`DefaultPlugins`].  

## Bevy's Default Plugins

Let's make our app more interesting by adding Bevy's [`DefaultPlugins`] which are a [`PluginGroup`] containing core engine features. (For those needing minimal features, [`MinimalPlugins`] exists).
`add_plugins(DefaultPlugins)` adds the features most people expect from an engine, such as a 2D / 3D renderer, asset loading, a UI system, windows, and input.

{{file_code_block(file="quick-start/getting_started_v6.rs", anchor="app_main")}}

Once again run `cargo run`.

You should hopefully notice two things:

* **A window should pop up**. This is because we now have [`WindowPlugin`], which defines the window interface (but doesn't actually know how to make windows), and [`WinitPlugin`] which uses the [winit library](https://github.com/rust-windowing/winit) to create a window using your OS's native window API.
* **Your console is now full of "hello" messages**: This is because [`DefaultPlugins`] adds an "event loop" to our application. Our App's ECS Schedule now runs in a loop once per "frame". We will resolve the console spam in a moment.

## Creating your first plugin

For better organization, let's move all of our "hello" logic to a plugin. To create a plugin we just need to implement the [`Plugin`] interface. Add the following code to your `main.rs` file:

{{file_code_block(file="quick-start/getting_started_v7.rs", anchor="hello_plugin")}}

Then register the plugin in your App like this:

{{file_code_block(file="quick-start/getting_started_v7.rs", anchor="app_main")}}

Note `add_plugins` can add any number of plugins (or plugin groups like `DefaultPlugins`) by passing in a tuple of them. Now all that's left is to move our systems into `HelloPlugin`, which is just a matter of cut and paste. The `app` variable in our plugin's `build()` function is the same builder type we use in our `main()` function:

{{file_code_block(file="quick-start/getting_started_v8.rs", anchor="hello_plugin_implementation")}}

Try running the app again. It should do exactly what it did before. In the next section, we'll fix the "hello" spam using Resources.

[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`UiPlugin`]: https://docs.rs/bevy/latest/bevy/ui/struct.UiPlugin.html
[`RenderPlugin`]: https://docs.rs/bevy/latest/bevy/render/struct.RenderPlugin.html
[`WindowPlugin`]: https://docs.rs/bevy/latest/bevy/window/struct.WindowPlugin.html
[`WininitPlugin`]: https://docs.rs/bevy/latest/bevy/winit/struct.WinitPlugin.html
[`DefaultPlugins`]: https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html
[`PluginGroup`]: https://docs.rs/bevy/latest/bevy/app/trait.PluginGroup.html
[`MinimalPlugins`]: https://docs.rs/bevy/latest/bevy/struct.MinimalPlugins.html
[`Plugin`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html

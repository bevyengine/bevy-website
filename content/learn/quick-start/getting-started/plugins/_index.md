+++
title = "Plugins"
weight = 4
sort_by = "weight"
template = "docs-section.html"
page_template = "docs-section.html"
insert_anchor_links = "right"
+++

One of Bevy's core principles is modularity. All Bevy engine features are implemented as plugins. This includes internal features like the renderer, but games themselves are also implemented as plugins! This empowers developers to pick and choose which features they want. Don't need a UI? Don't register the [`UiPlugin`]. Want to build a headless server? Don't register the [`RenderPlugin`].

This also means you are free to replace any components you don't like. If you feel the need, you are welcome to build your own [`UiPlugin`], but consider [contributing it back to Bevy](/learn/quick-start/contributing) if you think it would be useful!

However, most developers don't need a custom experience and just want the "full engine" experience with no hassle. For this, Bevy provides a set of "default plugins".  

## Bevy's Default Plugins

Let's make our app more interesting by adding the "default Bevy plugins".
`add_plugins(DefaultPlugins)` adds the features most people expect from an engine, such as a 2D / 3D renderer, asset loading, a UI system, windows, and input.

```rs
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
```

Once again run `cargo run`.

You should hopefully notice two things:

* **A window should pop up**. This is because we now have [`WindowPlugin`], which defines the window interface (but doesn't actually know how to make windows), and [`WininitPlugin`] which uses the [winit library](https://github.com/rust-windowing/winit) to create a window using your OS's native window API.
* **Your console is now full of "hello" messages**: This is because [`DefaultPlugins`] adds an "event loop" to our application. Our App's ECS Schedule now runs in a loop once per "frame". We will resolve the console spam in a moment.

## Creating your first plugin

For better organization, let's move all of our "hello" logic to a plugin. To create a plugin we just need to implement the [`Plugin`] interface. Add the following code to your `main.rs` file:

```rs
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
    }
}
```

Then register the plugin in your App like this:

```rs
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
```

Note `add_plugins` can add any number of plugins (or plugin groups like `DefaultPlugins`) by passing in a tuple of them. Now all that's left is to move our systems into `HelloPlugin`, which is just a matter of cut and paste. The `app` variable in our plugin's `build()` function is the same builder type we use in our `main()` function:

```rs
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, greet_people));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}
```

Try running the app again. It should do exactly what it did before. In the next section, we'll fix the "hello" spam using Resources.

[`UiPlugin`]: https://docs.rs/bevy/latest/bevy/ui/struct.UiPlugin.html
[`RenderPlugin`]: https://docs.rs/bevy/latest/bevy/render/struct.RenderPlugin.html
[`WindowPlugin`]: https://docs.rs/bevy/latest/bevy/window/struct.WindowPlugin.html
[`WininitPlugin`]: https://docs.rs/bevy/latest/bevy/winit/struct.WinitPlugin.html
[`DefaultPlugins`]: https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html
[`Plugin`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html

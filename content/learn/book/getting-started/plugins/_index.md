+++
title = "Plugins"
weight = 3
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

One of Bevy's core principals is modularity. All Bevy engine features are implemented as plugins. This includes internal features like the renderer, but games themselves are also implemented as plugins! This empowers developers to pick and choose which features they want. Don't need a UI? Don't register the {{rust_type(type="struct", mod="bevy::ui", name="UiPlugin", no_mod=true)}}. Want to build a headless server? Don't register the {{rust_type(type="struct", mod="bevy::render", name="RenderPlugin", no_mod=true)}}.

This also means you are free to replace any components you don't like. If you feel the need, you are welcome to build your own {{rust_type(type="struct", mod="bevy::ui", name="UiPlugin", no_mod=true)}}, but consider [contributing it back to Bevy](/learn/book/contributing) if you think it would be useful!

However, most developers don't need a custom experience and just want the "full engine" experience with no hassle. For this, Bevy provides a set of "default plugins".  

## Bevy's Default Plugins

Lets make our app more interesting by adding the "default Bevy plugins". 
{{rust_type(type="struct", mod="bevy::app", name="AppBuilder", method="add_default_plugins", no_mod=true, no_struct=true)}} adds the features most people expect from an engine, such as a 2D / 3D renderer, asset loading, a UI system, windows, and input. 

```rs
use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .run();
}
```

Once again run ```cargo run```. This time, you should see a window pop up! This is because we now have {{rust_type(type="struct", mod="bevy::window", name="WindowPlugin", no_mod=true)}}, which defines the window interface (but doesn't actually know how to make windows), and {{rust_type(type="struct", mod="bevy::winit", name="WinitPlugin", no_mod=true)}} which uses the <a href="https://github.com/rust-windowing/winit" target="_blank">winit library</a> to create a window using your OS's native window api.

{{rust_type(type="struct", mod="bevy::app", name="AppBuilder", method="add_default_plugins", no_mod=true, no_struct=true)}} is equivalent to the following:
```rs
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugin(CorePlugin::default());
        .add_plugin(InputPlugin::default());
        .add_plugin(WindowPlugin::default());
        /* more plugins omitted for brevity */
        .run();
}
```

Feel free to use whatever approach suits you! Hopefully now it is clear what we mean by "modular". You are free to remove whatever plugins you don't want!

## Creating your first plugin

To create a plugin we just need to implement the {{rust_type(type="trait" name="AppPlugin" mod="bevy::core" no_mod=true)}} interface. Add the following code to your `main.rs` file:

```rs
pub struct HelloPlugin;

impl AppPlugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // add things to your app here
    }
}
```


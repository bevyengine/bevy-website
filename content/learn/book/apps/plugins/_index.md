+++
title = "Plugins"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

One of Bevy's core principals is modularity. We believe a game engine doesn't need to be a large monolithic binary. As we saw in the previous section, the simplest Bevy app literally does (and contains) nothing!

All Bevy game logic and engine features are implemented as plugins. This empowers developers to pick and choose which features they want. Don't need a UI? Don't register the {{rust_type(type="struct", mod="bevy::ui", name="UiPlugin", short=true)}}. Want to build a headless server? Don't register the {{rust_type(type="struct", mod="bevy::render", name="RenderPlugin", short=true)}}.

This also means you are free to replace any components you don't like. Think you can build a better {{rust_type(type="struct", mod="bevy::window", name="WindowPlugin", short=true)}}? Go ahead! But consider [contributing it back to Bevy](/learn/book/contributing) so others can benefit from your genius :) 

Of course, most developers won't want to pick and choose which components they use right out of the gate. Bevy has a set of "default plugins" that that provide a "full engine" experience.  

## Bevy's default plugins

Lets make our app more interesting by adding the "default Bevy plugins". 
{{rust_type(type="struct", mod="bevy::app", name="AppBuilder", method="add_default_plugins", short=true)}} adds all the juicy engine features you crave, such as a 2D / 3D renderer, asset loading, a UI system, windows, and input. 

```rs
use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .run();
}
```

Once again run ```cargo run```. This time, you should see a window pop up!

## Creating a plugin
{{rust_type(type="trait" name="AppPlugin" mod="bevy::core" short=true)}} 
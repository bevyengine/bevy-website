+++
title = "World"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

When you start your first Bevy project, you may have noticed that your main interaction point is a Bevy `App`: you can add `Systems`, `Plugins`, and `Resources` to it, and then tell it to run:

TODO: Add basic `App` example

But what is an `App` composed of? How does it store data and drive your game along? Well to start, it contains a `World`.

### What in the World?
A `World` is a collection of all the data in your game: `Entities`, `Components`, `Resources`, and more are all stored within it. You have the ability to create your own:

TODO: Add basic `World` creation and spawning entities on it by hand

But in most cases, it's enough to use the default world generated within an app. Certain methods on `App`, such as `.init_resource`, correlate with injecting some data into the app's `World` (in this case, it's just calling the world's own `.init_resource` under the hood!). When `Query`s are run within a system, they fetch (and can mutate) data stored within the world. You can even make mutations on the world itself, or replace the world entirely!

### When to Create Your Own

TODO

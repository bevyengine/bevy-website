+++
title = "The Bevy Renderer"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 15
status = 'hidden'
+++

There's a good chance you'll want your game to have visuals, like adding image sprites to characters and enemies, using 3D meshes to represent objects and environments, or even creating custom shaders for unique visual effects.
All of these scenarios involve using **a renderer**, a tool used to customize the final look of a graphical program.
By default, Bevy provides a full suite of state-of-the-art rendering tools for both 2D and 3D games.

When we talk about the "Bevy Renderer", we're actually referring to a collection of crates and features that can be toggled on and off in order to add specific functionality to your Bevy application.
For example, the [`bevy_render`](https://docs.rs/bevy_render/latest/bevy_render/) crate contains the core rendering feature set, while crates like [`bevy_light`](https://docs.rs/bevy_light/latest/bevy_light/) and [`bevy_shader`](https://docs.rs/bevy_shader/latest/bevy_shader/) provide support for lighting functionality and custom shader types, respectively.
All of these are grouped under a "render" category, and thus we'll be documenting them throughout this chapter.

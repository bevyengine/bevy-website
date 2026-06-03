+++
title = "Bevy's Rendering Architecture"
insert_anchor_links = "right"
[extra]
weight = 1
+++

Bevy's renderer consists of a series of modular, performant, and cross-platform-compatible tools.
It provides industry standard features while still remaining accessible and approachable to users who aren't very familiar with graphics programming.
Ultimately, the goal of Bevy's renderer is to empower Bevy users to be able to create the visuals they want.
If users can't create the render style they want, then what is even the point!

Even though Bevy's core audience is game developers, we know that Bevy is used much more broadly.
While we won't be able to satisfy everyone, we want the renderer to be flexible enough for users to feel that they can always work with it.
We want Bevy's renderer to be useful when making pixel art, stylized 2D, high-end PBR, CAD visualizations, and anything else you can think of.
Modularity is a core value, but that also means that we can't offer everything by default.
If some feature isn't implemented directly, you should be able to build on what already exists, or have the space to create that functionality yourself.

Before we start detailing _how_ you can start using the many features of Bevy's renderer, it'll help to understand _what_ the renderer is doing and _why_ it's set up the way it is.
Understanding these concepts will allow you to get a bird's eye view of the rendering process and let you know where to look when you start working with the renderer.

## The Graphics Stack

Modern computers can come with some seriously powerful hardware.
GPUs in particular have evolved into the workhorses of modern games, utilizing millions of cores to render complex graphics at unbelievable speeds.
But the GPU doesn't inherently know how to output a "photo-realistic pirate ship" or a "voxel world filled with cubed creatures".
It's ultimately up to you to create those scenes and find that "perfect look".

To help with achieving that "perfect look", Bevy is built to allow you to interact with as much or as little of the rendering process as you want.
You can set everything up within Bevy itself, allowing the tools Bevy provides to handle the dirty work.
Or you can dive into the graphics APIs yourself, bypassing our abstractions and wielding all the rendering power yourself!

All of this is enabled through a series of layers that send data from your game to the computer's GPU.
At a high level, those layers (affectionately called "the stack") looks something like this:

1. Bevy's Renderer
2. Abstraction Layer (WGPU)
3. Graphics API (Vulkan, Metal, DirectX, OpenGL, etc.)
4. GPU Driver (AMD, NVIDIA, Intel, Apple, etc.)
5. GPU Hardware (The actual graphics card)

Every frame, your game will send data to the renderer to determine what is rendered to the player's screen.
By default, Bevy's renderer uses an abstraction tool to translate this data into the computer's underlying graphics API calls (Vulkan, Metal, DirectX12, WebGPU, etc.).
Finally, these calls are sent and then processed by the GPU driver before being executed on the GPU hardware itself.
The end result is a frame being rendered by the GPU and displayed on the screen!

## WGPU

[WGPU](https://wgpu.rs/) is a modern, low-level graphics library.
It's an implementation of the [WebGPU API](https://webgpu.org/) specification, a cross-platform graphics API originally designed to enable GPU accelerated rendering within web browsers.
However, it isn't just for web browsers. 
Using an implementation of WebGPU means that an application can natively target every modern graphics API, namely Vulkan, DirectX12, Metal, WebGL2, or WebGPU depending on the system.

Bevy uses WGPU (via the [`wgpu`](https://crates.io/crates/wgpu) crate) to unify its rendering process.
Instead of having to individually support each graphics API, WGPU gives Bevy a unified interface for rendering.
As long as you're targeting a supported platform (Windows, MacOS, Linux, Android, and iOS), WGPU can translate your requests into the appropriate graphics API calls.

{% callout(type="info") %}
### Why WGPU?

While there would be some benefit in creating our own rendering solution, the truth of the matter is that WGPU already occupies exactly the space that Bevy currently needs:

- Multiple supported backends by default, with the goal to support as many platforms as possible.
- A "baseline" feature set that works almost everywhere with a consistent API.
- A "limits" and "features" system that enables turning on arbitrary, sometimes backend-specific features and detecting when those features are available.
- A modern GPU API, but without the pain and complexity of raw Vulkan/DirectX/Metal.
  - Perfect for user-facing Bevy renderer extensions!

{% end %}

This book focuses on Bevy's own rendering features, but a broader base of knowledge is quite useful, and you may occasionally want to interact with WGPU directly for advanced features.
If you are interested in learning more about WGPU, we can point you towards [Learn WGPU](https://sotrh.github.io/learn-wgpu/), a series of tutorials that can help you learn more about working with WGPU and graphics programming concepts in general.

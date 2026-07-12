+++
title = "Bevy's Rendering Architecture"
insert_anchor_links = "right"
[extra]
weight = 1
+++

Bevy's renderer consists of a series of modular, performant, and cross-platform-compatible tools.
It provides industry standard features while still remaining accessible and approachable to users who aren't very familiar with graphics programming.
Ultimately, the goal of Bevy's renderer is to let users construct the visual style their game needs.
After all, if users can't create the look they want, then what is even the point!

Even though Bevy's core audience is game developers, we know that Bevy is used much more broadly.
We won't be able to satisfy every use case by default; this is why we focus on modularity. 
Bevy's renderer should be flexible enough that it can be used to make pixel art, stylized 2D, high-end PBR, CAD visualizations, or anything else you can think of.
This also means we want users to be able to build any features they need that Bevy doesn't offer by default.
If a feature isn't implemented directly, you should be able to build on what already exists, or have the space to create that functionality yourself.

Before we start detailing _how_ you can start using the many features of Bevy's renderer, it'll help to understand _what_ the renderer actually is and _why_ it's set up the way it is.
Understanding these concepts will allow you to get a bird's eye view of the rendering process and let you know where to look when you start working with the renderer.

## The Graphics Stack

Modern computers can come with some seriously powerful hardware.
GPUs in particular have evolved into the workhorses of modern games, utilizing thousands, or even tens of thousands of cores to render complex graphics at unbelievable speeds.
But the GPU doesn't inherently know how to output a "photo-realistic pirate ship" or a "voxel world filled with cubed creatures".
It's ultimately up to you to create those scenes and find that "perfect look".

To help with achieving that "perfect look", Bevy is built to let you interact with as much of the rendering process as you want.
You can set everything up within Bevy itself, allowing our tools to handle the dirty work.
Alternatively, you could bypass Bevy's abstractions and dive straight into the graphics APIs, wielding all the rendering power yourself!

This is enabled by a series of layers that send data from your game to the computer's GPU.
At a high level, those layers (affectionately called "the stack") looks something like this:

1. Bevy's Renderer
2. Abstraction Layer (WGPU)
3. Graphics API (Vulkan, Metal, DirectX12, WebGPU, etc.)
4. GPU Driver (AMD, NVIDIA, Intel, Apple, etc.)
5. GPU Hardware (The actual graphics card or integrated graphics device)

Every frame, your game will send data to the renderer to determine what should be rendered to the player's screen.
By default, Bevy uses an abstraction tool to translate this data into the computer's underlying graphics API calls (Vulkan, Metal, DirectX12, WebGPU, etc.).
Finally, these calls are sent and then processed by the GPU driver before being executed on the GPU hardware itself.
The end result is a frame being rendered by the GPU and displayed on the screen!

## WGPU

[WGPU](https://wgpu.rs/) is a modern, low-level graphics library.
It's an implementation of the [WebGPU API](https://webgpu.org/) specification, a cross-platform graphics API originally designed to enable GPU accelerated rendering within web browsers.
However, it isn't just for web browsers. 
Using an implementation of WebGPU means that an application can natively target every modern graphics API released after 2010, namely Vulkan, DirectX12, Metal, WebGL2, or WebGPU depending on the system.

Bevy uses WGPU (via the [`wgpu`](https://crates.io/crates/wgpu) crate) to unify its rendering process.
Instead of having to individually support each graphics API, WGPU gives Bevy a unified interface for rendering.
As long as you're targeting a supported platform (Windows, MacOS, Linux, Android, and iOS), WGPU can translate your requests into the appropriate graphics API calls.

{% callout(type="info") %}
### Why WGPU?

Now that we've gone over what Bevy uses for its rendering setup, you might be wondering why Bevy chooses to rely on a third-party tool instead of creating our own solution.
After all, we know what type of rendering setup we want, what platforms we want to support, and what features we want to implement.
We could write our own abstractions over the individual graphics APIs, and expose those features through a custom API.
A Bevy-native rendering solution might even give us better performance by integrating core engine features more closely.

While there would be benefits to creating a Bevy-native setup, the truth of the matter is that WGPU already occupies the space that Bevy currently needs.
By choosing to rely on WGPU, we're removing several burdens: supporting multiple graphics APIs, implementing unified features across those APIs, detecting hardware-specific limitations, and so on.
We also gain quite a lot by sharing knowledge and learning resources with WGPU rather than having to create our own bespoke material.
Ultimately, it's about finding a solution that meets Bevy's needs while removing unnecessary burdens from our maintainers: WGPU is currently the best fit for those goals.

{% end %}

This chapter focuses on Bevy's own rendering features, but a broader base of knowledge is quite useful, and you may occasionally want to interact with WGPU directly for advanced features.
If you are interested in learning more about WGPU, we can point you towards [Learn WGPU](https://sotrh.github.io/learn-wgpu/), a series of tutorials that can help you learn more about working with WGPU and graphics programming concepts in general.

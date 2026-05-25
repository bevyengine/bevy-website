+++
title = "Bevy's Rendering Architecture"
insert_anchor_links = "right"
[extra]
weight = 1
+++

Before we start detailing _how_ you can start using the many features of Bevy's renderer, you should understand _what_ the renderer is doing and _why_ it's set up the way it is.
Understanding these concepts will allow you to get a bird's eye view of the rendering process and let you know where in the stack a specific feature is implemented.

## Render Architecture Overview

### Pipelined Rendering

Bevy uses a pipelined rendering process that exists and operates on a separate CPU thread from the main app `World`.
Information about each scene is extracted from the main app `World` and into a `RenderWorld` that is then rendered to the screen using the GPU.
This extraction happens once per frame, and can be delayed if either the game logic in `World` or the rendering process in `RenderWorld` takes longer than the other.
`RenderWorld` still abides by the E.C.S. principles and even has its own schedules and systems to control how the scene is rendered.

{% callout(type="info") %}

#### GPU Vs CPU Rendering 

Bevy used to perform its rendering setup on the CPU, choosing which objects needed to be rendered by performing frustum and occlusion-based culling before passing that information to the GPU.
However, this process was inefficient and did not align with modern rendering standards.
Eventually the task of calculating what objects are visible (and thus are actually rendered) was moved to the GPU, which is both more efficient and is more inline with modern rendering practices.

However, the CPU still plays an important role in rendering.
Since Bevy uses a separate `RenderWorld` to perform the rendering, information about the main app `World` still has to be copied and moved to the `RenderWorld`.
The CPU is responsible for doing this, ensuring that the `RenderWorld` always reflects the previous state of the app `World`.

{% end %}

We'll look at how each [`RenderSystem`] is set up in a separate page, but for now we'll just do an overview of the whole process.
Specifically, we can look at the main five steps that occur during rendering:

 **1. Extract**

To render an image to the screen, we first have to figure out what is happening in our game!
We use the Extract step to copy information from the main app `World` into the `RenderWorld`.
The CPU is responsible for accomplishing this, but it doesn't inherently know what information is needed.
We have to indicate that a `Component` and its data should be copied to the `RenderWorld`.
One way of accomplishing this is deriving the [`ExtractComponent`] trait on the components that we want to transfer to the `RenderWorld`.

However, we have to be careful about what we copy over.
If every component in the main `World` is copied over, your game might become **Render-bound**: a situation that occurs when rendering takes longer to finish than the game logic, and thus the game starts to lag.
However, if the opposite occurs (the game logic takes longer to finish than the rendering process), then your game becomes **App-bound**. 

Neither situation is inherently bad, as it is usually unnecessary to perfectly align the time that computing both the game logic and rendering within a single frame will take.
The ideal situation is to try to balance both processes, while aiming to keep both as fast as possible.
Remember that because Bevy uses a split world setup for rendering, neither the `World` or `RenderWorld` can progress until the other is finished computing.

**2. Prepare**

The Prepare step is used to set up the GPU resources needed for rendering.
This is where assets and meshes are set up for the GPU to use, along with creating [`BindGroup`]s that hold the data needed for rendering.

**3. Queue**

This is where we begin to set up the actual jobs that have to be done by the renderer.
Once our mesh data and bind groups have been set up, we can start queuing the individual items ([`PhaseItem`]s) that the GPU has to render.
This step can also include an optional sorting step ([`RenderSystems::PhaseSort`]) depending on how each item is queued.

**4. Render**

This is where the magic finally happens!
Using [`RenderGraph`]s, we execute each [`PhaseItem`] that we queued in the previous step, incorporating the specified [`RenderPipeline`]s and [`BindGroup`]s that help us get the desired look.
Again, we'll go into more detail in a future page, but it's important to briefly cover what exactly a [`RenderGraph`] is and what it does.

A [`RenderGraph`] is exactly what it sounds like, a graph!
Its used to create the commands that are eventually sent to the GPU to render the game to the screen.
Textures and buffers (and occasionally Entities) are passed along the `RenderGraph` to construct the draw calls that are sent to the GPU.

`RenderGraph`s consists of [`Node`]s, [`Edge`]s, and [`Slot`]s.
Nodes are responsible for generating draw calls and operating on input and output slots.
Edges specify the order of execution for nodes and connect input and output slots together.
Slots describe the render resources created or used by the nodes.

**5. Cleanup**

Once all [`RenderGraph`]s have finished executing, the `RenderWorld` is cleared of all entities and the data is reset.
If you need data to persist in the `RenderWorld`, you can use a [`Resource`] to store it in between frames.

[`RenderSystem`]: https://docs.rs/bevy/latest/bevy/render/enum.RenderSystems.html
[`ExtractComponent`]: https://docs.rs/bevy/latest/bevy/render/extract_component/trait.ExtractComponent.html
[`BindGroup`]: https://docs.rs/bevy/latest/bevy/render/render_resource/struct.BindGroup.html
[`PhaseItem`]: https://docs.rs/bevy/latest/bevy/render/render_phase/trait.PhaseItem.html
[`RenderSystems::PhaseSort`]: https://docs.rs/bevy/latest/bevy/render/enum.RenderSystems.html#variant.PhaseSort
[`RenderGraph`]: https://docs.rs/bevy/latest/bevy/render/render_graph/struct.RenderGraph.html
[`RenderPipeline`]: https://docs.rs/bevy/latest/bevy/render/render_resource/struct.RenderPipeline.html
[`Node`]: https://docs.rs/bevy/latest/bevy/render/render_graph/trait.Node.html
[`Edge`]: https://docs.rs/bevy/latest/bevy/render/render_graph/enum.Edge.html
[`Slot`]: https://docs.rs/bevy/latest/bevy/render/render_graph/enum.SlotType.html
[`Resource`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.Resource.html

### The Graphics Stack

In order to take advantage of the power that modern GPUs have, Bevy uses [WGPU](https://wgpu.rs/) (via the [`wgpu`](https://crates.io/crates/wgpu) crate) to handle the low-level graphics API calls.
`wgpu` acts as an intermediary between Bevy and the underlying graphics API for each computer system.
Depending on whether you're using Windows, MacOS, or Linux (or Android or iOS), WGPU translates your requests into the appropriate graphics API calls, namely DirectX, Metal, or Vulkan for the previous respective platforms.
Finally, these calls are processed by the GPU driver and then executed on the GPU hardware itself.
The full stack looks something like this, starting with the higher level interactions on top and moving down the stack to the GPU itself:

1. Bevy Rendering
2. WGPU
3. Graphics API (Vulkan, Metal, DirectX, OpenGL, etc.)
4. GPU Driver (AMD, NVIDIA, Intel, Apple, etc.)
5. GPU Hardware (The actual graphics card)

Most layers in this stack can be interacted with if desired, however for practical purposes it's likely you'll only ever
directly work with the Bevy Renderer, WGPU, or the underlying graphics API for a computer system.
For the purposes of the information we will present in this chapter, we won't be detailing how WGPU or the individual graphics API work.
However, if you are interested in learning more about WGPU, we can point you towards [Learn WGPU](https://sotrh.github.io/learn-wgpu/), a series of tutorials that can help you learn more about working with WGPU and graphics programming concepts in general.

### WGPU

Bevy is built on top of the [wgpu](https://wgpu.rs/) library, which is a modern low-level GPU API that can target pretty much every popular graphics API: Vulkan, Direct3D 12, Metal, OpenGL, WebGL2, and WebGPU.
The best backend API is selected for a given platform.
It is a "native" rendering API, but it generally follows the WebGPU terminology and API design.

{% callout(type="info") %}
#### Why WGPU?

Bevy has always used WGPU to power our rendering technology.
While there would be some benefit in creating our own rendering solution, the truth of the matter is that WGPU already occupies exactly the space that Bevy currently needs:

- Multiple supported backends by default, with the goal to support as many platforms as possible.
- A "baseline" feature set that works almost everywhere with a consistent API.
- A "limits" and "features" system that enables turning on arbitrary, sometimes backend-specific features and detecting when those features are available.
- A modern GPU API, but without the pain and complexity of raw Vulkan.
  - Perfect for user-facing Bevy renderer extensions!

#### Replacing WGPU

Bevy's renderer is designed to allow the user as much control over the rendering process as they want.
Even though we'll recommend to stick with WGPU for your projects, it isn't impossible to replace if you really want to.
We'll cover the process of replacing WGPU in a future page.

{% end %}

## Render Pipelines



### Shaders



{% callout(type="info") %}
#### Why Not Use Rust?

Rust is a great language for writing low-level systems code, but it is not suitable for interfacing with GPU hardware.
Working with a domain specific language designed to be run on GPUs allows you to create shaders and other GPU-specific code without needing to worry about the constraints that Rust imposes.
{% end %}

## 2D Vs 3D Rendering

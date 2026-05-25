+++
title = "Bevy's Rendering Architecture"
insert_anchor_links = "right"
[extra]
weight = 1
+++

Before we start detailing _how_ you can start using the many features of Bevy's renderer, you should understand _what_ the renderer is doing and _why_ it's set up the way it is.
Understanding these concepts will allow you to get a bird's eye view of the rendering process and let you know where in the stack a specific feature is implemented.

## Render Architecture Overview

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

For practical purposes it's likely you'll only ever directly work with the Bevy Renderer, WGPU, or the underlying graphics API for a computer system.
However, detailing how WGPU or the individual graphics API work is well beyond the scope of this chapter.
If you are interested in learning more about WGPU, we can point you towards [Learn WGPU](https://sotrh.github.io/learn-wgpu/), a series of tutorials that can help you learn more about working with WGPU and graphics programming concepts in general.

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

## Pipelined Rendering

Bevy uses a pipelined rendering process that exists and operates on a separate CPU thread from the main app `World`.
Information about each scene is extracted from the main app `World` and into a `RenderWorld` that is then rendered to the screen using the GPU.
This extraction happens once per frame, and can be delayed if either the game logic in `World` or the rendering process in `RenderWorld` takes longer than the other.
`RenderWorld` still abides by the E.C.S. principles and even has its own schedules and systems to control how the scene is rendered.

{% callout(type="info") %}

#### GPU Vs CPU Rendering 

CPU-driven rendering is where draw commands are created on the CPU.
Bevy used to perform its rendering setup this way, using the CPU to choose which objects needed to be rendered and how they needed to be rendered before passing that information to the GPU.
However, this process was inefficient and did not align with modern rendering standards.
Eventually the task of calculating what objects needed to be rendered was moved to the GPU, which is both more efficient and is more inline with modern rendering practices.

With GPU-driven rendering, the CPU is only responsible for passing data to the GPU.
Draw commands are encoded on the GPU by compute shaders, and the GPU determines which objects need to be rendered.
This leverages GPU parallelism, and unlocks more advanced culling optimizations that are infeasible to do on the CPU, among many other methods that bring large performance benefits.

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

### Using Shader Languages

Within a [`RenderPipeline`], we can use shading language scripts to help modify the final look of our renders.
Specifically, Bevy uses the [WGSL] language (and soon the [WESL] language as well!) to write these scripts, which are made available in the pipeline for any specified [`RenderGraph`] to use.
WGSL was built specifically for WGPU rendering; if you choose to replace WGPU with another rendering solution, you'll have to look at what shading language to use instead.

Much like the other non-Bevy topics, we won't be showcasing how to write WGSL (or WESL) code, however we will be showcasing how you can provide scripts to be used in your rendering pipelines.
To start learning WGSL, we recommend going to _.
For WESL, we recommend learning WGSL first, and then investigating WESL.
This is because WESL is an extension of the WGSL language, which means that you'll wind up learning WGSL anyways.
To read more about the WESL specifications, check out _.

{% callout(type="info") %}
#### Why Not Use Rust?

When you compile Rust code, the Rust compiler works to build and optimize the code to be run on the CPU, a device which will usually only have a couple dozen cores and threads to use for processing.
CPUs are designed to switch between multiple single processes sequentially.
Think of the individual programs on your computer: internet browsers, email clients, and media players are all separate processes that aren't very intensive to individually run.

Meanwhile, modern GPUs contain upwards of hundreds of millions (even billions!) of processing units.
Each GPU processing unit is generally slower than a CPU's, but when all processing units are used together they're able to handle exponentially more smaller tasks at the same time.
Take outputting video to a display, for example.
Standard 1080p HD computer monitors will have 2,073,600 pixels, and each pixel will have a red, green, and blue value.
These values have to be repeatedly updated and calculated multiple times per second.
While the CPU could do the calculations, constantly changing and updating those values (like when playing a video game, for example) would cause the CPU struggle to both update the application itself and update each pixel at the same time.

It's the difference in device capability which requires us to use a specific shading language to help modify our rendering pipelines.

{% end %}

[WGSL]:
[WESL]:

## 2D Vs 3D Rendering

You might be wondering why we've gone into detail about "Render Pipelines", "Shaders", and "Graphs" when all you want to do is make a simple 2D game with image sprites.
Well, we've outlined everything above because when you look at a Bevy scene, it turns out that a 2D scene is actually just a "flattened" 3D scene.
Specifically, in Bevy 2D scenes are 3D scenes with the _layering_ of objects in a scene represented by their Z axis value.

With this, Bevy is now working towards unifying its 2D and 3D rendering systems.
This helps reduce the burden on our maintainers while also allowing users to benefit from features that might seem like they're only designed for one game type.
2D games can benefit from the full power of render pipelines and shader effects, while 3D games can use image sprites and other simpler objects when needed.

To illustrate why unifying these systems are beneficial, consider the [`Mesh`] type.
It appeared in Bevy 0.13 in order to help unify and streamline using primitive shapes, which previously were all independently created.
Now all primitive shapes share the [`Mesh`] type, and are all created using the same builder pattern, which makes the experience of using primitives simpler and cohesive.

The same principle is in the process for being implemented for the 2D and 3D render processes.
The goal is to eventually unify the systems so that both 2D and 3D developers are using the same tools, making it easier to swap between both game types and for all users to seamlessly learn about Bevy's renderer.

[`Mesh`]:

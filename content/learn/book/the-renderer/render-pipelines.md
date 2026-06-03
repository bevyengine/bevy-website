+++
title = "Render Pipelines"
insert_anchor_links = "right"
[extra]
weight = 2
+++

## Pipelined Rendering

Bevy uses a pipelined rendering process that exists and operates on a separate CPU thread from the main app `World`.
Information about each scene is extracted from the main app `World` and into a `RenderWorld` that is then rendered to the screen using the GPU.
This extraction happens once per frame, and can be delayed if either the game logic in `World` or the rendering process in `RenderWorld` takes longer than the other.
`RenderWorld` still abides by the E.C.S. principles and even has its own schedules and systems to control how the scene is rendered.

{% callout(type="info") %}

#### GPU Vs CPU Rendering 

CPU-driven rendering is where draw commands are created on the CPU, and only passed to the GPU to finally be rendered.
Bevy used to perform its rendering setup this way, using the CPU to choose which objects needed to be rendered and how they needed to be rendered before passing that information to the GPU.
However, this process was inefficient and did not align with modern rendering standards.

Eventually the task of calculating what objects needed to be rendered was changed to be GPU-driven.
With GPU-driven rendering, the CPU is only responsible for passing data to the GPU.
Draw commands are encoded on the GPU by compute shaders, and the GPU determines which objects need to be rendered.
This leverages GPU parallelism, and unlocks more advanced culling optimizations that are infeasible to do on the CPU, among many other methods that bring large performance benefits.

{% end %}

`RenderWorld` uses a series of sequentially executed [`RenderSystem`]s to carry out each step of the rendering process.
Each system has a specific purpose that relies on data processed in the previous system, which is why they need to be ordered in a specific sequence.
It's also possible for certain systems to be skipped if its function isn't required.
We'll look at how each [`RenderSystem`] is set up in a separate page, but for now we'll just do an overview of the whole process.
Specifically, we can look at the main five steps that occur during rendering:

 **1. Extract**

To render an image to the screen, we first have to figure out what is happening in our game!
We use the Extract step to copy information from the main app `World` into the `RenderWorld`.
Specifically, a buffer of component data is sent to `RenderWorld` every frame.

How do we know what data is actually sent to `RenderWorld` though?
One way of accomplishing this is to derive the [`ExtractComponent`] trait on the components that we want to transfer to the `RenderWorld`.
Other ways including manually setting up systems specifically designed to send component data based on input or other events.
Some Bevy components will also be sent automatically, like assets, meshes, and textures, and thus you do not have to manually specify that they need to be extracted.

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
WGSL is the shading language used by WGPU; if you choose to replace WGPU with another rendering solution, you'll have to look at what shading language to use instead.

Much like the other non-Bevy topics, we won't be showcasing how to write WGSL (or WESL) code, however we will show how you can provide scripts to be used in your rendering pipelines.
To start learning WGSL, we recommend [the WebGPU Fundamentals page on WGSL](https://webgpufundamentals.org/webgpu/lessons/webgpu-wgsl.html).
For WESL, we recommend learning WGSL first, and then investigating WESL.
This is because WESL is an extension of the WGSL language, which means that you'll wind up learning WGSL anyways.
To read more about the WESL specifications, check out [the WESL feature list](https://wesl-lang.dev/docs/WESL-Extensions).

{% callout(type="info") %}
#### Why Not Use Rust?

When you compile Rust code, the Rust compiler works to build and optimize the code to be run on the CPU, a device which will usually only have a couple dozen cores and threads to use for processing.
CPUs are designed to switch between multiple single processes sequentially.
Think of the individual programs on your computer: internet browsers, email clients, and media players are all separate processes that aren't very intensive to individually run.

Meanwhile, shading languages are specially designed to run on the GPU, a device which can contain upwards of hundreds of millions (even billions!) of processing units.
Each GPU processing unit is generally slower than a CPU's, but when all processing units are used together they're able to handle exponentially more, less intensive tasks at the same time.
Take outputting video to a display, for example.
Standard 1080p HD computer monitors will have 2,073,600 pixels, and each pixel will have a red, green, and blue value.
These values have to be repeatedly updated and calculated multiple times per second.
While the CPU could do the calculations, it's likely it would struggle to both constantly update the display and keep the application running smoothly.

It's this difference in device capabilities which requires us to use a specific shading language to help modify our rendering pipelines.

{% end %}

[WGSL]: https://gpuweb.github.io/gpuweb/wgsl/
[WESL]: https://wesl-lang.dev/

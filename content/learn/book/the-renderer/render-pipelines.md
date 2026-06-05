+++
title = "Render Pipelines"
insert_anchor_links = "right"
[extra]
weight = 2
+++

Traditional rendering methods usually rely on the "simulation" logic (game state, path-finding, physics, etc.)being computed and then "rendered" to the screen after.
This is a linear process though: the simulation can't progress until the render is finished, and the render can't start until the simulation is complete.
Modern rendering has outgrown this approach, as the task of rendering has moved from the CPU to dedicated GPU hardware.

**Pipeline Rendering** is an alternative approach that sees the main simulation logic decoupled from the render logic.
Instead of waiting for either process to complete, the simulation logic is run on one thread while the render logic is first copied and then moved over and executed on a separate thread.
This allows the simulation to execute concurrently with the render, meaning less "idle" time for both processes.

Bevy uses pipelined rendering, having a Main `App` that computes the simulation logic before then extracting that info into a separate Render `App`.
This extraction happens once per frame, and can delay the game if either the game logic in the Main `App` or the render process in the Render `App` takes longer than the other.
However, the Render `App` still abides by ECS principles and even has its own `World`, schedules, and systems to control how the scene is rendered.

{% callout(type="info") %}

#### GPU-Driven Vs CPU-Driven Rendering 

Determining what parts of a scene should be rendered is just as important as actually rendering the scene itself.
It's likely that some objects will obstruct or overlap others, and figuring out how those objects should be shown on screen can be an intensive process.
Like we mentioned above though, rendering has evolved to take advantage of the power that modern GPUs possess.
Instead of relying on the CPU (as was done in the past), modern rendering setups are shifting more intensive work onto the GPU itself.

This can be thought of as CPU-driven rendering versus GPU-driven rendering.
CPU-driven rendering is where the draw commands that determine how objects are rendered are created on the CPU, and only passed to the GPU to eventually be rendered.
On the other hand, GPU-driven rendering sees the GPU perform those computations itself; the CPU is only responsible for passing data to the GPU.

Bevy used to use CPU-driven rendering, using the CPU to choose which objects needed to be rendered and how they needed to be rendered before passing that information to the GPU.
However, this process was proving to be inefficient and was eventually changed to be GPU-driven.
Now instead of relying on the CPU to sequentially calculate each object to render, Bevy leverages GPU parallelism to perform these calculations in batches.
This helps speed up renders a lot, and reduces potential bottlenecks that might impede performance.

{% end %}

## The Rendering Process

The Render `App` follows a series of steps to carry out the rendering process.
Each step has a specific purpose in arranging and converting your games data into rendered frames.
However, it's also possible to skip certain steps if their function isn't explicitly required.

Let's look at the main five steps that occur during rendering:

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

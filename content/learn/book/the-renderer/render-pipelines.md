+++
title = "Pipelined Rendering"
insert_anchor_links = "right"
[extra]
weight = 2
+++

Traditional rendering methods usually rely on the "simulation" logic (game state, path-finding, physics, etc.) being computed and then "rendered" to the screen after.
This is a linear process, and can be restrictive: the simulation can't progress until the render is finished, and the render can't start until the simulation is complete.
While this does still work for less intensive or older games, the abilities of modern computer hardware have outgrown this approach.
Both simulations and rendering logic have become more complicated in recent years, meaning that it isn't performant for one process to wait for the other to finish.

**Pipeline Rendering** is an alternative approach that sees the main simulation logic decoupled from the render logic.
Instead of waiting on each other, the simulation logic is run on one CPU thread while the render logic is copied to a separate thread before being passed to the GPU to be rendered.
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
On the other hand, GPU-driven rendering sees the GPU perform those computations itself; the CPU is only responsible for passing the data to the GPU.

Bevy used to use CPU-driven rendering, using the CPU to choose which objects needed to be rendered and how they needed to be rendered before passing that information to the GPU.
However, this process was proving to be inefficient and was eventually changed to be GPU-driven.
Now instead of relying on the CPU to sequentially calculate each object to render, Bevy leverages GPU parallelism to perform these calculations in batches.
This helps speed up renders a lot, and reduces potential bottlenecks that might impede performance.

{% end %}

## The Rendering Process

The Render `App` follows a series of steps to carry out the rendering process.
Each step has a specific purpose in arranging and converting your games data into rendered frames.
However, it's also possible to skip certain steps if their function isn't explicitly required.

Let's look at the main four steps that occur during rendering:

### Extract

To render an image to the screen, we first have to figure out what is happening in our game!
We use the Extract step to copy information from the Main `App` and send it into the Render `App`.
This is the first thing to occur on every frame, so reducing the amount of information that is copied can be beneficial for improving the performance of your game.

How do we actually send data to the Render `App`?
One way is to derive the [`ExtractComponent`] trait on a component that we want to transfer.
This will automatically insert a [`SyncToRenderWorld`] component on an entity that receives the specified component.
During the [`ExtractSchedule`], entities with the `SyncToRenderWorld` component are copied over into the Render `World`.

```rust
// Derive `ExtractComponent` on a component.
#[derive(Component, ExtractComponent)]
struct ImageHandle {
    handle: Handle<Image>,
}

// Or, manually insert `SyncToRenderWorld` on an entity.
fn transfer_to_render_world(
    mut commands: Commands, 
    mut entity_query: Query<Entity, With<ImageHandle>>
) {
    for entity in entity_query.iter_mut() {
        commands.entity(entity).insert((SyncToRenderWorld));
    }
}
```

Once copied over, a connection between the Render `App` entity and their Main `App` counterpart is created by storing a component with their counterparts entity ID.
[`RenderEntity`] is inserted into the entity residing in the Main `App` and contains the Render `App` entity ID, while [`MainEntity`] is inserted into the entity residing in the Render `App` and contains the Main `App` entity ID.

Alternatively, if you only need a specific type of asset (like a texture or a mesh), you can load an [`Asset`].
`Asset`s that are loaded in your Main `App` are automatically copied over in the `ExtractSchedule`.
Bevy is able to do this by reading the associated [`AssetEvent`] messages that the `Asset` type emits when registered and loaded.
You can read more about how `Asset`s are loaded and handled by Bevy in the [dedicated Assets chapter](/learn/book/assets).

### Prepare & Queue

All the data that we just copied is just that - data.
We need to prepare the data by organizing it into [`BindGroup`]s, collections that point to an objects data stored in [`Buffer`]s.
Bevy is able to perform this for us, setting up each `BindGroup` using a [`BindGroupLayout`] to make accessing that data more convenient.

After our initial data is organized, the actual jobs that the renderer has to do are created.
It might be overwhelming to think about everything that has to be queued, but you can think of it as arranging a group of objects onto different layers (called a render "phase") using "instructions".
The data we copied contains the descriptions for each object in a scene (like a `Transform` component for where it's located in a scene).
Each render phase we set up contains the "instructions" for how a part of those objects should look when viewed from a specific camera.
For example, an opaque phase describes how the objects are arranged relative to each other from the camera's perspective, and a transparency phase describes if some objects can be seen through other objects.

Remember that data that's been assigned into [`BindGroup`]s?
We use these groupings to represent the individual objects that the camera sees in each render phase (aptly called [`PhaseItem`]s).
Each `PhaseItem` is assigned a [`Draw`] function, which is the final combination of instructions for how that item will be displayed on screen.
Finally, for each `Draw` function, a draw call is created which will tell the GPU what to display on the screen.

### Render

This is where the magic finally happens!
All the queued [`Draw`] functions for every [`PhaseItem`]s are executed, incorporating the specified [`RenderPipeline`]s and [`BindGroup`]s that help us get the desired look.

Since the Render `App` abides by the ECS principles, we can leverage systems to perform the actual task of rendering.
Each system that accesses the [`RenderContext`] system parameter can be used to create the commands that are sent to the GPU to render the game.
Using systems allows us to structure the execution of each system with a [`Schedule`], either those provided by Bevy by default ([`Core2d`] and [`Core3d`]), or by creating a custom render schedule.

### Cleanup

It should go without saying that the amount of data we copy into the Render `App` could quickly get out of hand if not managed carefully.
Therefore, the Render `App` does a little bit of cleaning once a frame has been fully rendered.

We don't want data that isn't needed sticking around when we could use that space for new textures or meshes.
However, we also don't want to completely wipe the Render `App`.
If we have entities and assets that persist in the Main `App`, it doesn't make sense to keep copying them over every frame.
Therefore, Bevy synchronizes the Render `App` to the current state of the Main `App`.

Remember [`RenderEntity`] and [`MainEntity`]?
These two components are the keys to keeping this synchronization process running.
If an entity is despawned in the Main `App`, it's subsequently removed from the Render `App` using the entity ID stored in these components.
The same is done for loaded assets, although once again the [`AssetEvent`]s are read to determine whether an asset should be unloaded from the Render `App`.
However, if you do need data to persist in the Render `App`, you can create and use a [`Resource`] to store it in the Render `App`.

[`ExtractComponent`]: https://docs.rs/bevy/latest/bevy/render/extract_component/trait.ExtractComponent.html
[`SyncToRenderWorld`]: https://docs.rs/bevy/latest/bevy/render/sync_world/struct.SyncToRenderWorld.html
[`ExtractSchedule`]: https://docs.rs/bevy/latest/bevy/prelude/struct.ExtractSchedule.html
[`RenderEntity`]: https://docs.rs/bevy/latest/bevy/render/sync_world/struct.RenderEntity.html
[`MainEntity`]: https://docs.rs/bevy/latest/bevy/render/sync_world/struct.MainEntity.html
[`Asset`]: https://docs.rs/bevy/latest/bevy/asset/trait.Asset.html
[`AssetEvent`]: https://docs.rs/bevy/latest/bevy/asset/enum.AssetEvent.html
[`BindGroup`]: https://docs.rs/bevy/latest/bevy/render/render_resource/struct.BindGroup.html
[`Buffer`]: https://docs.rs/bevy/latest/bevy/render/render_resource/struct.Buffer.html
[`BindGroupLayout`]: https://docs.rs/bevy/latest/bevy/render/render_resource/struct.BindGroupLayout.html
[`PhaseItem`]: https://docs.rs/bevy/latest/bevy/render/render_phase/trait.PhaseItem.html
[`Draw`]: https://docs.rs/bevy/latest/bevy/render/render_phase/trait.Draw.html
[`RenderPipeline`]: https://docs.rs/bevy/latest/bevy/render/render_resource/struct.RenderPipeline.html
[`RenderContext`]: https://docs.rs/bevy/latest/bevy/render/renderer/struct.RenderContext.html
[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Schedule.html
[`Core2d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/schedule/struct.Core2d.html
[`Core3d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/schedule/struct.Core3d.html
[`Resource`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.Resource.html

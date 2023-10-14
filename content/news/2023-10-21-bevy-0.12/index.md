+++
title = "Bevy 0.12"
date = 2023-10-21
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.12** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.12**, check out our [0.11 to 0.12 Migration Guide](/learn/migration-guides/0.11-0.12/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

## Feature Name

<div class="release-feature-authors">authors: @author</div>

## Automatic Batching and Instancing, and the Road to GPU-driven Rendering

<div class="release-feature-authors">authors: Rob Swain (@superdump), @james-j-obrien, @JMS55, @inodentry, @robtfm, @nicopap, @teoxoy, @IceSentry, @Elabajaba</div>

Bevy's renderer performance for 2D and 3D meshes can improve a lot. Both CPU and graphics API / GPU bottlenecks can be removed to give significantly higher frame rates. As always with Bevy, we want to make the most of the platforms you use, from the constraints of WebGL2, through WebGPU, to the highest-end native discrete graphics cards. A solid foundation can support all of this.

### What are the bottlenecks?

One major bottleneck is the structure of the data used for rendering.

* Mesh entity data is stored in one uniform buffer, but has to be rebound at different dynamic offsets per draw.
* Material type data (e.g. StandardMaterial uniform properties) are stored in individual uniform buffers that have to be rebound per draw if the material changes.
* Material textures are stored individually and have to be rebound per draw if the texture changes.
* Mesh index / vertex buffers are stored individually per-mesh and have to be rebound per draw.

All of this rebinding has both CPU and graphics API / GPU performance impact. On the CPU, it means encoding of draw commands has many more steps and takes more time than necessary. In the graphics API and on the GPU, it means issuing many more rebinding and separate draw commands.

Avoiding rebinding is both a big performance benefit for CPU-driven rendering, and is necessary to enable GPU-driven rendering.

### What are CPU- and GPU-driven rendering?

CPU-driven rendering is where draw commands are created on the CPU, in Bevy this means in Rust code, more specifically in render graph nodes.

In GPU-driven rendering, the draw commands are encoded on the GPU by compute shaders. This leverages GPU parallelism, and unlocks more advanced culling optimisations that are infeasible to do on the CPU, among many other methods that bring large performance benefits.

### Reorder Render Sets

<div class="release-feature-authors">authors: Rob Swain (@superdump), @james-j-obrien, @inodentry</div>

The order of draws needs to be known for some methods of instanced draws so that the data can be laid out, and looked up in order. For example, when per-instance data is stored in an instance-rate vertex buffer.

The render set order before 0.12 caused some problems with this as data had to be prepared before knowing the draw order. The previous order of sets was:

* Extract
* Prepare
* Queue
* Sort/Batch
* Render

This constraint was most visible in the sprite batching implementation that skipped Prepare, sorted and prepared data in Queue, and then after being sorted again alongside 2D meshes and other entities in the Transparent2d render phase, possibly had its batches split to enable drawing of those other entities.

The new render set order in 0.12 is:

* Extract
* PrepareAssets
* Queue
* Sort
* Prepare/Batch
* Render

PrepareAssets was introduced because we only want to queue entities for drawing once their assets have been prepared. Per-frame data preparation still happens in the Prepare set, but that is now after Queue and Sort, so the order of draws is known. This also made a lot more sense for batching, as it is now known at the point of batching whether an entity that is of another type in the render phase needs to be drawn.

### BatchedUniformBuffer and GpuArrayBuffer

OK, so we need to put many pieces of data of the same type into buffers in a way that we can bind them as few times as possible and draw multiple instances from them. How can we do that?

Instance-rate vertex buffers are one way, but they are very constrained to having a specific order. They are/may be suitable for per-instance data like mesh entity transforms, but they can't be used for material data.

The other main options are uniform and storage buffers. WebGL2 does not support storage buffers, only uniform buffers. Uniform buffers have a minimum guaranteed size per binding of 16kB on WebGL2. Storage buffers, where available, have a minimum guaranteed size of 128MB. Data textures are also an option, but are far more awkward for structured data and without support for linear data layouts on some platforms, they will perform less well. So, support uniform buffers on WebGL2 or where storage buffers are not supported, and use storage buffers everywhere else.

#### BatchedUniformBuffer

<div class="release-feature-authors">authors: Rob Swain (@superdump), @JMS55, @teoxoy, @robtfm, @konsolas</div>

We have to assume that on WebGL2, we may only be able to access 16kB of data at a time. Taking an example, MeshUniform requires 144 bytes per instance, which means 113 instances per 16kB binding. If we want to draw more than 113 entities, we need a way of managing a uniform buffer of data that can be bound at a dynamic offset per batch of instances. This is what BatchedUniformBuffer is designed to solve.

DEMO RUST CODE.

DEMO SHADER CODE.

PERFORMANCE IMPROVEMENT.

#### GpuArrayBuffer

<div class="release-feature-authors">authors: Rob Swain (@superdump), @JMS55, @IceSentry, @mockersf</div>

If users have to care about supporting both batched uniform and storage buffers to store arrays of data for use in shaders, many may choose not to because their priority is not WebGL2. We want to make it simple and easy to support all users.

GpuArrayBuffer was designed and implemented as an abstraction over BatchedUniformBuffer and using a StorageBuffer to store an array of T.

DEMO RUST CODE.

DEMO SHADER CODE.

PERFORMANCE IMPROVEMENT.

### 2D / 3D Mesh Entities using GpuArrayBuffer

<div class="release-feature-authors">authors: Rob Swain (@superdump), @robtfm, @Elabajaba</div>

The 2D and 3D mesh entity rendering was migrated to use GpuArrayBuffer for the mesh uniform data.

DEMO RUST CODE

DEMO SHADER CODE

PERFORMANCE IMPROVEMENT.

### Improved bevymark Example

<div class="release-feature-authors">authors: Rob Swain (@superdump), @IceSentry</div>

The bevymark example needed to be improved to enable benchmarking the batching / instanced draw changes. Modes were added to:

* draw 2D quad meshes instead of sprites
* vary the per-instance color data instead of only varying the colour per wave of birds
* generate a number of material / sprite textures and randomly choose from them either per wave or per instance depending on the vary per instance setting

This allows benchmarking of different situations for batching / instancing in the next section.

### Automatic Batching/Instancing of Draw Commands

<div class="release-feature-authors">authors: Rob Swain (@superdump), @robtfm, @nicopap</div>

There are many operations that can be done to prepare a draw command in a render pass. If anything needs to change either in bindings or the draw itself, then the draws cannot be batched together into an instanced draw. Some of the main things that can change between draws are:

* Pipeline
* BindGroup or its corresponding dynamic offsets
* Index/vertex buffer
* Index/vertex range
* Instance range

Pipelines usually vary due to using different shaders in custom materials, or using variants of a material due to shader defs as the shader defs produce different shaders. Bind group bindings can change due to different material textures, different buffers, or needing to bind different parts of some buffers using dynamic offsets. Index/vertex buffers and/or ranges change per mesh asset. Instance range is what we want to leverage for instanced draws.

#### Assumptions

The design of the automatic batching/instanced draws in Bevy makes some assumptions to enable a reasonable solution:

* Only entities with prepared assets are queued to render phases
* View bindings are constant across a render phase for a given draw function, as phases are per-view
* `batch_and_prepare_render_phase` is the only system that performs batching and has sole responsibility for preparing per-instance (i.e. mesh uniform) data

If these assumptions do not work for your use case, then you can add the `NoAutomaticBatching` component to your entities to opt-out and do your own thing. Note that mesh uniform data will still be written to the GpuArrayBuffer and can be used in your own mesh bind groups.

#### What 0.12 Enables

We can batch draws into a single instanced draw in some situations now that per-instance mesh uniform data is in a GpuArrayBuffer. If the mesh entity is using the same mesh asset, and same material asset, then it can be batched!

DEMO PERFORMANCE IMPROVEMENT.

#### What is next for rendering performance?

* Put material data into GpuArrayBuffer per material type (e.g. all StandardMaterial instances will be stored in one GpuArrayBuffer) - this enables batching of draws for entities with the same mesh, same material type and textures, but different material data! This is implemented on a branch.
* Put material textures into bindless texture arrays - this enables batching of draws for entities with the same mesh and same material type!
* Put mesh data into one big buffer per mesh attribute layout - this removes the need to rebind the index/vertex buffers per-draw, instead only vertex/index range needs to be passed to the draw command. A simple version of this is implemented on a branch.
* GPU-driven rendering
  * @JMS55 is working on GPU-driven rendering already, using a meshlet approach
  * Rob Swain (@superdump) also intends to implement an alternative method similar to what is done in rend3.

## Rendering Performance Improvements

### EntityHashMap

<div class="release-feature-authors">authors: Rob Swain (@superdump), @robtfm, @pcwalton, @jancespivo, @SkiFire13, @nicopap</div>

#### The Performance Problem

Since Bevy 0.6, Bevy's renderer has used a separate render world to store an extracted snapshot of the simulated data from the main world to enable pipelined rendering of frame N in the render app, while the main app simulates frame N+1.

Part of the design involves clearing the render world of all entities between frames. This enables consistent Entity mapping between the main and render worlds while still being able to spawn new entities in the render world that don't exist in the main world.

Unfortunately, this ECS usage pattern also incurred some significant performance problems. Entities are cleared and respawned each frame, components are inserted across many systems and different parts of the render app schedule.

The fastest ECS storage available is called table storage. A simplified concept for table storage is that it is a structure of arrays of component data. Each archetype has its own table for storage. Whenever a new component is inserted onto an entity that it didn't have before, its archetype is changed. This then requires that that entity's component data be moved from the table for the old archetype to the table for the new archetype.

In practice this was very visible in profiles as long-running system commands throughout the render app schedule.

DEMO PROFILE IMAGE

As can be seen, this was unfortunately leaving a lot of performance on the table. Many ideas were discussed over a long period for how to improve this. The main two paths forward were:

1. Persist render world entities and their component data across frames - this has the problem of memory leaks and Entity collisions
2. Stop using entities for storing component data in the render world

We have decided to explore option 2 for Bevy 0.12 as persisting entities involves solving other problems that have no simple and satisfactory answers.

#### Data Structures

Ideally we would only ever need to iterate over dense arrays of data (e.g. `Vec<T>`). CPUs are very good at this as it enables predictable data access that increases the cache hit rate and makes for very fast processing.

The options, for component data `T`:

* `Vec<T>`
* `SparseSet<Entity, T>` - contains a 'sparse' `Vec<T>` and a dense `Vec<usize>` that is indexed by `Entity.index` and the `usize` value is the index into the sparse `Vec`.
* `HashMap<Entity, T>`

`Vec<T>` cannot be used with the current renderer architecture. Mesh entities are extracted in extraction query iteration order. They are then queued to a render phase and sorted. They are iterated in phase order to prepare and batch into instanced draws. The renderer can be rearchitected to enable use of `Vec<T>` but that is more intrusive than there was time to finalize for 0.12.

`SparseSet<Entity, T>` is a good option and performs well. Lookups for batching involve indexing into two `Vec`s, which for lookups are fast. It has the benefit that the dense `Vec<T>` can be iterated directly if `Entity` order is irrelevant.

However, `SparseSet` has the downside that the dense `Vec<usize>` has to be as large as the largest contained `Entity.index`. If you spawn a million entities, then despawn 999,999, leaving the millionth entity still spawned, every `SparseSet<Entity, T>` for different `T` will have to have a `Vec<usize>` that is one million items large.

`HashMap<Entity, T>` is similar to `SparseSet`, and more familiar. It has good space complexity, with performance depending a lot on the hash function. Fast hash functions from the wild were tested AHash, FNV, FxHasher, SeaHasher, but all had quite a big performance drop compared to `SparseSet`. Ultimately, a hash function designed by @SkiFire13, inspired by `rustc-hash` was chosen that has strong performance and is robust enough for this usage.

### Sprite Instancing

Sprites were being rendered by generating a vertex buffer containing 4 vertices per sprite with position, UV, and possibly color data. This has proven to be very effective, but having to split batches of sprites into multiple draws because they use a different color is suboptimal.

Sprite rendering now uses an instance-rate vertex buffer to store the per-instance data. It contains an affine transformation matrix that enables translation, scaling, and rotation in one transform. It contains per-instance color, and UV offset and scale.

A quad is drawn by leveraging a rendering industry trick that leverages a special index buffer containing 6 indices. The indices encode the vertex position in their bits - the least significant bit is x, the next least significant bit is y. The vertices of the quad are then:

```text
10   11

00   01
```

This retains all the functionality of the previous method, enables the additional flexibility of any sprite being able to have a color tint and all still be drawn in the same batch, and uses a total of 80 bytes per sprite, versus 144 bytes previously. The practical result is a performance improvement of up to 40% versus the previous method!

### Overall Performance vs 0.11

3D Meshes

2D Meshes

Sprites

UI

## <a name="what-s-next"></a>What's Next?

We have plenty of work that is pretty much finished and is therefore very likely to land in **Bevy 0.13**:

Check out the [**Bevy 0.13 Milestone**](https://github.com/bevyengine/bevy/milestone/17) for an up-to-date list of current work being considered for **Bevy 0.13**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

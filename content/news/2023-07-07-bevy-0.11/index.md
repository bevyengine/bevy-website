+++
title = "Bevy 0.11"
date = 2023-07-07
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.11** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.11**, check out our [0.10 to 0.11 Migration Guide](/learn/migration-guides/0.10-0.11/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Feature**: description

## Feature

<div class="release-feature-authors">authors: @todo</div>

Description

## Simpler RenderGraph Construction

<div class="release-feature-authors">authors: @IceSentry, @cart</div>

Adding `Node`s to the `RenderGraph` requires a lot of boilerplate. In this release, we tried to reduce this for most common operations. No existing API have been removed, these are only helpers made to simplify working with the `RenderGraph`.

We added the `RenderGraphApp` trait to the `App`. This trait contains various helper functions to reduce the boilerplate with adding nodes and edges to a graph.

Another pain point of `RenderGraph` `Node`s is passing the view entity through each node and manually updating the query on that view. To fix this we added a `ViewNode` trait and `ViewNodeRunner` that will automatically take care of running the `Query` on the view entity. We also made the view entity a first party concept of the `RenderGraph`. So you can now access the view entity the graph is currently running on from anywhere in the graph without passing it around between each `Node`.

All these new apis assume that your Node implements `FromWorld` or `Default`.

Here's what it looks like in practice for the `BloomNode`:

```rust
// Adding the node to the 3d graph
render_app
    // To run a ViewNode you need to create a ViewNodeRunner
    .add_render_graph_node::<ViewNodeRunner<BloomNode>>(
        CORE_3D,
        core_3d::graph::node::BLOOM,
    );

// Defining the node
#[derive(Default)]
struct BloomNode;
// This can replace your `impl Node` block of any existing `Node` that operated on a view
impl ViewNode for BloomNode {
    // You need to define your view query as an associated type
    type ViewQuery = (
        &'static ExtractedCamera,
        &'static ViewTarget,
        &'static BloomSettings,
    );
    // You don't need Node::input() or Node::update() anymore. If you still need these they are still available but they have an empty default implementation.
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        // This is the result of your query. If it is empty the run function will not be called
        (camera, view_target, bloom_settings): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // When using the ViewNode you probably won't need the view entity but here's how to get it
        let view_entity = graph.view_entity();

        // Run the node
    }
}
```

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author

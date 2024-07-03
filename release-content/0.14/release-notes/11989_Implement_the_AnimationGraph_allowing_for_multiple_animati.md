Through the eyes of a beginner, handling animation seems simple enough.
Define a series of keyframes which transform the various bits of your model to match those poses.
We slap some interpolation on there to smoothly move between them, and the user tells you when to start and stop the animation. Easy!

But modern animation pipelines (especially in 3D!) are substantially more complex:
animators expect to be able to smoothly blend and programmatically alter different animations dynamically in response to gameplay.
In order to capture this richness, the industry has developed the notion of an **animation graph**, which is used to couple the underlying [state machine] of a game object to the animations that should be playing, and the transitions that should occur between each of the various states.

A player character may be walking, running, slashing a sword, defending with a sword...
to create a polished effect, animators need to be able to change between these animations smoothly, change the speed of the walk cycle to match the movement speed along the ground and even perform multiple animations at once!

In Bevy 0.14, we've implemented the [Animation Composition RFC], providing a low-level API that brings code- and asset-driven animation blending to Bevy.

```rust
#[derive(Resource)]
struct ExampleAnimationGraph(Handle<AnimationGraph>);

fn programmatic_animation_graph(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Create the nodes.
    let mut animation_graph = AnimationGraph::new();
    let blend_node = animation_graph.add_blend(0.5, animation_graph.root);
    animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/animated/Fox.glb")),
        1.0,
        animation_graph.root,
    );
    animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(1).from_asset("models/animated/Fox.glb")),
        1.0,
        blend_node,
    );
    animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(2).from_asset("models/animated/Fox.glb")),
        1.0,
        blend_node,
    );

    // Add the graph to our collection of assets.
    let handle = animation_graphs.add(animation_graph);

    // Hold onto the handle
    commands.insert_resource(ExampleAnimationGraph(handle));
}
```

While it can be used to great effect today, most animators will ultimately prefer editing these graphs with a GUI. We plan to build a GUI on top of this API as part of the fabled Bevy Editor. Today, there are also third party solutions like [`bevy_animation_graph`].

To learn more and see what the asset-driven approach looks like, take a look at the new [`animation_graph` example].

[state machine]: https://en.wikipedia.org/wiki/Finite-state_machine
[Animation Composition RFC]: https://github.com/bevyengine/rfcs/blob/main/rfcs/51-animation-composition.md
[`bevy_animation_graph`]: https://crates.io/crates/bevy_animation_graph
[`animation_graph` example]: https://github.com/bevyengine/bevy/tree/v0.14.0/examples/animation/animation_graph.rs

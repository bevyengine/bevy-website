Through the eyes of a beginner, handling animation seems simple enough.
Define a series of keyframes, transform, rotate and scale the various bits of your model to match those poses.
The user tells you when to start and stop an animation, we slap some interpolation on there to smoothly move between them, and you're done!

But modern animation pipelines (especially in 3D!) are substantially more complex:
animators expect to be able to smoothly blend and programmatically alter different animations dynamically in response to gameplay.
In order to capture this richness, the industry has developed the notion of an **animation graph**, which is used to couple the underlying [state machine] of a game object to the animations that should be playing, and the transitions that should occur between each of the various states.

A player character may be walking, running, slashing a sword, defending with a sword...
To create a polished effect, animators need to be able to change between these animations smoothly, change the speed of the walk cycle to match the movement speed along the ground and even perform multiple animations at once!

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

While it can be used to great effect today, most animators will ultimately prefer a node-based GUI solution like the third-party [`bevy_animation_graph`] or an eventual first party solution, shipped as part of the fabled Bevy Editor,
which can then generate the animation graph assets consumed by this API.

To learn more and see what the asset-driven approach looks like, take a look at the new [`animation_graph` example].

[state machine]: https://en.wikipedia.org/wiki/Finite-state_machine
[Animation Composition RFC]: https://github.com/bevyengine/rfcs/blob/main/rfcs/51-animation-composition.md
[`bevy_animation_graph`]: https://crates.io/crates/bevy_animation_graph
[`animation_graph` example]: https://github.com/bevyengine/bevy/blob/main/examples/animation/animation_graph.rs

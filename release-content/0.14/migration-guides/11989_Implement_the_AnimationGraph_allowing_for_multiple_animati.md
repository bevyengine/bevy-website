`AnimationPlayer`s can no longer play animations by themselves: they need to be paired with a `Handle<AnimationGraph>`. Code that used `AnimationPlayer` to play animations will need to create an `AnimationGraph` asset first, add a node for the clip (or clips) you want to play, and then supply the index of that node to the `AnimationPlayer`’s `play` method.

```rust
// 0.13
fn setup(mut commands: Commands, mut animations: ResMut<Assets<AnimationClip>>) {
    let mut animation = AnimationClip::default();

    // ...

    let mut player = AnimationPlayer::default();
    player.play(animations.add(animation));

    commands.spawn((
        player,
        // ...
    ));
}

// 0.14
fn setup(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
    // You now need access to the `AnimationGraph` asset.
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut animation = AnimationClip::default();

    // ...

    // Create a new `AnimationGraph` and add the animation handle to it.
    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

    let mut player = AnimationPlayer::default();
    // Play the animation index, not the handle.
    player.play(animation_index);

    commands.spawn((
        player,
        // Add the new `AnimationGraph` to the assets, and spawn the entity with its handle.
        graphs.add(graph),
        // ...
    ));
}
```

Furthermore, the `AnimationPlayer::play_with_transition()` method has been removed and replaced with the `AnimationTransitions` component. If you were previously using `AnimationPlayer::play_with_transition()`, add all animations that you were playing to the `AnimationGraph` and create an `AnimationTransitions` component to manage the blending between them.

For more information behind this change, you may be interested in [RFC 51](https://github.com/bevyengine/rfcs/blob/main/rfcs/51-animation-composition.md).

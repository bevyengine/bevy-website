<!-- Add Support for Triggering Events via `AnimationEvent`s -->
<!-- https://github.com/bevyengine/bevy/pull/15538 -->

Triggering gameplay events at specific points in an animation is a common pattern for synchronizing the visual, audible, and mechanical parts of your game. In **Bevy 0.15** we've added "animation event" support to [`AnimationClip`], which means that you can trigger a specific [`Event`] at a given point in time during [`AnimationClip`] playback:

```rust
#[derive(Event, Clone)]
struct PlaySound {
    sound: Handle<AudioSource>,
}

// This will trigger the PlaySound event at the 1.5 second mark in `animation_clip`
animation_clip.add_event(1.5, PlaySound {
    sound: assets.load("sound.mp3"),
});

app.add_observer(|trigger: Trigger<PlaySound>, mut commands: Commands| {
    let sound = trigger.event().sound.clone();
    commands.spawn(AudioPlayer::new(sound));
});
```

You can also trigger events for specific animation targets (such as bones):

```rust
animation_clip.add_event_to_target(AnimationTargetId::from_iter(["LeftLeg", "LeftFoot"], 0.5, TouchingGround);
```

This enables things like "triggering a dust effect each time a foot touches the ground in an animation":

<video controls><source src="animated_fox.mp4" type="video/mp4"/></video>

[`AnimationClip`]: https://docs.rs/bevy/0.15/bevy/animation/struct.AnimationClip.html
[`Event`]: https://docs.rs/bevy/0.15/bevy/ecs/event/trait.Event.html
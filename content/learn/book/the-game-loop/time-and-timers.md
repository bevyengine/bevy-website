+++
title = "Time and Timers"
insert_anchor_links = "right"
[extra]
weight = 5
+++

Responding to the passage of time is essential for gameplay, animations and audio.
But doing so correctly is surprisingly subtle.

The [`Time`] resource in Bevy is the source of truth
for the "current in-game time".
Unlike [`Instant::now()`], the [`Time`] is fixed for the entire frame,
rather than reflecting the precise wall clock time at the time of system evaluation.
This value is set at the start of the frame based on the time supplied by the renderer,
during the [`TimeSystem`] system set in the [`First`] schedule.

This is helpful for performance reasons, but more critically,
it ensures consistency of behavior across all of the various bits of game logic.

[`Time`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Time.html
[`Instant::now()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
[`TimeSystem`]: https://docs.rs/bevy/latest/bevy/time/struct.TimeSystem.html
[`First`]: https://docs.rs/bevy/latest/bevy/app/struct.First.html

## Frame-rate independence and delta time

Suppose we want to move our player to the right:

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn move_player(mut player_transform: Single<&mut Transform, With<Player>>){
    const PLAYER_MOVEMENT: f32 = 1.;
    
    player_transform.translation.x += PLAYER_MOVEMENT;
}
```

Every frame, our player will move 1 unit to the right. Great!
But what happens when our game stutters, and the frame rate drops?
Suddenly, rather than moving 60 units per second at our target 60 frames per second,
our player is moving at an unsteady 20-30 units per second. Oh no!

Instead, we can compensate for this effect by fixing our *speed* (or other rates of change per second),
and then multiplying by the elapsed time.

```rust, hide_lines=1-4
# use bevy::prelude::*;
# 
# #[derive(Component)]
# struct Player;
#
fn move_player(mut player_transform: Single<&mut Transform, With<Player>>, time: Res<Time>){
    // At 60 FPS, this will be the same as before
    const PLAYER_SPEED: f32 = 60.;
    
    player_transform.translation.x += PLAYER_SPEED * time.delta_secs_f32();
}
```

This technique is commonly known as ["delta time"] among game devs, because physicists use "delta" to mean "a change in a quantity".

["delta time"]: https://en.wikipedia.org/wiki/Delta_timing

## Pausing and time control

With all of our time-dependent logic driven by the delta time,
we can start playing tricks with the value of [`Time`] to implement features like pausing and slowing down the game.
To do this, we need to change how we account for [`Virtual`] (in-game) time:

```rust
use bevy::prelude::*;

fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

#[derive(Event)]
struct SetGameSpeed(f32);

fn set_game_speed(mut time: ResMut<Time<Virtual>>, events: EventReader::<SetGameSpeed>){
  if let Some(new_speed) = events.iter().last(){
      time.set_relative_speed(new_speed.0);
  }
}
```

If your systems uniformly rely on [`Time`], this will effect your entire game:
halting, slowing or speeding up animations, movement, projectiles, physics and so on.
Alternatively, [states] and [run conditions] can be used to skip systems while the game is paused.

[states]: ../control-flow/states.md
[run conditions]: ../control-flow/run-conditions.md

## Fixing your timestep

Compensating for fluctating frame times using delta time is a great start.
But as the timeless [fix your timestep article] explains for projects that require a higher level of stability and reproducibility,
it's better to simply always advance time by a fixed amount.
This is particularly important for physics and

To understand how to work with fixed time in Bevy, we need to first learn a little bit about how [`Time`] actually works under the hood.
As the excellent docs on [`Time`] explain, there's actually *three* distinct types of time being measured:

- real time: the actual wall time
  - use this for things like UI animations that you don't want to be affected by pausing
- virtual time: the "in-game time"
  - if you're using fixed time, this is useful for graphical effects
  - otherwise, this is used for all of your gameplay logic as well
- fixed time
  - all of your gameplay logic should go here if you're using a fixed timestep approach

When thinking about fixed time, it's important to clearly distinguish **frames** and **ticks**.
A "frame" is one pass of the [`Main`] schedule, and corresponds to a single rendered frames.
A "tick" is one pass of the [`FixedMain`] schedule, and corresponds to one iteration of gameplay logic.
The ratio between frames elapsed and ticks elapsed is not constant: they may be faster or slower, depending
on the rendering performance and your game's simulation needs.

{% callout(type="info") %}
Simply requesting [`Time`] in your systems will get you the correct flavor 90% of the time: virtual time in the [`Main`] schedule,
and fixed time in the [`FixedMain`] schedule.
To request a specific variation, change the implicit generic in [`Time`] from `()`
to [`Real`], [`Virtual`] or [`Fixed`].
{% end %}

Now that we have the required vocabulary, let's go over exactly how the fixed timestep logic works in Bevy:

- each frame, during the [`RunFixedMainLoop`] schedule, we determine how many times we should run the [`FixedMain`] schedule
  - we add the the elapsed virtual time for the frame to a running time buffer
  - if that time buffer is greater than or equal to [`Time<Fixed>::timestep`], we run the [`FixedMain`] schedule once, and subtract that timestep from our buffer
- when the [`FixedMain`] schedule is evaluated, the various subschedules are iterated through, running [`FixedPreUpdate`], [`FixedUpdate`] and so on in order
- within your fixed time systems, use the exact same delta time techniques to properly account for changes in your desired timestep during development

This means that we may have 0, 1, 2 or more ticks per frame,
with our fixed timestep logic running repeatedly until it's caught back up.
For even more detail, check out the excellent documentation on [`Fixed`].

Note that Bevy's "fixed timesteps" are not the right mechanism to use for gameplay logic like "every 5 seconds update this building".
Bevy only supports a single fixed timestep across your entire project, and its use is completely optional.
Please see our [timers and cooldowns] section below for the tools available to model this type of behavior.

[`Time<Fixed>::timestep`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Time.html#method.timestep
[`Main`]: https://docs.rs/bevy/latest/bevy/app/struct.Main.html
[`FixedMain`]: https://docs.rs/bevy/latest/bevy/app/struct.FixedMain.html
[`Real`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Real.html
[`Virtual`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Virtual.html
[`Fixed`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Fixed.html
[fix your timestep article]: https://gafferongames.com/post/fix_your_timestep/
[`RunFixedMainLoop`]: https://dev-docs.bevy.org/bevy/app/struct.RunFixedMainLoop.html

### Interpolation between ticks

One key problem with using a fixed timestep is that your game logic (including physics!) will have an uneven
number of updates between each frame.
This will lead to visible jittering, lagginess and tiny speedups.

To account for this, we need to distinguish between the logical and rendered position (and rotation, and sometimes scale) of
game objects.
This is a common problem and pattern for multiplayer games, but using a fixed timestep means that even single player
games need to handle it.

The core strategy is simple enough, but the devil is in the details:

1. Create your own custom `GamePosition` type, holding the translation/rotation/scale information that your project needs.
2. Modify and read this `GamePosition` type for all of your game logic and physics.
3. Interpolate/extrapolate between the previous [`GlobalTransform`] and the `GamePosition`, based on the elapsed [`Fixed`] time.
   1. This should occur after the final fixed update for the frame has run, but before rendering occurs.
   2. You ultimately want to set the [`GlobalTransform`], but doing this correctly in the presence of hierarchies is hard.
   3. As a result, modifying the local [`Transform`] and relying on transform propagation can be the least bad solution.

Bevy does not currently offer any built-in functionality for this form of interpolation,
but open source [ecosystem crates] are available to use and learn from.

[ecosystem crates]: https://bevy.org/assets/
[`GlobalTransform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.GlobalTransform.html
[`Transform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html

### Smooth camera movement

## Timers, cooldowns and delayed actions

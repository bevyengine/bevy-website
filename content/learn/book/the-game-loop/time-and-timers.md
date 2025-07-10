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

FIX YOUR TIMESTEP ARTICLE.
When thinking about fixed time, it's important to clearly distinguish **frames** and **ticks**.

To understand how to work with fixed time, we need to understand a little bit about how [`Time`] actually works under the hood.
As the excellent docs on [`Time`] explain, there's actually *three* distinct types of time being measured:

- real time: the actual wall time
  - use this for things like UI animations that you don't want to be affected by pausing
- virtual time: the "in-game time"
  - if you're using fixed time, this is useful for graphical effects
  - otherwise, this is used for all of your gameplay logic
- fixed time
  - advances by a [`Time::timestep`] every time [`FixedMain`] runs
  - [`FixedMain`] will run repeatedly until it has caught up with the elapsed virtual time
  - if not enough time has elapsed, [`FixedMain`] will not run at all for the frame
  - please read the excellent docs on [`Fixed`] for more details!

Simply requesting [`Time`] in your systems will get you the correct flavor 90% of the time: virtual time in the main schedule,
and fixed time in the fixed main schedule.
To request a specific variation, change the implicit generic in [`Time`] from `()`
to [`Real`], [`Virtual`] or [`Fixed`].

[`Time::timestep`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Time.html#method.timestep
[`FixedMain`]: https://docs.rs/bevy/latest/bevy/app/struct.FixedMain.html
[`Real`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Real.html
[`Virtual`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Virtual.html
[`Fixed`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Fixed.html

## Interpolation and smooth movement

## Timers and cooldowns

## Delayed actions

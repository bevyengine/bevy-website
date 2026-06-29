+++
title = "Time-based Controls"
insert_anchor_links = "right"
[extra]
weight = 13
status = 'hidden'
+++

Time is a fundamental aspect that most games will track in one form or another.
Its utility is almost universal.
Ability systems?
Cool-down and casting periods.
Online matches?
Usually have set lengths so that someone eventually wins.
Player/NPC turns?
Should be controlled so that everyone else isn't waiting for an eternity.

Most of these cases involve either counting down from a set period of time, or tracking how much time has passed at a given moment.
Bevy offers tools for both, namely the [`Timer`] and the [`Stopwatch`].
A `Timer` enters a "finished" state once its length of time has completed.
Meanwhile, a `Stopwatch` will simply track how much time has elapsed since it was created.

These are two simple tools that can be incredibly helpful, however Bevy provides many more features that can be used to structure your systems and even interact with `Commands`.
We'll detail those further down this page, but for now let's get a little more familiar with `Timer`s and `Stopwatch`s.

[`Timer`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Timer.html
[`Stopwatch`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Stopwatch.html
[`Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html

## Timers

The simplest time-based tool Bevy provides is the [`Timer`].
They allow you to track a duration of time and determine when it has completed.
`Timer`s are not components though: you cannot directly add a [`Timer`] to an entity.
Instead, `Timer`s are intended to be wrapped inside of simple components.

```rust
// A simple component that holds a `Timer` for the 
// cool-down of an ability.
#[derive(Component)]
struct AbilityTimer {
    timer: Timer,
}
```

Additionally, `Timer`s have no inherent logic: they do not update themselves, or have any inherent way to add something like an "on completion callbacks".
You have to indicate that a `Timer` should be updated using the [`Timer::tick`] method.
The `tick` method also requires specifying an amount of time (a [`Duration`] value) that will be used to advance the `Timer`.

```rust
fn update_ability_timer(mut query: Query<&mut AbilityTimer>) {
    for mut ability_timer in query.iter_mut() {
        ability_timer.timer.tick(Duration::from_secs(1));
    }
}
```

{% callout(type="warning") %}
### Consistent Ticking
Depending on when you `tick` a `Timer`, you might find that the timer is inconsistent or not behaving as intended.
This might depend on where the [`Schedule`] that the system you call this from is located.
Systems placed in the [`Update`] schedule will be run every frame, meaning that a `Timer` being ticked with a `Duration` value of 1 second could wind up advancing 60 seconds if your game runs at 60 frames per second.

This is known as "time-step", and depending on what you use a `Timer` for, it could severely alter your game.
The quick solution is to use the time between frames (also known as [delta time]) when determining the amount of time to advance a `Timer` by each `tick`.
This can be done by accessing the [`Time`] resource as a system parameter and using the [`Time::delta`] method to get the `tick` value.

Sometimes the solution isn't as straightforward though, so we'll also advise you to check out the dedicated [Time page] in the Game Loop chapter to read about how Bevy handles time and sets up different schedules.

[delta time]: https://en.wikipedia.org/wiki/Delta_timing
[Time page]: /learn/book/content/the-game-loop/game-time

[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Schedule.html
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html
[`Time`]: https://docs.rs/bevy/latest/bevy/time/struct.Time.html
[`Time::delta`]: https://docs.rs/bevy/latest/bevy/time/struct.Time.html#method.delta

{% end %}

### Creating Timers

When you create a `Timer`, you have to specify two values: a length of time (either a [`Duration`] or an `f32` amount of seconds) and a [`TimerMode`].
The length of time tracks how long the `Timer` will be active for, while the `TimerMode` determines whether the `Timer` is non-repeating ([`TimerMode::Once`]) or should repeat once it has finished ([`TimerMode::Repeating`]).

```rust
// A new timer that will expire after 5 seconds
Timer::new(Duration::from_secs(5), TimerMode::Once);

// A new timer that will repeat every 2 minutes
Timer::new(Duration::from_mins(2), TimerMode::Repeating);

// A new timer specifying a duration in seconds
Timer::from_seconds(3.0, TimerMode::Once);
```

Two `Duration`s are stored when a `Timer` is first created.
The first stores the length of the timer (i.e. the value we passed when creating the `Timer`), while the second stores how much time has elapsed since the timer started.
We can access both of these `Duration` values using the [`Timer::remaining`] and [`Timer::elapsed`] methods. 
If we only need the actual time values from these, we can use [`Timer::remaining_secs`] and [`Timer::elapsed_secs`] to get `f32` values instead of a `Duration` value.

```rust
fn ability_timer(ability_query: Query<(&Ability, &AbilityTimer)>) {
    for (ability, timer) in ability_query.iter() {
        if timer.remaining_secs() == 0.0 {
            // Indicate an ability is ready!
            println!("Ability {} is ready!", ability);
        } else {
            // Display the time remaining until the ability is ready.
            println!("Ability {} will be ready in {} seconds.", ability, timer.remaining_secs());
        }
    }
}
```

### Adjusting Timers

`Timer`s aren't locked in once they've been started.
The values you set can be adjusted, and they can even be paused if something should interrupt them.
Each `Timer` provides access to different methods that can change how they function.

For example, if the player pauses the game, you might want to pause any `Timer`s that are active.
[`Timer::pause`] can be done by mutably accessing the component the `Timer` is stored in.
When the player unpauses, simply call [`Timer::unpause`] to resume the timer from where they left off.

//TODO: Better examples, fix the Timer at least.
```rust
fn pause_timer(mut timer_query: Query<&mut Timer>) {
    for mut timer in timer_query.iter_mut() {
        timer.pause();
    }
}

fn unpause_timer(mut timer_query: Query<&mut Timer>) {
    for mut timer in timer_query.iter_mut() {
        timer.unpause();
    }
}
```

Additionally, we can reduce the amount of code by checking for the pause state of a `Timer`.
[`Timer::is_paused`] will return a `bool` value based on whether a `Timer` is paused or not.

```rust
fn check_timer_pause(mut timer_query: Query<&mut Timer>) {
    for mut timer in timer_query.iter_mut() {
        if timer.is_paused() {
            timer.unpause();
        }
    }
}
```

We can also change how a `Timer` operates.
Let's say the player has unlocked a power up and now their ability starts automatically regenerating after each use.
Instead of having to create a separate ability timer, just change the `TimerMode` to repeating with [`Timer::set_mode`].

```rust
fn on_ability_upgrade(mut ability_upgrade: Single<&mut AbilityTimer, With<AbilityUpgrade>) {
    *ability_upgrade.timer.set_mode(TimerMode::Repeating);
}
```

Each `Duration` value that the `Timer` tracks can be manually overridden.
[`Timer::set_elapsed`] will adjust the amount of time elapsed to the desired value, while [`Timer::set_duration`] can be used to change the total length the `Timer` will track for.
These can be helpful for giving buffs or debuffs to players or enemies based on the situations they encounter or the actions they take.

```rust
fn alter_ability(mut ability_timer: Single<&mut AbilityTimer, With<Ability>>) {
    // Reward the player with nearly unlimited ability use!
    *ability_timer.set_duration(Duration::from_secs(0.1));
    // Or punish them by effectively taking the ability away.
    *ability_timer.set_duration(Duration::from_secs(10000.0));
}
```


### Finishing Timers


[`Timer::tick`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.tick
[`TimerMode`]: https://docs.rs/bevy/latest/bevy/time/enum.TimerMode.html
[`TimerMode::Once`]: https://docs.rs/bevy/latest/bevy/time/enum.TimerMode.html#variant.Once
[`TimerMode::Repeating`]: https://docs.rs/bevy/latest/bevy/time/enum.TimerMode.html#variant.Repeating
[`Timer::elapsed`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.elapsed
[`Timer::elapsed_secs`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.elapsed_secs
[`Timer::remaining`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.remaining
[`Timer::remaining_secs`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.remaining_secs
[`Timer::pause`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.pause
[`Timer::unpause`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.unpause
[`Timer::is_paused`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.is_paused
[`Timer::set_elapsed`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.set_elapsed
[`Timer::set_duration`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.set_duration

## Stopwatches

## System Timer Conditions

```rust
#[derive(Resource)]
struct Cookies(u64);

#[derive(Component)]
struct AutomaticCookieClick {
    // This should be initialized as a repeating timer
    // ensuring it automatically resets
    timer: Timer,
    resources_gained: u64,
}

fn automatically_click_cookies(
    mut cookies: ResMut<Cookies>, 
    mut query: Query<&mut AutomaticCookieClick>,
    time: Res<Time>,
) {
    let delta_time = time.delta();

    for mut cookie_clicker in query.iter_mut() {
        cookie_clicker.timer.tick(delta_time);
        if cookie_clicker.timer.just_finished() {
            cookies.0 += cookie_clicker.resources_gained;
        }
    }
}
```

You could write your own cooldown mechanism in a similar way, by storing both a `duration` and a `remaining`.
Tracking each ability as its own entity, using a custom [relationship] to link it to the entity with that ability would be an elegant solution, as it would allow you to update all cooldowns in a single system.

```rust
#[derive(Relationship)]
#[relationship(relationship_target = Abilities)]
struct AbilityOf(Entity);

#[derive(RelationshipTarget)]
#[relationship_target(relationship = AbilityOf)]
struct Abilities(Vec<Entity>);

#[derive(Component)]
struct Cooldown {
    duration: Duration,
    remaining: Duration,
}

impl Cooldown {
    fn expend(&mut self) {
        self.remaining = self.duration;
    }
  
    fn is_ready(&self) -> bool {
        self.remaining == Duration::ZERO;
    }
}

fn update_cooldowns(time: Res<Time>, mut cooldowns: Query<&mut Cooldown>) {
    let delta_time = time.delta();
    for mut cooldown in cooldowns.iter_mut(){
        // We never want our remaining time to become negative
        cooldown.remaining = cooldown.remaining.saturating_sub(delta_time);
    }
}
```

To trigger a system periodically, the `on_timer` run condition can be very convenient.

```rust
fn tick_buildings(query: Query<&mut Building>){
   for mut building in query.iter_mut(){
      building.tick();
   }
}

App::new()
  .add_systems(Update, tick_buildings.run_if(on_timer(Duration::from_secs(5))));
```

Timers (and the `on_timer` run condition) can safely be ticked in any schedule.
When they are in a fixed time schedule, the `Time<Fixed>` delta time will automatically be used instead.

Note that systems run periodically via an `on_timer` run condition are still blocking!
While it is tempting to use them for very heavy, infrequent tasks (like chunk updating or path finding), a naive approach to this will simply result in your game stuttering every few seconds.

Instead, you should either split the work into bite-sized pieces that can safely be completed within a single frame, or spawn an async task which you periodically poll for completion.

[relationship]: /learn/book/storing-data/relations
[`on_timer`]: https://docs.rs/bevy/latest/bevy/time/common_conditions/fn.on_timer.html

## Delaying Commands

When developing your game, you might encounter some functionality that you'll want to run at a later point.
Using [`Commands`] would work for delaying the functionality until after the end of the system, but what if you need to delay it for a particular number of seconds?
Fortunately we have [`DelayedCommands`], a wrapper over the regular `Commands` struct which will store a queue of commands that will be applied after a specified delay.

Using `DelayedCommands` will look very similar to using the regular `Commands`, although we have to insert the [`.delayed`] method and an amount of time to delay in between our `Commands` struct and the command we want to execute.

```rust
fn delayed_spawn(mut commands: Commands) {
    commands.delayed().secs(1.0).spawn(DummyComponent);
}
```

`DelayedCommands` can be set using either seconds (using [`.secs`]) or a duration (using [`.duration`]), much like `Timer`s and `Stopwatch`s can.
However, instead of needing to manually tick our `DelayedCommands`, Bevy will automatically tick them in a system run in the `PreUpdate` schedule.
All we have to do is provide the amount of time to delay our command by, and Bevy will handle the rest.

[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Commands.html
[`DelayedCommands`]: https://docs.rs/bevy/latest/bevy/time/delayed_commands/struct.DelayedCommands.html
[`.delayed`]: https://docs.rs/bevy/latest/bevy/time/delayed_commands/trait.DelayedCommandsExt.html#tymethod.delayed
[`.secs`]: https://docs.rs/bevy/latest/bevy/time/struct.DelayedCommands.html#method.secs
[`.duration`]: https://docs.rs/bevy/latest/bevy/time/struct.DelayedCommands.html#method.duration

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
`Timer`s are not components though: you cannot directly add a `Timer` to an entity.
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
This might depend on what [`Schedule`] the `tick`ing system is placed in.
Systems placed in the [`Update`] schedule will run every frame, meaning that a `Timer` being ticked with a `Duration` value of 1 second could wind up advancing 60 seconds if your game runs at 60 frames per second.

This is known as "time-step", and depending on what you use a `Timer` for, it could severely alter your game.
The quick solution is to use the time between frames (also known as [delta time]) as the amount of time to advance a `Timer` by for each `tick`.
This can be done by accessing the [`Time`] resource as a system parameter and using the [`Time::delta`] method to get the `tick` value.

Sometimes the solution isn't as straightforward though.
To read more about "time-step" and how Bevy handles time, we recommend reading the dedicated [Time page] in the Game Loop chapter.

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

### Changing Timer Properties

`Timer`s aren't locked in once they've been started.
The values you set can be adjusted, and they can even be paused if something should interrupt them.
Each `Timer` provides access to different methods that can change how they function.

For example, if the player pauses the game, you might want to pause any `Timer`s that are active.
[`Timer::pause`] can be done by mutably accessing the component the `Timer` is stored in.
When the player unpauses, simply call [`Timer::unpause`] to resume the timer from where they left off.

```rust
fn pause_ability_timers(mut timer_query: Query<&mut AbilityTimer>) {
    for mut ability_timer in timer_query.iter_mut() {
        ability_timer.timer.pause();
    }
}

fn unpause_ability_timers(mut timer_query: Query<&mut AbilityTimer>) {
    for mut ability_timer in timer_query.iter_mut() {
        ability_timer.timer.unpause();
    }
}
```

Calling `pause` on a `Timer` will not increase the elapsed `Duration` while the `Timer` is paused.
Additionally, we can reduce the amount of code by checking for the pause state of a `Timer`.
[`Timer::is_paused`] will return a `bool` value based on whether a `Timer` is paused or not.

```rust
fn check_ability_timer(
    mut timer_query: Query<&mut AbilityTimer>,
    pause_state: Res<GamePauseState>,
) {
    for mut ability_timer in timer_query.iter_mut() {
        if ability_timer.timer.is_paused() && pause_state == false {
            ability_timer.timer.unpause();
        }
    }
}
```

We can also change how a `Timer` operates.
Let's say the player has unlocked a power up and now their ability starts automatically regenerating after each use.
Instead of having to create a separate ability timer, just change the `TimerMode` to repeating with [`Timer::set_mode`].

```rust
fn on_ability_upgrade(
    mut ability_upgrade: Single<&mut AbilityTimer, With<AbilityUpgrade>
) {
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

### Timer Completions

Once a `Timer` reaches its duration, it's placed in a "finished" state, and will stay "finished" unless it's reset or replaced with a new `Timer`.
We can check if a `Timer` is "finished" with the [`Timer::is_finished`] method.
This will return a simple `bool` value indicating whether the `Timer` is finished or not.

As an example, let's look at a Cookie Clicker inspired scenario.
We want to automatically click a cookie every at set time intervals, which will increase the total number of cookies the player has.
We'll create an `AutomaticCookieClick` component that contains a `Timer` to track how often the auto click should happen.

```rust
// A resource to track how many cookies the player has
#[derive(Resource)]
struct Cookies(u64);

// A source of automatic cookie clicks
#[derive(Component)]
struct AutomaticCookieClick {
    // This should be initialized as a repeating timer
    // ensuring it automatically resets
    timer: Timer,
    resources_gained: u64,
}

// A system that automatically clicks cookies based
// on the `AutomaticCookieClick` timer
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

There might also be situations where you want to immediately finish the `Timer`.
Continuing our example, maybe the player wants to click the cookie themselves?
If we wanted to tie in the player's clicks into the `Timer`, we could use the [`Timer::finish`] method to end the `Timer` and add a cookie.

```rust
// See if the player has manually clicked a cookie
fn player_click(
    mut query: Query<&mut AutomaticCookieClick>,
    player_input: Res<ButtonInput<MouseButton>>,
) {
    if player_input.just_pressed(MouseButton::Left) {
        for mut cookie_clicker in query.iter_mut() {
            cookie_clicker.timer.finish();
        }
    }
}
```

However, we also have two more methods to talk about that can be beneficial depending on the situation.
[`Timer::almost_finish`] will advance a `Timer` to have 1 nanosecond remaining.
This can be useful when you need an immediate action that occurs based on a `Timer` without having to wait for the `set_duration` of the timer in the first tick.

We can use this method in our example to ensure that a player's input properly interacts with our `Timer`.
Unless we specifically arrange the `player_click` system to run before the `automatically_click_cookies` system, there is a chance that the player's input finishes the `Timer` after the `automatically_click_cookies` system checks the `Timer`.
One way to prevent this is to use [`Timer::almost_finish`] to advance the `Timer`, but not fully complete it.
This ensures that the `Timer` finish is always detected by the `automatically_click_cookies` system.

```rust
fn player_click(
    mut query: Query<&mut AutomaticCookieClick>,
    player_input: Res<ButtonInput<MouseButton>>,
) {
    if player_input.just_pressed(MouseButton::Left) {
        for mut cookie_clicker in query.iter_mut() {
            // Change `finish` to `almost_finish`
            cookie_clicker.timer.almost_finish();
        }
    }
}
```

Additionally, the [`Timer::times_finished_this_tick`] method will return a `u32` value that represents how many times a `Timer` completed during the last `tick`.
Remember that each [`Timer::tick`] has to be advanced by a `Duration` value, meaning that it's possible for a `tick` to advance a `Timer` beyond its total length of time.
However, non-repeating `Timer`s will only ever return a 0 or 1 from this method.

To wrap up our Cookie Clicker example, let's create a boost in the number of cookies gained in a click and use [`Timer::times_finished_this_tick`] to print this out to the player.

```rust
// A bool value that indicates whether the cookie boost is active
#[derive(Resource)]
struct CookieBoost(pub bool);

fn automatically_click_cookies(
    mut cookies: ResMut<Cookies>, 
    mut query: Query<&mut AutomaticCookieClick>,
    time: Res<Time>,
    boost: Res<CookieBoost>,
) {
    let delta_time = time.delta();

    for mut cookie_clicker in query.iter_mut() {
        if boost.0 {
            let cookie_boost = 10;
            cookie_clicker.timer.tick(Duration::from_secs(cookie_boost));
            if cookie_clicker.timer.just_finished() {
                cookies.0 += cookie_clicker.resources_gained * cookie_boost;
                println!("Cookie Explosion! {}x Multiplier!", cookie_boost);
            }
        } else {
            cookie_clicker.timer.tick(delta_time);
            if cookie_clicker.timer.just_finished() {
                cookies.0 += cookie_clicker.resources_gained;
            }
        }
    }
}
```

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
[`Timer::finish`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.finish
[`Timer::almost_finish`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.almost_finish
[`Timer::times_finished_this_tick`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.times_finished_this_tick
[`Timer::is_finished`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html#method.is_finished

## Stopwatches

It might appear like [`Timer`]s and [`Stopwatch`]s are two separate tools, however the two are actually closely tied.
This is because a `Timer` is actually just a `Stopwatch` that is stopped or reset after a specific length of time passes.

With this, using a `Stopwatch` will feel very similar to using a `Timer`.
A `Stopwatch` still has to be `tick`ed by a `Duration` of time, we can still `pause` a `Stopwatch`, and we're still able to alter the values of a `Stopwatch` using a handful of methods.
However, we aren't able to see the "remaining" amount of time or "finish" a `Stopwatch`, because there is no duration a `Stopwatch` is constrained by.
This means you have to be careful when using `Stopwatch`s, since they won't automatically stop unless the containing component is removed or despawned.

```rust
#[derive(Resource)]
struct MatchTimeCounter {
    stopwatch: Stopwatch,
    ...
}

// System to create the match timer
fn create_match_timer(mut commands: Commands) {
    commands.insert_resource(MatchTimeCounter {
        stopwatch: Stopwatch::new(),
        ...
    });
}

// System to tick the match timer
fn tick_match_timer(
    mut match_time: ResMut<MatchTimeCounter>,
    time: Res<Time>,
) {
    match_time.stopwatch.tick(time.delta());
}

// System to pause the match timer
fn pause_match(mut match_time: ResMut<MatchTimeCounter>) {
    match_time.stopwatch.pause();
}

// System to end the match after 5 minutes
fn end_match(
    mut commands: Commands,
    mut match_time: ResMut<MatchTimeCounter>
) {
    let match_time_length = match_time.stopwatch.elapsed();
    if match_time_length > Duration::from_mins(5) {
        commands.remove_resource::<MatchTimeCounter>();
    }
}

```

## System Timer Conditions

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

Using `DelayedCommands` will look very similar to using the regular `Commands`, although we have to insert the [`Commands::delayed`] method (which converts our `Commands` struct into a `DelayedCommands` wrapper).
Additionally, we have to specify an amount of time to delay the command we want to execute.

```rust
fn delayed_spawn(mut commands: Commands) {
    commands.delayed().secs(1.0).spawn(DummyComponent);
}
```

`DelayedCommands` can be set using either seconds (using [`DelayedCommands::secs`]) or a duration (using [`DelayedCommands::duration`]), much like `Timer`s and `Stopwatch`s can.
However, instead of needing to manually tick our `DelayedCommands`, Bevy will automatically tick them in a system run in the `PreUpdate` schedule.
All we have to do is provide the amount of time to delay our command by, and Bevy will handle the rest.

[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Commands.html
[`DelayedCommands`]: https://docs.rs/bevy/latest/bevy/time/delayed_commands/struct.DelayedCommands.html
[`Commands::delayed`]: https://docs.rs/bevy/latest/bevy/prelude/trait.DelayedCommandsExt.html#tymethod.delayed
[`DelayedCommands::secs`]: https://docs.rs/bevy/latest/bevy/time/struct.DelayedCommands.html#method.secs
[`DelayedCommands::duration`]: https://docs.rs/bevy/latest/bevy/time/struct.DelayedCommands.html#method.duration

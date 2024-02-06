+++
title = "Resources"
weight = 5
sort_by = "weight"
template = "docs-section.html"
page_template = "docs-section.html"
insert_anchor_links = "right"
aliases = ["learn/book/getting-started/resources"]
+++

**[`Entities`]** and **[`Component`]s** are great for representing complex, query-able groups of data. But most Apps will also require "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using **[`Resource`]s**.

Here are some examples of data that could be encoded as **[`Resource`]s**:

* Elapsed Time
* Asset Collections (sounds, textures, meshes)
* Renderers

[`Entities`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`Component`]: https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html
[`Resource`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.Resource.html

## Tracking Time with Resources

Let's solve our App's "hello spam" problem by only printing "hello" once every two seconds. We'll do this by using the [`Time`] resource, which is automatically added to our App via `add_plugins(DefaultPlugins)`.

For simplicity, remove the `hello_world` system from your App. This way we only need to adapt the `greet_people` system.

Resources are accessed in much the same way that we access components. You can access the [`Time`] resource in your system like this:

```rs,hide_lines=1
# use bevy::prelude::*;
fn greet_people(time: Res<Time>, query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
```

[`Res`] and [`ResMut`] pointers provide read and write access (respectively) to resources.

The `delta` field on [`Time`] gives us the time that has passed since the last update. But in order to run our system once every two seconds, we must track the amount of time that has passed over a series of updates. To make this easier, Bevy provides the [`Timer`] type. Let's create a new Resource for our system to track elapsed time with a [`Timer`]:

```rs,hide_lines=1
# use bevy::prelude::*;
#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
```

Now all that's left is adding a `GreetTimer` Resource to our `HelloPlugin`. Use [`TimerMode::Repeating`] to make the timer repeat.

```rs,hide_lines=1
# use bevy::prelude::*;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}
```

Now `cargo run` the App. It should now greet people at a reasonable rate.

[`Time`]: https://docs.rs/bevy_time/latest/bevy_time/struct.Time.html
[`Timer`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html
[`Res`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Res.html
[`ResMut`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.ResMut.html
[`TimerMode::Repeating`]: https://docs.rs/bevy/latest/bevy/time/enum.TimerMode.html#variant.Repeating

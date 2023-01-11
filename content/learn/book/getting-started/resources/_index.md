+++
title = "Resources"
weight = 5
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

**Entities** and **Components** are great for representing complex, query-able groups of data. But most Apps will also require "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using **Resources**.

Here are some examples of data that could be encoded as **Resources**:

* Elapsed Time
* Asset Collections (sounds, textures, meshes)
* Renderers

## Tracking Time with Resources

Let's solve our App's "hello spam" problem by only printing "hello" once every two seconds. We'll do this by using the {{rust_type(type="struct" crate="bevy_core" name="Time")}} resource, which is automatically added to our App via `add_plugins(DefaultPlugins)`.

For simplicity, remove the `hello_world` system from your App. This way we only need to adapt the `greet_people` system.

Resources are accessed in much the same way that we access components. You can access the `Time` resource in your system like this:

```rs
fn greet_people(time: Res<Time>, query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}
```

`Res` and `ResMut` pointers provide read and write access (respectively) to resources.

The `delta` field on `Time` gives us the time that has passed since the last update. But in order to run our system once every two seconds, we must track the amount of time that has passed over a series of updates. To make this easier, Bevy provides the `Timer` type. Let's create a new Resource for our system to track elapsed time with a `Timer`:

```rs
#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}
```

Now all that's left is adding a `GreetTimer` Resource to our `HelloPlugin`. Use `TimerMode::Repeating` to make the timer repeat.

```rs
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}
```

Now `cargo run` the App. It should now greet people at a reasonable rate.

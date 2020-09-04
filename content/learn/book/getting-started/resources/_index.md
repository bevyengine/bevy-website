+++
title = "Resources"
weight = 5
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

**Entities** and **Components** are great for representing complex, query-able groups of data. But most Apps will also require "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using **Resources**.

Here are some examples data that could be encoded as **Resources**:
* Elapsed Time
* Asset Collections (sounds, textures, meshes)
* Renderers

## Tracking Time with Resources

Lets solve our App's "hello spam" problem by only printing "hello" once every two seconds. We'll do this by using the {{rust_type(type="struct" crate="bevy_core" name="Time")}} resource, which is automatically added to our App via `add_default_plugins()`.

For simplicity, remove the `hello_world` system from your App. This way we only need to adapt the `greet_people` system.

Resources are accessed in much the same way that we access components. You can access the `Time` resource in your system like this:

```rs
fn greet_people(time: Res<Time>, person: &Person, name: &Name) {
    println!("hello {}!", name.0);
}
```

`Res` and `ResMut` pointers provide read and write access (respectively) to resources. Note that resources *must* come before components or your function will not be convertible to a System. I actually recommend that you try putting `time: Res<Time>` in an invalid position, just so you know what an "invalid system" compilation error looks like.

The `delta_seconds` field on `Time` gives us the time that has passed since the last update. But in order to run our system once every two seconds, we must track the amount of time that has passed over a series of updates. To make this easier, Bevy provides the `Timer` type. Lets create a new Resource for our system to track elapsed time with a `Timer`:

```rs
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, person: &Person, name: &Name) {
    // update our timer with the time elapsed since the last update
    timer.0.tick(time.delta_seconds);

    // check to see if the timer has finished. if it has, print our message
    if timer.0.finished {
        println!("hello {}!", name.0);
    }
}
```

Now all thats left is adding a `GreetTimer` Resource to our `HelloPlugin`:
```rs
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}
```

Now `cargo run` the App. It _almost_ looks how its supposed to, but not quite!

### The Bug

The good news is that we no longer have console spam. But it seems like we're printing names randomly and the timing seems wrong. Whats going on here?

The problem is that this system runs once _for each entity_ that has a `Person` and `Name` component. We have 3 entities that match this criteria, so on each update we are actually updating the timer 3 times. This also means that when a timer is "finished", the first entity to update resets the `GreetTimer` and the other entities don't print their message. This means our other two people never get a chance to be greeted. Thats not polite at all!

We need a way for our system to "tick" the timer once per update, but greet _everyone_ whenever the `Timer` is finished. Fortunately Bevy ECS has `Queries` for exactly this purpose!

Continue to the next section so we can fix this nasty bug!

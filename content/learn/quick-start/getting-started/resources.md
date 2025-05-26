+++
title = "Resources"
insert_anchor_links = "right"
aliases = ["learn/book/getting-started/resources"]
[extra]
weight = 5
+++

The **[`Entity`]** and **[`Component`]** data types are great for representing complex, query-able groups of data. But most Apps will also require "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using the **[`Resource`]** trait.

Here are some examples of data that could be encoded as a **[`Resource`]**:

* Elapsed Time
* Asset Collections (sounds, textures, meshes)
* Renderers

[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`Component`]: https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html
[`Resource`]: https://docs.rs/bevy/latest/bevy/ecs/resource/trait.Resource.html

## Tracking Time with Resources

Let's solve our App's "hello spam" problem by only printing "hello" once every two seconds. We'll do this by using the [`Time`] resource, which is automatically added to our App via `add_plugins(DefaultPlugins)`.

For simplicity, remove the `hello_world` system from your App. This way we only need to adapt the `greet_people` system.

Resources are accessed in much the same way that we access components. You can access the [`Time`] resource in your system like this:

{{file_code_block(file="quick-start/getting_started_v9.rs", anchor="greet_people_system")}}

[`Res`] and [`ResMut`] pointers provide read and write access (respectively) to resources.

The `delta` field on [`Time`] gives us the time that has passed since the last update. But in order to run our system once every two seconds, we must track the amount of time that has passed over a series of updates. To make this easier, Bevy provides the [`Timer`] type. Let's create a new Resource to track elapsed time with a [`Timer`]:

{{file_code_block(file="quick-start/getting_started_v10.rs", anchor="greet_timer_resource")}}

And use it in our system:

{{file_code_block(file="quick-start/getting_started_v10.rs", anchor="greet_people_system")}}

Now all that's left is adding a `GreetTimer` Resource to our `HelloPlugin`. Use [`TimerMode::Repeating`] to make the timer repeat.

{{file_code_block(file="quick-start/getting_started_v10.rs", anchor="hello_plugin_implementation")}}

Now `cargo run` the App. It should now greet people at a reasonable rate.

[`Time`]: https://docs.rs/bevy_time/latest/bevy_time/struct.Time.html
[`Timer`]: https://docs.rs/bevy/latest/bevy/time/struct.Timer.html
[`Res`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Res.html
[`ResMut`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.ResMut.html
[`TimerMode::Repeating`]: https://docs.rs/bevy/latest/bevy/time/enum.TimerMode.html#variant.Repeating

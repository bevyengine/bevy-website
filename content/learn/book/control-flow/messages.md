+++
title = "Messages"
insert_anchor_links = "right"
[extra]
weight = 12
status = 'hidden'
+++

<!-- TBW -->

What do we do when we want to communicate between systems, but we don't need to immediately react to those communications? We use **Messages**! [`Messages`] offer a communication channel between one or more systems. They can be created and read from using system parameters and are stored in the [`Messages<M>`] resource. 

You can use `Messages` for a variety of different functionalities. Some messages might re-occur over a longer period of time, like continuously applying damage effects or regularly updating a leaderboard. Others can be short-lived, such as checking for whether a player can interact with an object or if a UI element should appear. Bevy implements `Message` for a number of actions that are also `Events`, including [`GamepadEvent`], [`AssetEvent`], and even [`AppExit`] when the application closes. These allow you to add some deferred functionality in a `Message` in addition to using the `Event` for more immediate actions.

To start using messages, our first steps are derive the `Message` trait on a struct and then add the [`MessageWriter`] and [`MessageReader`] system parameters to some systems.

```rust
// Derive the `Message` trait on our Greeting struct.
#[derive(Message)]
struct Greeting(String);

// A system that writes a greeting message.
fn write_hello(mut writer: MessageWriter<Greeting>) {
    writer.write(Greeting("Hello!".to_string()));
}

// A system that will print out all Greeting messages.
fn read_messages(mut reader: MessageReader<Greeting>) {
    // Process all messages of type `Greeting`.
    for Greeting(greeting) in reader.read() {
        println!("{greeting}");
    }
}
```

Then we have to tell our application to begin handling `Greeting` messages. While we already derived the `Message` trait on `Greeting`, our application still needs some additional setup to get everything running. 

If we want the application to automatically handle all of our `Message` types, we can use the [`App::add_message<M>`] method. This automatically inserts a `Messages<M>` queue resource for the `M` message type we specify and schedules a [`message_update_system`] in the [`First`] schedule. `message_update_system` will call the [`Messages::update`] method for all `Message` types we've registered in the `World`.

```rust
fn main() {
    App::new()
        .add_plugins(...)
        // Register our message type.
        .add_message::<Greeting>()
        // Run our two systems with MessageWriter and MessageReader in order.
        .add_systems(
            // `chain` ensures that our systems run in the specified order.
            Update, (write_hello, read_messages).chain(),
        );
}
```

If you would prefer to dictate `Message` handling yourself, you are free to omit `App::add_message` and handle all of the functionality manually.

[`Messages`]: https://docs.rs/bevy/latest/bevy/prelude/trait.Message.html
[`Messages<M>`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Messages.html
[`MessageWriter`]: https://docs.rs/bevy/latest/bevy/prelude/struct.MessageWriter.html
[`MessageReader`]: https://docs.rs/bevy/latest/bevy/prelude/struct.MessageReader.html
[`App::add_message<M>`]: https://docs.rs/bevy/*/bevy/app/struct.App.html#method.add_message
[`message_update_system`]: https://docs.rs/bevy/latest/bevy/ecs/message/fn.message_update_system.html
[`First`]: https://docs.rs/bevy/latest/bevy/prelude/struct.First.html
[`Messages::update`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Messages.html#method.update
[`GamepadEvent`]: https://docs.rs/bevy/latest/bevy/input/gamepad/enum.GamepadEvent.html
[`AssetEvent`]: https://docs.rs/bevy/latest/bevy/prelude/enum.AssetEvent.html
[`AppExit`]: https://docs.rs/bevy/latest/bevy/prelude/enum.AppExit.html

## Messages Vs Events

At a glance it might seem like `Messages` and [`Events`] contain overlapping functionality, but they have some key distinctions. `Messages` are not processed immediately, instead they are usually only processed once per frame. This gives us some breathing room when compared to `Events` and `Observers` which will run immediately in reaction to being triggered.  Additionally, `Messages` have to be periodically polled for, typically as part of a specific `Schedule` that runs at various fixed points. `Events` on the other hand are executed sequentially either immediately if triggered by `World` or at the end of the `Schedule` if triggered with `Commands`.

While it might be tempting to always use `Events` and `Observers` to immediately update your application, `Messages` can be used much more efficiently in the right contexts. Processing a large number of `Messages` in a single batch is usually more efficient that triggering multiple `Events`. This is because a single `Message` can be consumed by multiple systems in parallel. Add in the fact that `Messages` are read at a predictable fixed point and we can start to see that unless we *need* to immediately react to something occuring in our application, using `Messages` is usually preferred.


[`Events`]: https://docs.rs/bevy/latest/bevy/prelude/trait.Event.html

## Altering Messages



## Message Lifespan

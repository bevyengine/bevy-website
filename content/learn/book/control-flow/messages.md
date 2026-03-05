+++
title = "Messages"
insert_anchor_links = "right"
[extra]
weight = 12
status = 'hidden'
+++

<!-- TBW -->

What do we do when we want to communicate between systems, but we don't need to immediately react to those communications? We use **Messages**! [`Messages`] offer a communication channel between one or more systems. They can be created and read from using system parameters and are stored in the [`Messages<M>`] resource. 

You can use `Messages` for a variety of different functionalities. Some messages might re-occur over a longer period of time, like continuously applying damage effects or regularly updating a leaderboard. Others can be short-lived, such as checking for whether a player can interact with an object or if a UI element should appear. The key concept to remember is that a `Message` is best used for logic that can be *deferred* rather than requiring immediate action.

To start using messages, our first steps are to derive the `Message` trait on a struct and then add the [`MessageWriter`] and [`MessageReader`] system parameters to some systems.

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

If we want the application to automatically handle all of our `Message` types, we can use the [`App::add_message<M>`] method. This automatically inserts a `Messages<M>` queue resource for the `M` message type we specify (`Greeting` in the above example) and schedules a [`message_update_system`] in the [`First`] schedule. `message_update_system` will call the [`Messages::update`] method for all `Message` types we've registered in the `World`.

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

If you would prefer to implement `Message` handling yourself, you are free to omit `App::add_message` and handle all of the functionality manually. Setting up `Message` handling manually follows the same process used by `App::add_message<M>`, however we are able to choose when `Messages::update` gets called.

```rust
// Our message type.
#[derive(Message)]
struct Greeting(String);

// Initialize and insert our message resource into the World.
fn setup_greeting(mut commands: Commands) {
    // Create a new instance of Messages<Greeting>.
    let greeting_messages = Messages::<Greeting>::default();
    // Insert the Messages<Greeting> into the World as a resource.
    commands.insert_resource(greeting_messages);
}

// Update Messages<Greeting> to collect new messages.
fn update_greeting_messages(greeting_res: Res<Messages<Greeting>>) {
    greeting_res.update();
}

fn main() {
    App::new()
        .add_plugins(...)
        // Insert our message resource in the Startup schedule.
        .add_systems(Startup, setup_greeting)
        // Update our message resource in the PreUpdate schedule.
        .add_systems(PreUpdate, update_greeting_messages)
}
```

[`Messages`]: https://docs.rs/bevy/latest/bevy/prelude/trait.Message.html
[`Messages<M>`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Messages.html
[`MessageWriter`]: https://docs.rs/bevy/latest/bevy/prelude/struct.MessageWriter.html
[`MessageReader`]: https://docs.rs/bevy/latest/bevy/prelude/struct.MessageReader.html
[`App::add_message<M>`]: https://docs.rs/bevy/*/bevy/app/struct.App.html#method.add_message
[`message_update_system`]: https://docs.rs/bevy/latest/bevy/ecs/message/fn.message_update_system.html
[`First`]: https://docs.rs/bevy/latest/bevy/prelude/struct.First.html
[`Messages::update`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Messages.html#method.update

## Reading & Writing Messages

Messages function based on *writing* them in response to something happening and *reading* them to perform some functionality as a result. Lets work through an example to showcase how and why messages should be used in your application. To do this we'll be creating a very basic scoreboard update mechanic for a king-of-the-hill style gamemode. Before we jump into the code, lets assess our objectives for the scoreboard update:

- Update each player's score based on if they control the hill.
- End the game when a player's score reaches 500.

For simplicity's sake, let's assume that we've already set up the systems that will tell us which player controls the hill and a mechanism to end the game once a player's score reaches 500. We can start by defining our `Message`, which in this case is called `ScoreboardUpdate` and contains a tuple tracking the player (`Entity`) and the update value (`i32`). We'll initialize and update it with `App::add_message` to keep things simple.

```rust
// Our ScoreboardUpdate message which will tell our ScoreboardUpdate to update the score
// for a player.
#[derive(Message)]
struct ScoreboardUpdate((Entity, i32));

fn main() {
    App::new()
        // A plugin containing our gamemode logic.
        .add_plugins(king_of_the_hill_plugin)
        // Initialize our ScoreboardUpdate message and update it in the First schedule.
        .add_message::<ScoreboardUpdate>();
}

```

Now lets *write* our message which will update the scoreboard. We'll do this by creating a `Single` query for the hill objective that our players are fighting for control of. Specifically we'll be looking for the player `Entity` value stored in a `CurrentHillKing` component, which is apart of an `Entity` with a `HillObjective` marker component. Once we have our player `Entity`, we'll write a new `ScoreboardUpdate` message containing the player and the value to update their score by (5 in our case).

```rust
// A system that will create a ScoreboardUpdate message.
fn write_scoreboard_update(
    // A MessageWriter to create a new message.
    mut update_writer: MessageWriter<ScoreboardUpdate>,
    // Our hill which contains the player in control of the hill.
    hill_query: Single<&CurrentHillKing, With<HillObjective>>,
) {
    // Read the Entity stored in the CurrentHillKing component on 
    // the HillObjective entity.
    let current_king = hill_query.0;
    // Create a new ScoreboardUpdate message with the Entity in 
    // control and a point value of 5.
    update_writer.write(ScoreboardUpdate(current_king, 5));
}
```

Next, we need a system to *read* our message and do something with the values we pass in. With our example, since `ScoreboardUpdate` contains a player `Entity` and a score `i32` value we'll access a `Scoreboard` resource and call a method to update the scores contained within. Additionally, we can check to see if any player's score is above 500, which is our condition for the match ending.

```rust
// A system that will read all of the ScoreboardUpdate messages and
// update the Scoreboard resource in response.
fn read_scoreboard_update(
    mut update_reader: MessageReader<ScoreboardUpdate>,
    mut scoreboard_res: ResMut<Scoreboard>,
) {
    // Read all of the ScoreboardUpdate messages in `update_reader`. 
    for score_update in update_reader.read() {
        scoreboard_res.update_score(score_update.player, score_update.value);
    }
    
    if scoreboard_res.highest_player_score() >= 500 {
        // If a player's score is above 500, end the game.
        println!("Game Over!");
    } else {
        // Otherwise clear the reader to prepare for the next update.
        update_reader.clear();
    }
}
```

Finally, we can add our systems into their respective schedules. Keep in mind that the system which will update our `ScoreboardUpdate` messages is going to be run in the `First` schedule, which is the first schedule ran on every new frame. As a result we'll want to schedule `read_scoreboard_update` *before* `write_scoreboard_update`. This prevents multiple `ScoreboardUpdate` messages being read at once and allows our gameplay systems to run with a freshly updated `Scoreboard` resource.

```rust

fn main() {
    App::new()
        .add_plugins(king_of_the_hill_plugin)
        .add_message::<ScoreboardUpdate>()
        // The `First` schedule runs before PreUpdate, which allows our messages to be updated.
        .add_systems(PreUpdate, read_scoreboard_update)
        // Once all of our gameplay systems ran in Update, then we can accurately write
        // a new ScoreboardUpdate message, which will be updated in the next frame.
        .add_systems(PostUpdate, write_scoreboard_update)
    ;
}
```

In reality, this setup is too simple to be used without further modifications. For example, because all of our schedules are run *every frame*, it wouldn't be practical to update an actual scoreboard like this. However, you should now be able to see how `Messages` can be used and how they are executed.

## Altering Messages



## Messages Vs Events

At a glance it might seem like `Messages` and [`Events`] contain overlapping functionality, but they have some key distinctions. `Messages` are not processed immediately, instead they are usually only processed once per frame. This gives us some breathing room when compared to `Events` and `Observers` which will run immediately in reaction to being triggered.  Additionally, `Messages` have to be periodically polled for, typically as part of a specific `Schedule` that runs at various fixed points. `Events` on the other hand are executed sequentially either immediately if triggered by `World` or at the end of the `Schedule` if triggered with `Commands`.

While it might be tempting to always use `Events` and `Observers` to immediately update your application, `Messages` can be more efficient than using `Events` in the right contexts. Processing a large number of `Messages` in a single batch is one such case. This is because a single `Message` can be *read* by multiple systems in parallel (although *writing* is still sequential). Add in the fact that `Messages` are read at a predictable fixed point and we can start to see that unless we *need* to immediately react to something occuring in our application, using `Messages` is usually preferred.

[`Events`]: https://docs.rs/bevy/latest/bevy/prelude/trait.Event.html

### Using Messages *With* Events

Bevy implements `Message` for a number of actions that are also `Events`, including [`GamepadEvent`] and other input events, [`AssetEvent`] when loading assets, and even [`AppExit`] when the application closes. These allow you to add some deferred functionality in a `Message` while also using an `Event` to perform more immediate actions.

[`GamepadEvent`]: https://docs.rs/bevy/latest/bevy/input/gamepad/enum.GamepadEvent.html
[`AssetEvent`]: https://docs.rs/bevy/latest/bevy/prelude/enum.AssetEvent.html
[`AppExit`]: https://docs.rs/bevy/latest/bevy/prelude/enum.AppExit.html




## Message Lifespan

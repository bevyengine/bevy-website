+++
title = "Messages"
insert_anchor_links = "right"
[extra]
weight = 12
status = 'hidden'
+++

<!-- TBW -->

When designing your Bevy applications, you might encounter situations where `Events` are evaluated *too* quickly.
Or, maybe you don't need to *immediately* perform some logic that an `Observer` will watch for.
There might even be some repeated functionality that you'll want to *defer* and accumulate before eventually processing.
These are the situations where **Messages** are the preferred tool.
[`Messages`] offer a communication channel for accumulating and efficiently processing many similar actions.
They can be created and read from using system parameters and are stored in a [`Messages<M>`] resource.

You can use `Messages` for a variety of different functionalities.
Some messages might re-occur over a longer period of time, like continuously applying damage effects or regularly updating a leaderboard.
Others can be short-lived, such as checking for whether a player can interact with an object or if a UI element should appear.
The key concept to remember is that a `Message` is best used for logic that can be *deferred* rather than requiring immediate action.

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

Then we have to tell our application to begin handling `Greeting` messages.
While we already derived the `Message` trait on `Greeting`, our application still needs some additional setup to get everything running.

If we want the application to automatically handle all of our `Message` types, we can use the [`App::add_message<M>`] method.
This automatically inserts a `Messages<M>` queue resource for the `M` message type we specify (`Greeting` in the above example) and schedules a [`message_update_system`] in the [`First`] schedule.
`message_update_system` will call the [`Messages::update`] method for all `Message` types we've registered in the `World`.

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

If you would prefer to implement `Message` handling yourself, you are free to omit `App::add_message` and handle all of the functionality manually.
Setting up `Message` handling manually follows the same process used by `App::add_message<M>`, however we are able to choose when `Messages::update` gets called.

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

Messages function based on *writing* them in response to something happening and *reading* them at a later point to perform some functionality.
Lets work through an example to showcase how and why messages should be used in your application.
To do this we'll be creating a very basic scoreboard update mechanic for a king-of-the-hill style gamemode.
Before we jump into the code, lets assess our objectives for the scoreboard update:

- Update each player's score based on if they control the hill.
- End the game when a player's score reaches 500.

For simplicity's sake, let's assume that we've already set up the systems that will tell us which player controls the hill and a mechanism to end the game once a player's score reaches 500.
We can start by defining our `Message`, which in this case is called `ScoreboardUpdate` and contains a tuple tracking the player (`Entity`) and the update value (`i32`).
We'll initialize and update it with `App::add_message` to keep things simple.

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

Now lets *write* our message which will update the scoreboard.
We'll do this by creating a `Single` query for the hill objective that our players are fighting for control of.
Specifically we'll be looking for the player `Entity` value stored in a `CurrentHillKing` component, which is apart of an `Entity` with a `HillObjective` marker component.
Once we have our player `Entity`, we'll write a new `ScoreboardUpdate` message containing the player and the value to update their score by (5 in our case).

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

Next, we need a system to *read* our message and do something with the values we pass in.
With our example, since `ScoreboardUpdate` contains a player `Entity` and a score `i32` value we'll access a `Scoreboard` resource and call a method to update the scores contained within.
Additionally, we can check to see if any player's score is above 500, which is our condition for the match ending.

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

Finally, we can add our systems into their respective schedules.
Keep in mind that the system which updates our `ScoreboardUpdate` message resource is going to be ran in the `First` schedule, which is the first thing ran on every new frame.
As a result we'll want to schedule `read_scoreboard_update` *before* `write_scoreboard_update`, but after the `First` schedule completes.
The `PreUpdate` schedule fits nicely for `read_scoreboard_update`, running before `Update` and well before `PostUpdate` which is where we'll place `write_scoreboard_update`.
This prevents multiple `ScoreboardUpdate` messages being read at once and allows our gameplay systems to run with a freshly updated `Scoreboard` resource.

```rust
fn main() {
    App::new()
        .add_plugins(king_of_the_hill_plugin)
        .add_message::<ScoreboardUpdate>()
        // The `First` schedule runs before PreUpdate, which allows our messages to be 
        // updated.
        .add_systems(PreUpdate, read_scoreboard_update)
        // Once all of our gameplay systems ran in Update, then we can accurately write
        // a new ScoreboardUpdate message, which will be updated in the next frame.
        .add_systems(PostUpdate, write_scoreboard_update)
    ;
}
```

In reality, this setup is too simple to be used without further modifications.
For example, because all of our schedules are run *every frame*, it wouldn't be practical to update an actual scoreboard like this.
However, you should now be able to see one way that `Messages` can be used and how they need to be arranged so that they will work as you intend them to.

## Altering Messages

We can do more than just write and read messages though. One of the best aspects of messages is that they're able to be *mutated* after we've already written them. [`MessageMutator`] gives you access to read and alter messages of a specific type. We can use it by declaring it as a system parameter in the same way that we use `MessageWriter` and `MessageReader`.

```rust
// Custom message type.
#[derive(Message, Debug)]
pub struct MyMessage(pub u32); 

// A system which reads and mutates all MyMessage messages.
fn my_system(mut mutator: MessageMutator<MyMessage>) {
    for message in mutator.read() {
        message.0 += 1;
        println!("received message: {:?}", message);
    }
}
```

One thing to note is that `MessageMutator` does not run in parallel.
This is because `MessageMutator` mutably accesses the `Messages<M>` resource that contains all of the messages of a given type.
The same applies for `MessageWriter` as well.
Systems accessing either will only run sequentially with each other.
On the other hand, `MessageReader` *can* be accessed by multiple systems concurrently if they only access `MessageReader`.
Although these systems still cannot run concurrently with systems accessing `MessageMutator` or `MessageWriter`.

Ultimately `MessageWriter`, `MessageReader`, and `MessageMutator` are all accessing the `Messages<M>` resource for a given `Message` type.
If you find that there is some functionality that these three system parameters cannot perform with their supplied methods, you always have the option of accessing the `Messages<M>` resource itself using `Res` or `ResMut`. Simply access it as a system parameter like you would any other `Resource`.

```rust
// Custom message type.
#[derive(Message, Debug)]
pub struct MyMessage(pub u32); 

fn message_resource(mut messages: ResMut<Messages<MyMessage>>) {
    // Drain all of the messages out of the resource, 
    // and remove any message with a value of 2.
    let filtered_messages = messages.drain().filter(|message| message.0 != 2);
    
    // Add the filtered messages back into the resource.
    messages.write_batch(filtered_messages);
}
```

[`MessageMutator`]: https://docs.rs/bevy/latest/bevy/ecs/message/struct.MessageMutator.html

## Messages Vs Events

At a glance it might seem like `Messages` and [`Events`] contain overlapping functionality, but they have some key distinctions.
`Messages` are not processed immediately, instead they are only processed at a specific moment each frame.
We can write and read messages at any point, but we can only access the updated values once the relevant `Messages<M>` resource has been updated.

This gives us some breathing room when compared to `Events` and `Observers`.
`Events` are executed sequentially either immediately if triggered by `World` or at the end of the `Schedule` if triggered with `Commands`.
If we trigger multiple `Events` within a frame, all of those events will be evaluated within that frame.

With this in mind, lets look at a scenario where `Messages` can be more efficient than using `Events`.

```rust
#[derive(Message, Debug)]
pub struct TickDamage {
    entity: Entity,
    damage: i32,
}

fn deal_tick_damage(mut tick_damage_reader: MessageReader<TickDamage>, mut health_query: Query<&mut Health>) {
    for tick_damage in tick_damage_reader.par_read() {
        if let Some(mut health) = health_query.get_mut(tick_damage.entity) {
            health.value -= tick_damage.damage;
        }
    }
    tick_damage_reader.clear();
}
```

In the above example, we have a `TickDamage` message that tracks an `Entity` and a damage amount (`i32`).
Throughout our schedules we can use a `MessageWriter` to accumulate `TickDamage` messages.
When `Messages<TickDamage>` gets updated, we can then apply all of the `TickDamage` messages at once, benefitting from the parallel access that `MessageReader` gives us.

Reproducing the same behavior using `Events` would involve several more steps.
First we would have to first track all of our entities separately, since we only want to apply `TickDamage` to our entities at a single point.
Then we would have to queue each `Event` to be evaluated sequentially.
As the number of entities we're applying `TickDamage` to grows larger and larger, the more time it will take to execute those `Events`.

By using messages, we avoid the headache of making sure entities are tracked separately and gain stability by avoiding sequential `Event` execution.

[`Events`]: https://docs.rs/bevy/latest/bevy/prelude/trait.Event.html

## Message Lifespan

We've mentioned that one of the benefits of using messages is that we can *defer* processing them until a later point.
This might raise several questions. How long do messages exist for? Can we defer processing them indefinitely? When are messages dropped or deleted?

The short answer is that every `Message` stored in a `Messages<M>` resource is accessible for up to two `Messages::update` method calls.

- You can access a `Message` after it is created, before a `Messages::update` is called.
- The initial `Messages` are then moved into a different section of memory (but still accessible) after the first `Messages::update` is called.
- After a second `Messages::update` is called, the initial set of `Messages` is dropped by the application.

This is a variation of a double buffer strategy.
New messages are placed into a buffer, and are made available for systems to read.
Whenever the `Messages::update` method is called, that buffer is swapped with a second buffer.
Messages are dropped after a second `Messages::update` is called, clearing the buffer and freeing up memory to be used for a new set of messages.
If `Messages::update` is never called, these buffers will continue to grow in size, and can cause performance issues for your application.

### Messages in FixedUpdate

Using `App:add_message<M>()` to register your `Message` will run `Messages::update` *every frame*.
Like we mentioned at the top, this is because the `message_update_system` is placed in the `First` schedule, meaning it will be the first thing updated every frame.
However we aren't locked into this behaviour.
If we instead want our `Messages` to be updated within the [`FixedUpdate`] schedule rather than being frame-dependent, we can do so by several means.

The first would be to manually place `message_update_system` in the `FixedUpdate` schedule (or some other schedule within the [`FixedMain`] set).

```rust
fn main() {
    App::new()
        // The message_update_system will update all Messages 
        // registered in your application.
        .add_systems(FixedUpdate, message_update_system);
}
```

However, this is indiscriminant and will update *every* `Message` type in your application.
Instead, it's likely that you'll want more fine-grained control over which `Messages` update in `FixedUpdate` versus those that update every frame.
We'll have to access each individual `Messages<M>` resource to accomplish this.

```rust
// This Message will update in the FixedUpdate schedule (every update).
#[derive(Message, Debug)]
pub struct TickDamage {
    entity: Entity,
    damage: i32,
}

// This message will update in the Update schedule (every frame).
#[derive(Message, Debug)]
pub struct WarnTickDamage {
    entity: Entity,
}

// This system will update the Message<TickDamage> resource.
fn deal_tick_damage_update(mut tick_damage_messages: ResMut<Messages<TickDamage>>) {
    tick_damage_messages.update();
}

// This system will read from the Message<TickDamage> resource 
// and apply damage to entities.
fn deal_tick_damage(mut tick_damage_reader: MessageReader<TickDamage>, mut health_query: Query<&mut Health>) {
    for tick_damage in tick_damage_reader.par_read() {
        if let Some(mut health) = health_query.get_mut(tick_damage.entity) {
            health.value -= tick_damage.damage;
        }
    }
    tick_damage_reader.clear();
}

// This system will update the Message<WarnTickDamage> resource.
fn warn_tick_damage_update(mut warn_messages: ResMut<Messages<WarnTickDamage>>) {
    warn_message.update();
}

// This system will read from the Message<WarnTickDamage> resource 
// and print a warning message.
fn warn_tick_damage(warn_tick_damage_reader: MessageReader<WarnTickDamage>) {
    for message in warn_tick_damage_reader.par_read() {
        println!("{} will take Tick Damage!", message.entity);
    }
}

// This system will add the Message<TickDamage> and Message<WarnTickDamage> 
// resources to the world.
fn add_messages(mut commands: Commands) {
    let warn_message = Messages::<WarnTickDamage>::default();
    let tick_damage_message = Messages::<TickDamage>::default();
    
    commands.insert_resource(warn_message);
    commands.insert_resource(tick_damage_message);
}

fn main() {
    App::new()
        .add_systems(Startup, add_messages)
        // Manually update TickDamage messages in the FixedUpdate schedule.
        .add_systems(FixedUpdate, (deal_tick_damage_update, deal_tick_damage).chain())
        // Manually update WarnTickDamage messages in the Update schedule.
        .add_systems(Update, (warn_tick_damage_update, warn_tick_damage).chain());
}
```

By placing `warn_tick_damage_update` in `Update`, it gets ran every frame. In contrast, `deal_tick_damage_update` is placed in `FixedUpdate`, meaning that it will run at a consistent interval instead of every frame.

[`FixedUpdate`]: https://docs.rs/bevy/latest/bevy/prelude/struct.FixedUpdate.html
[`FixedMain`]: https://docs.rs/bevy/latest/bevy/app/struct.FixedMain.html

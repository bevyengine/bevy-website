+++
title = "Entities, Components and Systems (new draft)"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 13
status = 'hidden'
+++

Congratulations, you've made it through the introduction and you've successfully installed Bevy. Let's write a simple app (pun intended).

```rust
use bevy::app::App;

fn main() {
    let mut app = App::new();
}
```
This compiles, and does ... nothing.
Well what did we actually do here. App is the global Bevy state.
It holds all information the we need for our game/application to run.
Here is where we can register all of our logic to be run.

The paradigm that is used is the Entity-Component-System paradigm.
In short, we have entities, which are unique instances of any object we want to represent.
Think of it as the camera, the player, the attack projectiles.

Each entity we can assign a finite set of components. In our example
we can attach a Position component to the camera. 
Or we can assign a health component to the player that keeps track of the total hitpoints.

Finally we have systems, those are functions which operate on those components.

#### Systems

Let us start with the simplest case by adding a function.
```rust
use bevy::app::{App, Startup};


fn main() {
    let mut app = App::new();
    app.add_systems(Startup, system_function);
    app.update();
}

fn system_function() {
    println!("I am running during startup.");
}
```
What has happened?
We have registered the system_function to run during startup. 
And on the next line we execute the application to advance one step of its lifecycle.

Let us add another function, which runs during update.
```rust
use bevy::app::{App, Startup, Update};

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, system_function);
    app.add_systems(Update, update_function);
    app.update();
    app.update();
}

fn system_function() {
    println!("I am a system.");
}

fn update_function() {
    println!("I run during the update lifecycle.");
}
```
Can you guess what the output is? That's right, first we enter the Startup Schedule once, and only once. We get the console output of "I am a system." After that we trigger the update schedule on every invocation of the app being updated.
Therefore we get "I run during the update lifecycle." twice.

Now that seems like a convoluted way of calling functions. We just renamed them to systems and we are using an indirection.
And we didn't even mention components and entities.

#### Entities and Components


##### Add players and health 

We'll update our previous example to introduce a Player Entity, which has a Health Component and a Name Component.

We'll also modify our startup function to spawn a player Bob with 100 HP and 50 Mana.

In the update we print the player name and his stats.
Can you guess what the following code snippet is doing?

```rust
use bevy::{app::{App, Startup, Update}, ecs::{bundle::Bundle, component::Component, system::{Commands, Query}}};

#[derive(Component)]
struct HealthComponent {
    health: i32,
    mana: i32,
}

#[derive(Component)]
struct NameComponent {
    name: String,
}

#[derive(Bundle)]
struct Player {
    health: HealthComponent,
    name: NameComponent,
}

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_player);
    app.add_systems(Update, print_health);
    app.update();
    app.update();
}

fn spawn_player(mut command: Commands) {
    let health = HealthComponent {health:100, mana:50};
    let name = NameComponent {name:"Bob".to_owned()};
    let player = Player {health,name};
    command.spawn(player);
}

fn print_health(query: Query<(&NameComponent,&HealthComponent)>) {
    for query_entry in query.iter() {
        let name = query_entry.0.name.as_str();
        let hp = query_entry.1.health;
        let mana = query_entry.1.mana;
        println!("Player {} has {} HP and {} Mana",name,hp,mana);
    }
}
```

That's right the console output is:
```
$ Player Bob has 100 HP and 50 Mana.
$ Player Bob has 100 HP and 50 Mana.
```

So what has happened: We have introduced four new concepts, that are reflected in the code as follows.

##### #Derive(Component) macro

This one is quite simple, we are registering the Health and Name components to actually be Bevy components.

Here is the expanded macro to actually see what's going on.
```rust
impl bevy::ecs::component::Component for HealthComponent
where
    Self: Send + Sync + 'static,
{
    const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
    fn register_required_components(
        requiree: bevy::ecs::component::ComponentId,
        components: &mut bevy::ecs::component::Components,
        storages: &mut bevy::ecs::storage::Storages,
        required_components: &mut bevy::ecs::component::RequiredComponents,
        inheritance_depth: u16,
    ) {}
    #[allow(unused_variables)]
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {}
}
```
If we execute 
```
grep -r "register_required_components" .
```
inside the Bevy source directory we can find the files where the components are getting registered. If we remove the derive Component impl and print a backtrace we can see where the framework initialises our Component.
Let us use a manual implementation to print the backtrace by adding the backtrace inside the register_required_components impl. For simplicity we only display the last two traces.
```
   0: <bevly::HealthComponent as bevy_ecs::component::Component>::register_required_components
             at ./src/main.rs:22:18
   1: bevy_ecs::component::Components::register_component
             at /home/user/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.15.1/src/component.rs:917:13
```
The function that is responsible for registering the Component is update itself.
```
  68: bevy_app::app::App::update
             at /home/user/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_app-0.15.1/src/app.rs:139:9
  69: bevly::main
             at ./src/main.rs:46:5
``` 
This is done by the Rust type checker. It has bounds on the types of system functions, and it also has bounds on the type of the inner types of the Query.
In fact, [command.spawn()](https://docs.rs/bevy/latest/bevy/prelude/struct.Commands.html#method.spawn)  has a bound on a bundle type, and the bundle type has a bound on a component type. The typechecker thus concludes that the corresponding impls exists and calls a monomorphised version of [register_component()](https://docs.rs/bevy/latest/bevy/ecs/component/struct.Components.html#method.register_component).
All in all, we don't need to worry about it that much except that it works.

The takeaway is, if we need a component, just use the derive macro and the framework/compiler will do the rest for you.

##### #Derive(Bundle) macro

Similar things can in principle be done for the Bundle macro. A bundle is an Entity, which contains zero to N components. Deriving the bundle lets us spawn entities with given component values.

##### Commands System parameter

Commands is one of many valid system parameters, i.e. the parameters that can be part of system functions. It lets us spawn/despawn entities. For now, our only function we will use with Commands is [commands.spawn()](https://docs.rs/bevy/latest/bevy/prelude/struct.Commands.html#method.spawn)


##### Query System parameter

This is the power of the ECS. Query is very featureful. For now, we shall only query shared and exclusive references.


#### Modifying player health

Let us add another system that periodically removes 25 HP of every player. 
Think about it as a global degen field that affects everything that has a health component attached to it.
```rust
use bevy::{app::{App, Startup, Update}, ecs::{bundle::Bundle, component::Component, system::{Commands, Query}}};

#[derive(Component)]
struct HealthComponent {
    health: i32,
    mana: i32,
}

#[derive(Component)]
struct NameComponent {
    name: String,
}

#[derive(Bundle)]
struct Player {
    health: HealthComponent,
    name: NameComponent,
}

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_player);
    app.add_systems(Update, print_health);
    app.add_systems(Update, degen);
    app.update();
    app.update();
}

fn spawn_player(mut command: Commands) {
    let health = HealthComponent {health:100, mana:50};
    let name = NameComponent {name:"Bob".to_owned()};
    let player = Player {health,name};
    command.spawn(player);
}


fn print_health(query: Query<(&NameComponent,&HealthComponent)>) {
    for query_entry in query.iter() {
        let name = query_entry.0.name.as_str();
        let hp = query_entry.1.health;
        let mana = query_entry.1.mana;
        println!("Player {} has {} HP and {} Mana.",name,hp,mana);
    }
}

fn degen(mut query: Query<&mut HealthComponent>) {
    for query_entry in query.iter_mut() {
        let health = query_entry.into_inner();
        health.health -=25;
    }
}
```
Can you guess what the output is?
If you said 100 HP, then 75 HP you'd be wrong, or right, we don't know.
On my machine the output was
```
$ Player Bob has 75 HP and 50 Mana.
$ Player Bob has 50 HP and 50 Mana.
```
So it appears that the degen system was executed before the print_health system, even though that it was registered after.
This is because by default systems run independently of each other, and we didn't specify any order.

Let us fix that by specifying that degen should run after print_health.

Change the corresponding line to 
```rust
    app.add_systems(Update, degen.after(print_health));
```
We have now specified that degen runs after print_health and the output
```
$ Player Bob has 100 HP and 50 Mana.
$ Player Bob has 75 HP and 50 Mana.
```
confirms that.

We still have to call update manually, in the next section we will introduce a basic plugins that will do that work for us.

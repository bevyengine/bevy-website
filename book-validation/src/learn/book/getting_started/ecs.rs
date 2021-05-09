use bevy::prelude::*;
// ANCHOR: component-normal-struct
struct Position {x: f32, y: f32}
// ANCHOR_END: component-normal-struct

// ANCHOR: system-normal-function
fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}
// ANCHOR_END: system-normal-function

// ANCHOR: entity-type-integer
struct Entity(u64);
// ANCHOR_END: entity-type-integer

// ANCHOR: first-system
fn hello_world() {
    println!("hello world!");
}
// ANCHOR_END: first-system

mod main_first_system {
use super::*;
// ANCHOR: add-to-app
fn main() {
    App::build().add_system(hello_world.system()).run();
}
// ANCHOR_END: add-to-app
}


// ANCHOR: component-person
struct Person;
// ANCHOR_END: component-person

// ANCHOR: component-name
struct Name(String);
// ANCHOR_END: component-name

// ANCHOR: system-add_people
fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}
// ANCHOR_END: system-add_people

mod main_add_people {
use super::*;
// ANCHOR: main-app
fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .run();
}
// ANCHOR_END: main-app
}
// ANCHOR: system-greet_people
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}
// ANCHOR_END: system-greet_people
mod main_greet_people {
use super::*;
// ANCHOR: main-app-2
fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}
// ANCHOR_END: main-app-2
}
use bevy::prelude::*;

// ANCHOR: person_component
#[derive(Component)]
struct Person;
// ANCHOR_END: person_component

#[expect(dead_code)]
// ANCHOR: name_component
#[derive(Component)]
struct Name(String);
// ANCHOR_END: name_component

// ANCHOR: add_people_system
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
// ANCHOR_END: add_people_system

fn hello_world() {
    println!("hello world!");
}

// ANCHOR: app_main
fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, hello_world)
        .run();
}
// ANCHOR_END: app_main

use bevy::prelude::*;

// ANCHOR: hello_world
fn hello_world() {
    println!("hello world!");
}
// ANCHOR_END: hello_world

// ANCHOR: app_main
fn main() {
    App::new().add_systems(Update, hello_world).run();
}
// ANCHOR_END: app_main

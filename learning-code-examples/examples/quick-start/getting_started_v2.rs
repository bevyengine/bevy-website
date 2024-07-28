use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new().add_systems(Update, hello_world).run();
}

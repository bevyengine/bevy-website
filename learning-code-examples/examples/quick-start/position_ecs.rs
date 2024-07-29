use bevy::prelude::*;

// ANCHOR: position_component
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
// ANCHOR_END: position_component

// ANCHOR: position_system
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}
// ANCHOR_END: position_system

fn main() {
    App::new().add_systems(Update, print_position_system).run();
}

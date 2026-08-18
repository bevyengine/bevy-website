#![expect(dead_code)]
use bevy::prelude::*;

// ANCHOR: components
/// The grid-based location of a player, creature, or object in our game.
#[derive(Component)]
struct Location {
    x: u32,
    y: u32,
}

/// The color of an object in our game.
#[derive(Component)]
enum Color {
    Red,
    Green,
    Blue,
    Heliotrope,
}

/// A "marker" component for entities which represents a player.
/// Since this contains no data, this is more like a tag.
#[derive(Component)]
struct Player;
// ANCHOR_END: components

impl Location {
    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    App::new()
        .add_systems(Startup, spawn_entities)
        .add_systems(Update, my_system)
        .run();
}

// ANCHOR: spawn_entities
fn spawn_entities(mut commands: Commands) {
    // Spawn an entity with all our components
    commands.spawn((Location::zero(), Color::Red, Player));
    // Spawn an entity with only one component
    commands.spawn(Color::Heliotrope);
}
// ANCHOR_END: spawn_entities

// ANCHOR: systems
// No derive macro needed!
fn my_system(mut entities: Query<&mut Location>) {
    for mut location in entities.iter_mut() {
        location.x += 1;
    }
}
// ANCHOR_END: systems

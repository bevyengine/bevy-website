use bevy::prelude::*;
use std::marker::PhantomData;
// ANCHOR: generic_plugin
// Example plugin with a generic type
pub struct YourPlugin<T> {
    pub phantom_t: PhantomData<T>,
}

// Implementation of YourPlugin as a Plugin with
// a generic type parameter of the `Component` trait.
impl<T: Component> Plugin for YourPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, example_system::<T>);
    }

    // …your other logic here…
}

impl<T> YourPlugin<T> {
    pub fn new() -> Self {
        Self::default()
    }

    // …your other logic here…
}

impl<T> Default for YourPlugin<T> {
    fn default() -> Self {
        Self {
            phantom_t: PhantomData,
        }
    }
}

// Example component.
#[derive(Component)]
pub struct Something;

// Example system using the generic type parameter
// of `Component` trait.
pub fn example_system<T: Component>(query: Query<&T>) {
    for _example_component in &query {
        // …do something…
    }
}
// ANCHOR_END: generic_plugin

fn main() {
    App::new().add_plugins(YourPlugin::<Something>::new()).run();
}

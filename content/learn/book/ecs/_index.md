+++
title = "Entities, components and systems"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

Bevy is fundamentally powered by its ECS (a central [paradigm](https://ajmmertens.medium.com/ecs-from-tool-to-paradigm-350587cdf216) for organizing and operating on data which stands Entity-Component-System): almost all data is stored as components which belong to entities, and all logic is executed by its systems.

We can think of our **entity-component data storage** as a giant in-memory database:

* each row is an **entity**, representing an object (perhaps a player, tile, or UI button) in our game
* each column is a type of **component**, storing data of a particular type (perhaps the sprite, team or life of a player entity) in an [efficient way](https://github.com/bevyengine/bevy/pull/1525) that keeps data of the same type tightly packed together
* each cell is a component of a particular entity, which has a concrete value we can look up and change
* we access data from this database using **queries**, which fetch component data from entities with the specified components
* the primary key of this database is the [`Entity`] identifier, which can be used to look up specific entities using [`Query::get`]

Of course, this database is [very ragged](https://www.transdatasolutions.com/what-is-ragged-data/): not all entities will have every component!
We can use this fact to specialize behavior between entities: systems only perform work on entities with the correct combination of components.
You don't want to apply gravity to entities without a position in your world, and you're only interested in using the UI layout algorithm to control the layout of UI entities!

When we want to go beyond this tabular data storage, we can use **resources**: global singletons which store data, each in their own monolithic blob.
You might use resources to interface with other libraries, store unique bits of state like the game's score, or store secondary data structures like indexes to augment your use of entity-component data.

In order to manipulate and act on this data, we must use systems.
**Systems** are Rust functions that request specific data, such as resources and entities, from the [`World`]. They define a query in their parameters (arguments) that selects data with a particular combination of components.
All of the rules and behaviours of our game are governed by systems.

Once the systems are added to our app, the **runner** takes this information and automatically runs our systems, typically once during each pass of the **game loop** according to the rules defined in their **schedule**.

Bevy's default execution strategy runs systems in parallel by default, without the need for any manual setup.
Because the **function signature** of each of our systems fully define the data it can access, we can ensure that only one system can change a piece of data at once (although any number can read from a piece of data at the same time).

Systems within the same **stage** are allowed to run in parallel with each other (as long as their data access does not conflict), and are assigned to a thread to perform work as soon as one is free.

When we need to access data in complex, cross-cutting ways that are not cleanly modelled by our systems' function signatures, we can defer the work until we have exclusive access to the entire world's data: executing **commands** generated in earlier systems at the end of each stage or performing complex logic (like saving the entire game) in our own **exclusive systems**.
You will first encounter this when spawning and despawning entities: we have no way of knowing precisely which other components our entities might have, and so we are forced to wait until we can ensure that we can safely write to *all* component data at once.

## ECS by example

Before we dive into the details of each of these features, let's take a quick look at a simple but fun whack-a-mole game that you can run and play.
Unsurprisingly, the different parts of the ECS tend to be closely linked: components are not very useful without a way to spawn entities and systems that run our logic are very dull if we can't discuss the data they can access.
The details of each part are more easily grasped if you have a basic sense of the whole.

```rust
use bevy::prelude::*;

// This is a fast but insecure HashMap (dictionary, for those coming from Python)
// implementation that Bevy re-exports for internal and external use
use bevy::utils::HashMap;

// We want to randomize our moles,
// so we're also using the `rand` crate.
// Be sure to add `rand = "0.8"` to your Cargo.toml
use rand::distributions::{Distribution, Uniform};
use rand::seq::IteratorRandom;

// The main function defines all the code that our program will run
// in an imperative fashion
fn main() {
    // The App stores entities, their components and all resources in the World
    // and systems in its Schedule
    App::new()
        // Sets up the graphics and input that we need for this example
        .add_plugins(DefaultPlugins)
        // We only want to run this once, at the start of the app, so it's a startup system
        .add_startup_system(spawn_camera_system)
        // This resource is explicitly initialized with a new value
        .insert_resource(Score(0))
        // These systems all run once per frame,
        // in parallel if possible
        .add_system(report_score_system)
        // This resource is initialized using its FromWorld trait impl
        .init_resource::<MoleColors>()
        // This resource is initialized using its Default trait impl
        .init_resource::<SpawnTimer>()
        .add_system(spawn_mole_system)
        .add_system(despawn_on_timer_system)
        .add_system(whack_system)
        .run();
}

/// Resource that stores the current score
struct Score(isize);

/// Creates a standard 2D camera
fn spawn_camera_system(mut commands: Commands) {
    // This spawns a new entity for our camera
    // with the set of components defined by the OrthographicCameraBundle
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

/// Reports the value of the Score resource for the player
fn report_score_system(score: Res<Score>) {
    // We only want to report the new score when it has changed
    if score.is_changed() {
        println!("Score: {}", score.0);
    }
}

const MOLE_SIZE: f32 = 50.0;

/// Marker component for our moles
#[derive(Component)]
struct Mole;

/// Component that stores how long this entity has left to live
#[derive(Component)]
struct DespawnTimer(Timer);

impl Default for DespawnTimer {
    fn default() -> Self {
        const MOLE_LIFESPAN: f32 = 5.0;
        DespawnTimer(Timer::from_seconds(MOLE_LIFESPAN, false))
    }
}

/// Component that determines how many points each mole is worth
// Enums can be added as components!
// We need the additional traits to allow us to use this type as a key in our MoleColors hashmap
#[derive(Component, PartialEq, Eq, Hash, Clone, Copy)]
enum MoleType {
    Avoid,
    Click,
    Prioritize,
}

impl MoleType {
    /// The number of points each type of mole is worth when clicked
    fn points(&self) -> isize {
        match *self {
            MoleType::Avoid => -10,
            MoleType::Click => 1,
            MoleType::Prioritize => 5,
        }
    }

    /// The color each type of mole will have when spawned
    fn color(&self) -> Color {
        match *self {
            MoleType::Avoid => Color::RED,
            MoleType::Click => Color::BLUE,
            MoleType::Prioritize => Color::VIOLET,
        }
    }

    /// Lists the types of moles for easy iteration
    fn types() -> impl Iterator<Item = MoleType> {
        [MoleType::Avoid, MoleType::Click, MoleType::Prioritize]
            .iter()
            // This allows us to return MoleType as our item,
            // rather than just a reference to the objects in this array (&MoleType)
            .copied()
    }
}

/// A resource to store the colors for our moles in an efficient way
struct MoleColors {
    map: HashMap<MoleType, Handle<ColorMaterial>>,
}

// This trait allows us to initialize a resource in more complex ways
impl FromWorld for MoleColors {
    fn from_world(world: &mut World) -> Self {
        // Here, we're grabbing the Assets<ColorMaterial> resource directly from the world
        // so we can record the materials that we create in it
        let mut material_assets = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let mut mole_colors = MoleColors {
            map: HashMap::default(),
        };

        // The types() method on MoleTypes we made above returns an iterator,
        // so we can loop over it directly
        for mole_type in MoleType::types() {
            // Each mole type has its own color.
            // We need to create a ColorMaterial from our simple RGBA color
            let color_material: ColorMaterial = mole_type.color().into();
            // Then, we add that new color material to our assets collection,
            // storing the original handle created for it
            let material_handle = material_assets.add(color_material);
            // Assets are expensive and batching is critical for performance,
            // so we're storing reference-counted pointers to a single common asset
            // called "handles", then cloning references as needed
            mole_colors.map.insert(mole_type, material_handle);
        }

        // The last expression evaluated is implicitly (and idiomatically) returned in Rust
        mole_colors
    }
}

impl MoleColors {
    fn get_handle(&self, mole_type: MoleType) -> Handle<ColorMaterial> {
        // We need to clone a new handle for each new mole we create.
        // We can skip reference-counting using `clone_weak` rather than `clone`,
        // since we know that the colors stored in our resource will never be unloaded
        self.map
            .get(&mole_type)
            .expect("Missing color handle")
            .clone_weak()
    }
}

/// Resource that tracks when the next mole should be spawned
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        const SPAWN_PERIOD: f32 = 0.5;

        // The second value `true` sets the `repeating` field of the timer,
        // causing the timer to reset when it ticks down to 0
        SpawnTimer(Timer::from_seconds(SPAWN_PERIOD, true))
    }
}

/// Spawns a new random mole whenever the SpawnTimer has elapsed
fn spawn_mole_system(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    windows: Res<Windows>,
    time: Res<Time>,
    mole_colors: Res<MoleColors>,
) {
    // Advance the timer by the amount of time that elapsed during the last game loop
    spawn_timer.0.tick(time.delta());

    // .0 refers to the first (and in this case only) field,
    // of our SpawnTimer tuple struct: the Timer-type field
    if spawn_timer.0.just_finished() {
        // Initialize our pseudo-random number generator
        let mut rng = rand::thread_rng();

        // Grab the window so we can use its size to determine where the moles can spawn
        let window = windows.get_primary().expect("No window found!");

        // By default, the camera is centered on (0, 0)
        // and we don't want the boxes to go outside of the screen
        let x_limit = (window.width() - MOLE_SIZE) / 2.0;
        let y_limit = (window.height() - MOLE_SIZE) / 2.0;

        // Randomly determine properties of our new mole
        let mole_pos_x = Uniform::from(-x_limit..x_limit).sample(&mut rng);
        let mole_pos_y = Uniform::from(-y_limit..y_limit).sample(&mut rng);
        let mole_type = MoleType::types()
            .choose(&mut rng)
            .expect("No mole type could be selected.");

        // Spawn our new mole
        commands
            // This creates an empty entity
            .spawn()
            // First, we add a marker component to it
            // (in case we want to filter for only moles later)
            .insert(Mole)
            // Then, we add a collection of components called a bundle
            // to give our moles an associated graphical sprite
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(MOLE_SIZE, MOLE_SIZE)),
                // Get the appropriate material handle for the type of mole
                // to ensure consistent color-score correspondence
                material: mole_colors.get_handle(mole_type),
                transform: Transform::from_xyz(mole_pos_x, mole_pos_y, 0.0),
                // This is "struct update syntax", and sets all other values of the SpriteBundle struct
                // to their default values
                ..Default::default()
            })
            //Then, we add a timer to our mole so it will despawn automatically if not clicked
            .insert(DespawnTimer::default())
            // Finally, we add our MoleType component to the new entity,
            // with the appropriate randomly selected value
            .insert(mole_type);
    }
}

/// Automatically despawn anything with a DespawnTimer component when that timer elapses
fn despawn_on_timer_system(
    mut mole_query: Query<(Entity, &mut DespawnTimer)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut despawn_timer) in mole_query.iter_mut() {
        // Each mole has its own timer, allowing them to despawn independently
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.finished() {
            commands.entity(entity).despawn()
        }
    }
}

/// Handles mouse clicks and checks if you hit a mole
fn whack_system(
    mut commands: Commands,
    windows: Res<Windows>,
    mut score: ResMut<Score>,
    mouse_input: Res<Input<MouseButton>>,
    mole_query: Query<(Entity, &Transform, &MoleType), With<Mole>>,
) {
    // Get the mouse position in screen coordinates
    let window = windows.get_primary().unwrap();
    let win_size = Vec2::new(window.width(), window.height());
    // The cursor may not be within the window
    let maybe_screen_pos = window.cursor_position();

    // Now, we need to convert the position from screen coordinates to world coordinates
    // We declare this value outside of our branch to satisfy the compiler
    let world_pos = match maybe_screen_pos {
        Some(screen_pos) => screen_pos - win_size / 2.0,
        None => Vec2::default(),
    };

    // Check for moles if the mouse button is pressed
    if mouse_input.just_pressed(MouseButton::Left) {
        for (mole_entity, mole_transform, mole_type) in mole_query.iter() {
            //dbg!(mole_entity);
            //let did_hit = mole_hit(world_pos, mole_transform);
            //dbg!(did_hit);
            if mole_hit(world_pos, mole_transform) {
                // Each type of mole has its own point total
                score.0 += mole_type.points();
                // Despawn the mole once they've been clicked once
                commands.entity(mole_entity).despawn();
            }
        }
    }
}

// This helper function encapsulates our checking logic,
// making the system above easier to follow
fn mole_hit(cursor_pos: Vec2, mole_transform: &Transform) -> bool {
    // Bevy transforms are relative to the center of the sprite
    let mole_min_x = mole_transform.translation.x - 0.5 * MOLE_SIZE;
    let mole_max_x = mole_transform.translation.x + 0.5 * MOLE_SIZE;

    let mole_min_y = mole_transform.translation.y - 0.5 * MOLE_SIZE;
    let mole_max_y = mole_transform.translation.y + 0.5 * MOLE_SIZE;

    // If the cursor is within both the x and y range, it is within the box
    mole_min_x <= cursor_pos.x
        && cursor_pos.x <= mole_max_x
        && mole_min_y <= cursor_pos.y
        && cursor_pos.y <= mole_max_y
}
```

If you'd like to see more tiny but cohesive examples like this, check out our [game examples](https://github.com/bevyengine/bevy/tree/latest/examples/game) on the Bevy GitHub repository.

+++
title = "Manipulating entities with commands"
weight = 4
template = "book-section.html"
page_template = "book-section.html"
+++

**Commands** are powerful, unrestricted tools for modifying the `World` of your Bevy game.
Queue them up, and you can reshape the world in arbitrary ways when they're evaluated!

Commands are fundamentally designed to perform work that cannot be safely done in parallel, and are used to change the world in ways that touch large amounts of data at once (requiring exclusive access to the archetypes and other metadata of the `World`).
While you can check out the full list of options by reading the API docs for [`Commands`](https://docs.rs/bevy/0.5.0/bevy/ecs/system/struct.Commands.html) and [`EntityCommands`](https://docs.rs/bevy/0.5.0/bevy/ecs/system/struct.EntityCommands.html), commands are commonly used to:

- Spawn and despawn entities with `spawn`, `spawn_bundle` and ``commands.entity(my_entity).despawn`
- Add and remove resources dynamically with `init_resource`, `insert_resouce` and `remove_resource`
- Add and remove components to entities with `commands.entity(my_entity).insert` and `commands.entity(my_entity).remove`

Due to the limitations on the parallel execution of the operations performed using commands, **commands do not take effect immediately.**
This is, by and large, their defining characteristic: they must wait until the next **hard sync point** (where single function have access to the entire world in a sequential fashion) in order to be resolved as they require exclusive mutable access to the world.
Unless you are already operating in an exclusive way on the `World`, commands are typically going to be processed at the end of the current stage.

As a result, **avoid using commands unnecessarily**.
For example, you *could* use the overwriting behavior of component insertion to mutate components in place.
However, this is unclear, slow and takes delayed effect, making it strictly worse than just requesting the appropriate data and mutating it in an ordinary fashion.

## Manipulating entities with commands

Most of the time, you'll be using commands to modify entities.
Let's take a look at the details of that, beginning with the various ways to spawn and despawn entities.

```rust
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera.system())
        .add_startup_system(spawn_button.system())
        .add_system(despawn_on_click.system())
        .run()
}

fn spawn_camera(mut commands: Commands) {
    // spawn_bundle spawns an entity with a particular bundle as its components
    commands.spawn_bundle(UiCameraBundle::default());
}

struct ButtonMarker;

fn spawn_button(mut commands: Commands) {
    // .spawn() creates a new entity with no components
    // You can chain .insert and .insert_bundle to add additional components to spawned bundles
    commands
        .spawn()
        .insert_bundle(ButtonBundle {
            style: Style {
                // Set button size
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // Center button
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ButtonMarker);
}

fn despawn_on_click(
    query: Query<(Entity, &Interaction), With<ButtonMarker>>,
    mut commands: Commands,
) {
    for (entity, interaction) in query.iter() {
        if *interaction == Interaction::Clicked {
            // When you want to interact with a particular entity using commands,
            // select the appropriate entity with Commands::entity()
            // Then, you can call EntityCommands like despawn on that entity
            commands.entity(entity).despawn();
        }
    }
}
```

When you want to spawn large numbers of entities at once in an efficient way, use [`spawn_batch`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html#method.spawn_batch):

```rust
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera.system())
        .add_startup_system(spawn_lines.system())
        .run()
}

fn new_line(i: u8, material_handle: Handle<ColorMaterial>) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite::new(Vec2::new(10.0, 200.0)),
        transform: Transform::from_xyz(i as f32 * 50.0, 0.0, 1.0),
        material: material_handle,
        ..Default::default()
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn spawn_lines(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material_handle = materials.add(Color::PINK.into());

    // spawn_batch accepts any object which can be turned into an iterator
    // which returns a Bundle in each item
    // and creates one entity for each item in that iterator
    commands.spawn_batch((1..9).map(move |i| new_line(i, material_handle.clone())));
}
```

Finally, let's examine how we could use `EntityCommands::insert()` and `EntityCommands::remove()` to dynamically add and remove components.

```rust
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<WhiteMaterial>()
        .init_resource::<PurpleMaterial>()
        .add_startup_system(spawn_camera.system())
        .add_startup_system(spawn_button.system())
        .add_system(purplify_on_click.system())
        .add_system(enforce_purple.system())
        .run()
}

// These resources store persistent handles to our white and purple materials
// See the chapter on Assets for more details on how this works
struct WhiteMaterial(Handle<ColorMaterial>);
struct PurpleMaterial(Handle<ColorMaterial>);

impl FromWorld for WhiteMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let handle = materials.add(Color::WHITE.into());
        WhiteMaterial(handle)
    }
}

impl FromWorld for PurpleMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let handle = materials.add(Color::PURPLE.into());
        PurpleMaterial(handle)
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_button(mut commands: Commands) {
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(150.0)),
            margin: Rect::all(Val::Auto),
            ..Default::default()
        },
        ..Default::default()
    });
}

/// Simple marker component to dictate whether our button should be purple or not
struct Purple;

fn purplify_on_click(
    query: Query<(Entity, &Interaction, Option<&Purple>), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction, maybe_purple) in query.iter() {
        if *interaction == Interaction::Clicked {
            // Adds or removes the Purple marker component when the entity is clicked
            match maybe_purple {
                // Adding components requires a concrete value for the new component
                None => commands.entity(entity).insert(Purple),
                // But removing them just requires the type
                Some(_) => commands.entity(entity).remove::<Purple>(),
            };
        }
    }
}

// This example is contrived for demonstration purposes:
// it would be much more efficient to simply set the material directly
// rather than using a marker component + system
fn enforce_purple(
    mut query: Query<(&mut Handle<ColorMaterial>, Option<&Purple>)>,
    white_material: Res<WhiteMaterial>,
    purple_material: Res<PurpleMaterial>,
) {
    for (mut material, maybe_purple) in query.iter_mut() {
        *material = match maybe_purple {
            None => white_material.0.clone(),
            Some(_) => purple_material.0.clone(),
        }
    }
}
```

Note that the equivalent commands exist for bundles, and work in exactly the same way (but are convenient for adding and removing more than one component at once).
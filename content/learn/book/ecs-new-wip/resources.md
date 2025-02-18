+++
title = "A blue triangle"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 3
status = 'hidden'
+++

```rust
use bevy::prelude::*;


#[derive(Bundle)]
struct Spike {
    pos: Transform,
    mesh: Mesh2d,
    texture: MeshMaterial2d<ColorMaterial>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, init_scene);

    app.run();
}

fn init_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let triangle_params = Triangle2d::new(
        Vec2::new(100.0, 0.0),
        Vec2::new(-100.00, 0.0),
        Vec2::new(0.0, 200.0));
    let mesh = Mesh2d(meshes.add(triangle_params));

    let colour = Color::srgb(0.0, 0.0,1.0);
    let colour_material = MeshMaterial2d(materials.add(colour));

    let position = Transform::IDENTITY;

    let triangle = Spike {
        mesh,
        pos: position,
        texture: colour_material,
    };

    commands.spawn(Camera2d);
    commands.spawn(triangle);
}
```

What is going on. We have spawned a blue triangle. But how does it work.
To render something simple, we need create a mesh (a set of triangles) and a colour map, also we need to specify where the triangle is located in the world space. This is done by declaring it to be at the origin of world space.

ResMut<Assets<Mesh>> are all meshes in our world. We register the mesh by calling [mesh.add()](https://docs.rs/bevy/latest/bevy/asset/struct.Assets.html#method.add) to obtain a mesh handle.
Similarly we call [mesh.add()](https://docs.rs/bevy/latest/bevy/asset/struct.Assets.html#method.add) on the ColorMaterial to register the colour map blue. Note that this can also be texture maps/images, but for now let us keep the example simple.

#### Moving the triangle

Aaah, that should be easy. Let us add a system with a query on the transform, then we can just mutate the transform to move the triangle, right? RIGHT?
```rust
use bevy::prelude::*;


#[derive(Bundle)]
struct Spike {
    pos: Transform,
    mesh: Mesh2d,
    texture: MeshMaterial2d<ColorMaterial>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, init_scene);
    app.add_systems(FixedUpdate, move_triangle);

    app.run();
}

fn init_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let triangle_params = Triangle2d::new(
        Vec2::new(100.0, 0.0),
        Vec2::new(-100.00, 0.0),
        Vec2::new(0.0, 200.0));
    let mesh = Mesh2d(meshes.add(triangle_params));

    let colour = Color::srgb(0.0, 0.0,1.0);
    let colour_material = MeshMaterial2d(materials.add(colour));

    let position = Transform::IDENTITY;

    let triangle = Spike {
        mesh,
        pos: position,
        texture: colour_material,
    };

    commands.spawn(Camera2d);
    commands.spawn(triangle);
}

fn move_triangle(mut query: Query<&mut Transform>) {
    for query_entry in query.iter_mut() {
        let transform = query_entry.into_inner();
        transform.translation.x += 5.0;
    }
}
```
And, the triangle doesn't move. Well actually it does, but so does the camera.

Remember when I said that the Query is very powerful. We can filter the camera out, or lets say, we can filter by something that has a component, let's say a Mesh?

```rust
fn move_triangle(mut query: Query<&mut Transform,With<Mesh2d>>) {
    for query_entry in query.iter_mut() {
        let transform = query_entry.into_inner();
        transform.translation.x += 5.0;
    }
}
```
Hooray, our triangle moves off screen.
Let us explain what has happened in more detail

##### Query filters

The beauty of the ECS is that it allows us to write our application very modular. We only have to state our dependencies and the framework will figure it out for us.

The [Query object](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Query.html) is a generic of Data and Filter, which means we can specify in detail what entities we want to consider and what our filtering criteria is. In our case we wanted everything that is positioned into space, but has a mesh. This excludes our camera conveniently.

However if we also introduce a square it will also move the square.

```rust
use bevy::prelude::*;


#[derive(Bundle)]
struct Spike {
    pos: Transform,
    mesh: Mesh2d,
    texture: MeshMaterial2d<ColorMaterial>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, init_scene);
    app.add_systems(FixedUpdate, move_triangle);

    app.run();
}

fn init_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let triangle_params = Triangle2d::new(
        Vec2::new(100.0, 0.0),
        Vec2::new(-100.00, 0.0),
        Vec2::new(0.0, 200.0));
    let triangle_mesh = Mesh2d(meshes.add(triangle_params));

    let colour = Color::srgb(0.0, 0.0,1.0);
    let colour_material = MeshMaterial2d(materials.add(colour));

    let position = Transform::IDENTITY;

    let triangle = Spike {
        mesh: triangle_mesh,
        pos: position,
        texture: colour_material.clone(),
    };

    let square_params = Rectangle::from_corners(Vec2::new(0.0, 0.0) ,Vec2::new(100.0,100.0));
    let square_mesh = Mesh2d(meshes.add(square_params));
    let square_position = Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0));
    commands.spawn(Camera2d);
    commands.spawn(triangle);
    commands.spawn((square_mesh,square_position,colour_material));
}

fn move_triangle(mut query: Query<&mut Transform,With<Mesh2d>>) {
    for query_entry in query.iter_mut() {
        let transform = query_entry.into_inner();
        transform.translation.x += 5.0;
    }
}
```
This now moves both the square and the triangle. We don't want that, we just want to move the triangle.
Let's add a triangle component to the triangle.
```rust
use bevy::prelude::*;


#[derive(Bundle,Default)]
struct Spike {
    pos: Transform,
    mesh: Mesh2d,
    texture: MeshMaterial2d<ColorMaterial>,
    marker: TriangleMarker,
}

#[derive(Component,Default)]
struct TriangleMarker {}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, init_scene);
    app.add_systems(FixedUpdate, move_triangle);

    app.run();
}

fn init_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let triangle_params = Triangle2d::new(
        Vec2::new(100.0, 0.0),
        Vec2::new(-100.00, 0.0),
        Vec2::new(0.0, 200.0));
    let triangle_mesh = Mesh2d(meshes.add(triangle_params));

    let colour = Color::srgb(0.0, 0.0,1.0);
    let colour_material = MeshMaterial2d(materials.add(colour));

    let position = Transform::IDENTITY;

    let triangle = Spike {
        mesh: triangle_mesh,
        pos: position,
        texture: colour_material.clone(),
        ..Default::default()
    };

    let square_params = Rectangle::from_corners(Vec2::new(0.0, 0.0) ,Vec2::new(100.0,100.0));
    let square_mesh = Mesh2d(meshes.add(square_params));
    let square_position = Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0));
    commands.spawn(Camera2d);
    commands.spawn(triangle);
    commands.spawn((square_mesh,square_position,colour_material));
}

fn move_triangle(mut query: Query<&mut Transform,With<TriangleMarker>>) {
    for query_entry in query.iter_mut() {
        let transform = query_entry.into_inner();
        transform.translation.x += 5.0;
    }
}
```
Congratulation, now only our triangle moves.


#### Schedules

We are using FixedUpdate as our Schedule, which as of now is not much different than the Update Schedule. Let us run our physics independent of our rendering at 250ms and let's also change the colour of the triangle to red. We'll see why soon.
```rust
use std::time::Duration;

use bevy::prelude::*;


#[derive(Bundle,Default)]
struct Spike {
    pos: Transform,
    mesh: Mesh2d,
    texture: MeshMaterial2d<ColorMaterial>,
    marker: TriangleMarker,
}

#[derive(Component,Default)]
struct TriangleMarker {}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(250)));
    app.add_systems(Startup, init_scene);
    app.add_systems(FixedUpdate, move_triangle);

    app.run();
}

fn init_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let triangle_params = Triangle2d::new(
        Vec2::new(100.0, 0.0),
        Vec2::new(-100.00, 0.0),
        Vec2::new(0.0, 200.0));
    let triangle_mesh = Mesh2d(meshes.add(triangle_params));

    let colour = Color::srgb(0.0, 0.0,1.0);
    let colour_material = MeshMaterial2d(materials.add(colour));

    let position = Transform::IDENTITY;

    let triangle = Spike {
        mesh: triangle_mesh,
        pos: position,
        texture: colour_material,
        ..Default::default()
    };

    let square_params = Rectangle::from_corners(Vec2::new(0.0, 0.0) ,Vec2::new(100.0,100.0));
    let square_mesh = Mesh2d(meshes.add(square_params));
    let square_position = Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0));

    let colour = Color::srgb(1.0, 0.0,0.0);
    let colour_material = MeshMaterial2d(materials.add(colour));
    commands.spawn(Camera2d);
    commands.spawn(triangle);
    commands.spawn((square_mesh,square_position,colour_material));
}

fn move_triangle(mut query: Query<&mut Transform,With<TriangleMarker>>) {
    for query_entry in query.iter_mut() {
        let transform = query_entry.into_inner();
        transform.translation.x += 5.0;
    }
}
```
If everything worked we should see that the triangle should move every 250 ms. This is because FixedUpdate is now running on a fixed frequency independent of the rendering.

While it is advisable to keep physics running on a different schedule we'll move it back to the rendering schedule to make use of the FixedUpdate.

We want to add a new system, we want to switch the colours of the triangle and the square every FixedUpdate cycle.

Also the triangle moves too fast, let's slow it down.

Let us see how we can accomplish that. You are encouraged to try it yourself first.

```rust
use std::time::Duration;

use bevy::prelude::*;


#[derive(Bundle,Default)]
struct Spike {
    pos: Transform,
    mesh: Mesh2d,
    texture: MeshMaterial2d<ColorMaterial>,
    marker: TriangleMarker,
}

#[derive(Component,Default)]
struct TriangleMarker {}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(250)));
    app.add_systems(Startup, init_scene);
    app.add_systems(Update, move_triangle);
    app.add_systems(FixedUpdate, switch_colours);

    app.run();
}

fn init_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let triangle_params = Triangle2d::new(
        Vec2::new(100.0, 0.0),
        Vec2::new(-100.00, 0.0),
        Vec2::new(0.0, 200.0));
    let triangle_mesh = Mesh2d(meshes.add(triangle_params));

    let colour = Color::srgb(0.0, 0.0,1.0);
    let colour_material = MeshMaterial2d(materials.add(colour));

    let position = Transform::IDENTITY;

    let triangle = Spike {
        mesh: triangle_mesh,
        pos: position,
        texture: colour_material,
        ..Default::default()
    };

    let square_params = Rectangle::from_corners(Vec2::new(0.0, 0.0) ,Vec2::new(100.0,100.0));
    let square_mesh = Mesh2d(meshes.add(square_params));
    let square_position = Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0));

    let colour = Color::srgb(1.0, 0.0,0.0);
    let colour_material = MeshMaterial2d(materials.add(colour));
    commands.spawn(Camera2d);
    commands.spawn(triangle);
    commands.spawn((square_mesh,square_position,colour_material));
}

fn move_triangle(mut query: Query<&mut Transform,With<TriangleMarker>>) {
    for query_entry in query.iter_mut() {
        let transform = query_entry.into_inner();
        transform.translation.x += 1.0;
    }
}

fn switch_colours(mut query: ResMut<Assets<ColorMaterial>>) {
    let mut iter = query.iter_mut();
    let first: (AssetId<ColorMaterial>, &mut ColorMaterial) = iter.next().unwrap();
    let second: (AssetId<ColorMaterial>, &mut ColorMaterial) = iter.next().unwrap();
    std::mem::swap(&mut first.1.color,&mut second.1.color);
}
```

#### Summary
We have learned how to use assets, colours and learned something about query filters.

+++
title = "Systems do work"
weight = 4
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

In order to make useful, fun or interesting games or apps, you'll need to manipulate the data that you store in components and resources.
In Bevy, virtually all of your logic will be stored in **systems**, functions that automatically receive data from the [`World`] according to their **system parameters**, and can mutate that data to change the world.
Any type which implements the [`SystemParam`] trait can be used as a function parameter in your system functions: this trait tells Bevy how to pass out access to the [`World`] in a safe and efficient way.

Most commonly, you'll be using:

- [`Query`], to access entity-component data
- [`Res`] and [`ResMut`], to access the global singleton data stored in resources
- [`Commands`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html), to queue up complex changes to the world like spawning entities
- [`EventWriter`](https://docs.rs/bevy/latest/bevy/app/struct.EventWriter.html) and [`EventReader`](https://docs.rs/bevy/latest/bevy/app/struct.EventReader.html), to work with events in an ergonomic fashion

You can see the full list by checking [which types implement `SystemParam`](https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html#implementors).
If you would like, you can even add your own custom system parameters by deriving the [`SystemParam`] trait on types whose fields all `impl SystemParam`.

Systems are added to your app using [`App::add_system`](https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.add_system) and related methods on [`App`], which will cause them to run once each time the game loop advances (commonly called a **tick**, which often but not always coincides with one rendered **frame**).

At the end of this page, there's a more complex example, demonstrating how you might structure systems for a real (but simple) game of Pong.

[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
[`SystemParam`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html
[`Query`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html
[`Res`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Res.html
[`ResMut`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.ResMut.html
[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`App::add_system`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.add_system

## Startup systems

In many cases, you don't *want* your systems to run constantly: instead, you may only want them to run a single time to perform some setup before your game begins.
**Startup system** run exactly once, before any ordinary systems.
You can add them using [`App::add_startup_system`].

Carefully controlling if and when systems run is one of the most important tools you have for managing the behavior of the game.
Check out the pages on [system ordering](../../game-logic/system-ordering/_index.md), [run criteria](../../game-logic/run-criteria/_index.md) and [states](../../game-logic/states/_index.md) in the next chapter for more details.

[`App::add_startup_system`](https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.add_startup_system)

## Generic systems

You are not limited to only *one* copy of each system: you can insert more copies in different places through your schedule when you want to perform the work repeatedly.
This fact becomes particularly useful when we combine it with [generic types](https://doc.rust-lang.org/book/ch10-01-syntax.html): creating **generic systems** whose behavior is specialized on individual types.

Generic systems are useful for repeating the same logic across many related types, and are incredibly value for library authors who wish to provide configurable APIs that mesh nicely with their users code.
In the latter case, note that entire [plugins] can be made generic in the same way.

All of the standard tricks for Rust's generics work when used in systems, allowing you to create systems with [trait bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax), multiple generic type parameters and even [const generics](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html) as type arguments.

[plugins]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html

## Pong example

FIXME: panics on missing `WindowDescriptor` resource.

```rust
use bevy::math::const_vec2;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use core::marker::PhantomData;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // We're inserting two different but related Score resources,
        // with their own distinct type parameter
        .init_resource::<Score<Player1>>()
        .init_resource::<Score<Player2>>()
        // These systems will run exactly once
        .add_startup_system(spawn_paddles)
        .add_startup_system(spawn_ball)
        .add_startup_system(spawn_walls)
        // These systems will run each frame
        .add_system(move_ball)
        .add_system(handle_collisions)
        .add_system(check_scored_points)
        // These generic systems allow us to specialize the logic of each system
        // based on the type parameter provided
        // Using generic systems here is *definitely* overengineering,
        // but it's a great way to teach them
        .add_system(move_paddle::<Player1>)
        .add_system(move_paddle::<Player2>)
        .run();
}

// Traits can be a great way to organize and share behavior
// The Component trait bound allows us to insert any type that implements this trait as a marker component
trait Player: Component + Default {
    // These associated constants are unique for each type that implements this trait
    const STARTING_POS: Vec2;
    const COLOR: Color;
    const MOVE_UP: KeyCode;
    const MOVE_DOWN: KeyCode;
}

#[derive(Component, Default)]
struct Player1;

impl Player for Player1 {
    const STARTING_POS: Vec2 = const_vec2!([-300., 0.]);
    const COLOR: Color = Color::RED;
    const MOVE_UP: KeyCode = KeyCode::W;
    const MOVE_DOWN: KeyCode = KeyCode::S;
}

#[derive(Component, Default)]
struct Player2;

impl Player for Player2 {
    const STARTING_POS: Vec2 = const_vec2!([-300., 0.]);
    const COLOR: Color = Color::BLUE;
    const MOVE_UP: KeyCode = KeyCode::Up;
    const MOVE_DOWN: KeyCode = KeyCode::Down;
}

// The trait bound on our type here means that any supplied
// type parameter P must implemnt the Player trait
#[derive(Default)]
struct Score<P: Player> {
    score: usize,
    // This field is a dummy value;
    // the PhantomData type is used to associate this struct with our type P
    _phantom: PhantomData<P>,
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Collidable;

#[derive(Bundle)]
struct PaddleBundle<P: Player> {
    paddle: Paddle,
    player_marker: P,
    #[bundle]
    sprite_bundle: SpriteBundle,
    collidable: Collidable,
}

impl<P: Player> PaddleBundle<P> {
    fn new() -> Self {
        const PADDLE_WIDTH: f32 = 10.;
        const PADDLE_HEIGHT: f32 = 40.;

        PaddleBundle {
            paddle: Paddle,
            // Because the Player trait requires the Default trait,
            // we can use its methods here
            player_marker: P::default(),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // The associated constants of type P can be used here,
                    // because we specified a Player trait bound and are thus guaranteed to have access
                    // to the methods and constants of the Player trait
                    translation: Vec3::new(P::STARTING_POS.x, P::STARTING_POS.y, 0.),
                    scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.),
                    ..default()
                },
                sprite: Sprite {
                    color: P::COLOR,
                    ..default()
                },
                ..default()
            },
            collidable: Collidable,
        }
    }
}

fn spawn_paddles(mut commands: Commands) {
    // Behold the power of traits!
    commands.spawn_bundle(PaddleBundle::<Player1>::new());
    commands.spawn_bundle(PaddleBundle::<Player2>::new());
}

#[derive(Component)]
struct Ball;

fn spawn_ball(mut commands: Commands) {
    const STARTING_VELOCITY: Velocity = Velocity { x: 40., y: -50. };
    const BALL_SIZE: f32 = 10.;

    commands
        .spawn()
        .insert(STARTING_VELOCITY)
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(BALL_SIZE, BALL_SIZE, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::YELLOW,
                ..default()
            },
            ..default()
        });
}

// Resources are inserted before startup systems run
// and so we can access the WindowDescriptor resource here
fn spawn_walls(mut commands: Commands, window_descriptor: Res<WindowDescriptor>) {
    // In Bevy, the origin of our coordinates is in the center of our screen by default
    let top = window_descriptor.height / 2.0;
    let bottom = -window_descriptor.height / 2.0;

    // Spawn top wall
    commands
        .spawn()
        .insert(Collidable)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., top, 0.),
                scale: Vec3::new(window_descriptor.width, 5.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            ..default()
        });

    // Spawn bottom wall
    commands
        .spawn()
        .insert(Collidable)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., bottom, 0.),
                scale: Vec3::new(window_descriptor.width, 5.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            ..default()
        });
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn move_ball(mut query: Query<(&mut Transform, &Velocity), With<Ball>>) {
    let (mut ball_transform, ball_velocity) = query.single_mut();

    ball_transform.translation.x += ball_velocity.x;
    ball_transform.translation.y += ball_velocity.y;
}

fn handle_collisions(
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    collidable_query: Query<&Transform, With<Collidable>>,
) {
    let (ball_transform, mut ball_velocity) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for collidable_transform in collidable_query.iter() {
        // Check for collisions with each paddle
        let maybe_collision = collide(
            ball_transform.translation,
            ball_size,
            collidable_transform.translation,
            collidable_transform.scale.truncate(),
        );

        // If a collision occured, handle it
        if let Some(collision) = maybe_collision {
            // Reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // Only reflect if the ball's velocity is going in
            // the opposite direction of the collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // Reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // Reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

fn check_scored_points(
    window_descriptor: Res<WindowDescriptor>,
    mut ball_query: Query<&mut Transform, With<Ball>>,
    mut player_1_score: ResMut<Score<Player1>>,
    mut player_2_score: ResMut<Score<Player2>>,
) {
    let left_bound = -window_descriptor.width / 2.;
    let right_bound = window_descriptor.width / 2.;
    let mut ball_translation = ball_query.single_mut().translation;

    // Increase the score and reset the ball when it goes past the paddles
    if ball_translation.x <= left_bound {
        player_2_score.score += 1;
        ball_translation.x = 0.;
    } else if ball_translation.x >= right_bound {
        player_1_score.score += 1;
        ball_translation.x = 0.;
    }
}

// Because P is a Player (which requires the Component trait), we can use it as a marker component
// and filter our queries using it
fn move_paddle<P: Player>(
    mut query: Query<&mut Transform, (With<Paddle>, With<P>)>,
    input: Res<Input<KeyCode>>,
) {
    const PADDLE_SPEED: f32 = 20.;
    let mut paddle_translation = query.single_mut().translation;

    if input.just_pressed(P::MOVE_UP) {
        paddle_translation.y += PADDLE_SPEED;
    }

    if input.just_pressed(P::MOVE_DOWN) {
        paddle_translation.y -= PADDLE_SPEED;
    }
}
```

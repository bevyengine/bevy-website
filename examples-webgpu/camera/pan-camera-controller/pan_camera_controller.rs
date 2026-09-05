use bevy::camera_controller::pan_camera::{PanCamera, PanCameraPlugin};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanCameraPlugin) // Adds the PanCamera plugin to enable camera panning and zooming controls.
        .add_systems(Startup, (setup, spawn_text).chain())
        .run();
}

fn spawn_text(mut commands: Commands, camera: Query<&PanCamera>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(-16),
            left: px(12),
            ..default()
        },
        children![Text::new(format!("{}", camera.single().unwrap()))],
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D Camera with default PanCamera settings
    commands.spawn((Camera2d, PanCamera::default()));

    commands.spawn(Sprite::from_image(
        asset_server.load("branding/bevy_bird_dark.png"),
    ));
}

use bevy::{asset::io::web::WebAssetPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WebAssetPlugin {
            silence_startup_warning: true,
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let url = "https://raw.githubusercontent.com/bevyengine/bevy/refs/heads/main/assets/branding/bevy_bird_dark.png";
    // Simply use a url where you would normally use an asset folder relative path
    commands.spawn(Sprite::from_image(asset_server.load(url)));
}

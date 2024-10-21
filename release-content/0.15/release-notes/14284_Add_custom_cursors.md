<!-- Add custom cursors -->
<!-- https://github.com/bevyengine/bevy/pull/14284 -->

Previously Bevy's native window cursors supported only a fixed set of built-in OS cursors. Now they also support arbitrary images as "custom cursors". Custom cursors still use native facilities of the OS, which allows them to stay perfectly responsive even when the frame rate of the application drops.

```rust
use bevy::prelude::*;
use bevy::winit::cursor::{CursorIcon, CustomCursor};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, update)
        .run();
}

#[derive(Resource)]
struct MyCursorIcon(CursorIcon);

// It's recommended to load cursor images at startup.
// Otherwise changing the cursor could be delayed by a frame or two.
fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MyCursorIcon(CursorIcon::Custom(CustomCursor::Image {
        handle: asset_server.load("cursor_icon.png"),
        hotspot: (5, 5),
    })));
}

// Insert the cursor component to the window. Usually you would insert a different
// image depending on the context (click, hover, etc.), but here we have only one.
// Using a series of images can achieve an animated look.
fn update(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    my_cursor_icon: Res<MyCursorIcon>,
) {
    commands.entity(*window).insert(my_cursor_icon.0.clone());
}
```

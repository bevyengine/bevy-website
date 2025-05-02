The `bevy::ui::render::extract_default_ui_camera_view()` system is now hard-wired to both the `Camera2d` and `Camera3d` components, and is no longer added twice for each type.

This change was made to fix a bug introduced after moving render phases to resources. The first thing this system does is clear out all entities from the previous frame. By having two separate systems, one was always clearing out the other, causing some entities to not be rendered.

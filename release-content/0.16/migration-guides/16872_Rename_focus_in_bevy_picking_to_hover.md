Various terms related to “focus” in `bevy_picking` have been renamed to refer to “hover” to avoid confusion with `bevy_input_focus`. In particular:

- The `update_focus` system has been renamed to `generate_hovermap`
- `PickSet::Focus` and `PostFocus` have been renamed to `Hover` and `PostHover`
- The `bevy_picking::focus` module has been renamed to `bevy_picking::hover`
- The `is_focus_enabled` field on `PickingPlugin` has been renamed to `is_hover_enabled`
- The `focus_should_run` run condition has been renamed to `hover_should_run`

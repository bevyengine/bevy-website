Since bevy_state is now gated behind the `bevy_state` feature, projects that use state but don’t use the `default-features` will need to add that feature flag.

Since it is no longer part of bevy_ecs, projects that use bevy_ecs directly will need to manually pull in `bevy_state`,  trigger the StateTransition schedule, and handle any of the elements that bevy_app currently sets up.

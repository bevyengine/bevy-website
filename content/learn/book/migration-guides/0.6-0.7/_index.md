+++
title = "0.6 to 0.7"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.6 to 0.7"
+++

### AliasedMutability

<https://github.com/bevyengine/bevy/pull/4298>

The QueryEntityError enum now has a `AliasedMutability` variant, and returns the offending entity

### Remove Margins

<https://github.com/bevyengine/bevy/pull/4284>

The Margins type got removed. To migrate you just have to change every occurrence of Margins to UiRect.

### Remove face_toward.rs

<https://github.com/bevyengine/bevy/pull/4277>

The FaceToward trait got removed. To migrate you just have to change every occurrence of Mat4::face_toward to Mat4::look_at_rh.

### unsafeify World::entities_mut

<https://github.com/bevyengine/bevy/pull/4093>

```rs
// 0.6
world.entities_mut()

// 0.7
unsafe { world.entities_mut() }
```

### Mesh vertex buffer layouts

<https://github.com/bevyengine/bevy/pull/3959>

TODO

### Remove the need for '.system' when using run criteria piping

<https://github.com/bevyengine/bevy/pull/3923>

```rs
// 0.6
.with_run_criteria(RunCriteria::pipe(
    "is_done_label",
    IntoSystem::into_system(inverse),
))

// 0.7
.with_run_criteria(RunCriteria::pipe("is_done_label", inverse))
```

### Obviate the need for RunSystem, and remove it

<https://github.com/bevyengine/bevy/pull/3817>

TODO

### Replace VSync with PresentMode

https://github.com/bevyengine/bevy/pull/3812

TODO link to PresentMode enum

```rs
// 0.6
App::new()
    .insert_resource(WindowDescriptor {
        vsync: false,
        ..Default::default()
    })

// 0.7
App::new()
    .insert_resource(WindowDescriptor {
        vsync: false,
        present_mode: PresentMode::Mailbox,
        ..Default::default()
    })
```

### Fix mul_vec3 tranformation order

<https://github.com/bevyengine/bevy/pull/3811>

Fixes order of transformations to scale -> rotate -> translate. This doesn't require any code changes, but it means SpriteBundle will behave as expected when rotating.

### Use marker components for cameras instead of name strings

<https://github.com/bevyengine/bevy/pull/3635>

TODO

### Remove the config api

<https://github.com/bevyengine/bevy/pull/3633>

TODO

### Add capability to render to a texture

<https://github.com/bevyengine/bevy/pull/3412>

```rs
// 0.6
commands.spawn_bundle(PerspectiveCameraBundle {
    camera: Camera {
        window: window_id,
        ..Default::default()
    },
    ..Default::default()
});

// 0.7
commands.spawn_bundle(PerspectiveCameraBundle {
    camera: Camera {
        target: RenderTarget::Window(window_id),
        ..Default::default()
    },
    ..Default::default()
});
```

### Implement init_resource for Commands and World

<https://github.com/bevyengine/bevy/pull/3079>

```rs
#[derive(Default)]
struct Scoreboard {
    current_score: u32,
    high_score: u32,
}

// 0.6
commands.insert_resource(Scoreboard::Default());

// 0.7
commands.init_resource::<Scoreboard>();
```

### ParamSet for conflicting SystemParam

<https://github.com/bevyengine/bevy/pull/2765>

TODO
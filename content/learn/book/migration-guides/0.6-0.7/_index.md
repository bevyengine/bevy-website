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

<!-- Github filter used to find the relevant PRs "is:pr label:C-Breaking-Change closed:>2022-02-01 [Merged by Bors]" -->

### AliasedMutability

<https://github.com/bevyengine/bevy/pull/4298>

The `QueryEntityError` enum now has a `AliasedMutability` variant, and returns the offending entity.

### Remove margins.rs

<https://github.com/bevyengine/bevy/pull/4284>

The `Margins` type was removed. To migrate, replace of `Margins` with `UiRect`.

### Remove face_toward.rs

<https://github.com/bevyengine/bevy/pull/4277>

The `FaceToward` trait was removed. To migrate, replace every occurrence of `Mat4::face_toward` to `Mat4::look_at_rh`.

### `World::entities_mut` is now unsafe

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

### Remove the need for 'IntoSystem::into_system()' when using run criteria piping

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

Instead of using a boolean flag for vsync we switched to using a {{rust_type(type="struct" crate="bevy" mod="window" version="0.7.0" name="PresentMode" no_mod=true)}} enum with multiple variants.

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
        present_mode: PresentMode::Mailbox,
        ..Default::default()
    })
```

### Fix mul_vec3 tranformation order

<https://github.com/bevyengine/bevy/pull/3811>

Transforms are now consistently applied in the standard scale -> rotate -> translate. This doesn't require any code changes, but it means SpriteBundle will behave as expected when rotating.

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

### Infallabile resource getters

<https://github.com/bevyengine/bevy/pull/4047>

```rs
// 0.6
let score = world.get_resource::<Score>().unwrap();

// 0.7
let score = world.resource::<Score>();
```
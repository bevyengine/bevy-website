+++
title = "0.6 to 0.7"
weight = 3
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.6 to 0.7"
+++

<!-- Github filter used to find the relevant PRs "is:pr label:C-Breaking-Change closed:>2022-02-01 [Merged by Bors]" -->

### [ParamSet for conflicting SystemParam](https://github.com/bevyengine/bevy/pull/2765)

```rs
// 0.6
fn system(
    mut transforms: QuerySet<(
        QueryState<&mut Transform, With<Marker>>,
        QueryState<&Transform>,
    )>,
) {
    for transform in transforms.q1().iter() {
        // ...
    }
}
// 0.7
fn system(
    mut transforms: ParamSet<(
        Query<&mut Transform, With<Marker>>,
        Query<&Transform>
    )>,
) {
    for transform in transforms.p1().iter() {
        // ...
    }
}
```

### [AliasedMutability](https://github.com/bevyengine/bevy/pull/4298)

The `QueryEntityError` enum now has a `AliasedMutability` variant, and returns the offending entity.

### [Remove margins.rs](https://github.com/bevyengine/bevy/pull/4284)

The `Margins` type was removed. To migrate, replace every occurrence of `Margins` with `Rect`.

### [Remove face_toward.rs](https://github.com/bevyengine/bevy/pull/4277https://github.com/bevyengine/bevy/pull/4277)

The `FaceToward` trait was removed. To migrate, replace every occurrence of `Mat4::face_toward` to `Mat4::look_at_rh`.

### [`World::entities_mut` is now unsafe](https://github.com/bevyengine/bevy/pull/4093)

```rs
// 0.6
world.entities_mut()

// 0.7
unsafe { world.entities_mut() }
```

### [Custom vertex attributes](https://github.com/bevyengine/bevy/pull/3959)

Custom vertex attributes are now referenced by a `MeshVertexAttribute` rather than a simple string and `set_attribute` has been renamed to `insert_attribute` better reflect its behavior.

```rs
// 0.6
mesh.set_attribute("Vertex_Custom", VertexAttributeValues::Sint32x4(vec![]));

// 0.7
// Generate your own random identifier here.
// https://play.rust-lang.org/?gist=cc7e824724ba023e9bff25db35ef1f5e
pub const ATTRIBUTE_CUSTOM: MeshVertexAttribute =
    MeshVertexAttribute::new("Custom", 17351772347970238659, VertexFormat::Sint32x4);
mesh.insert_attribute(
    ATTRIBUTE_CUSTOM,
    VertexAttributeValues::Sint32x4(vec![]),
);
```

### [Mesh vertex buffer layouts](https://github.com/bevyengine/bevy/pull/3959)

Vertex buffers no longer need to be manually laid out with offset and stride values in a `RenderPipelineDescriptor`.

```rs
// 0.6
let vertex_buffer_layout = VertexBufferLayout {
    array_stride: 20,
    step_mode: VertexStepMode::Vertex,
    attributes: vec![
        VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        },
        VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: 12,
            shader_location: 1,
        },
    ],
};

// 0.7
let mut formats = vec![
    VertexFormat::Float32x3,
    VertexFormat::Float32x2,
];
let vertex_layout = VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);
```

### [Remove the need for 'IntoSystem::into_system()' when using run criteria piping](https://github.com/bevyengine/bevy/pull/3923)

```rs
// 0.6
.with_run_criteria(RunCriteria::pipe(
    "is_done_label",
    IntoSystem::into_system(inverse),
))

// 0.7
.with_run_criteria(RunCriteria::pipe("is_done_label", inverse))
```

### [Remove RunSystem](https://github.com/bevyengine/bevy/pull/3817)

You probably should not have been using [`RunSystem`] or [`ParamSystem`], but if you were and you really need it, please make sure to let us know by creating a new discussion over at <https://github.com/bevyengine/bevy/discussions>

<!-- TODO add links to RunSystem and ParamSystem maybe? -->

### [Replace VSync with PresentMode](https://github.com/bevyengine/bevy/pull/3812)

Instead of using a boolean flag for vsync we switched to using a [`PresentMode`] enum with multiple variants.

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
        present_mode: PresentMode::Immediate,
        ..Default::default()
    })
```

<!-- TODO make sure this works after release -->
[`PresentMode`]: https://docs.rs/bevy/0.7/bevy/window/enum.PresentMode.html

### [Fix mul_vec3 tranformation order](https://github.com/bevyengine/bevy/pull/3811)

Transforms are now consistently applied in the standard scale -> rotate -> translate. This doesn't require any code changes unless you had something to handle the wrong behaviour, but it means SpriteBundle will now behave as expected when rotating.

### [Use marker components for cameras instead of name strings](https://github.com/bevyengine/bevy/pull/3635)

```rs
// 0.6
pub const FIRST_PASS_CAMERA: &str = "first_pass_camera";
fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        camera: Camera {
            name: Some(FIRST_PASS_CAMERA.to_string()),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });
}

fn camera_system(cameras: Query<&Camera>) {
    for camera in cameras.iter() {
        if camera.name == Some(FIRST_PASS_CAMERA.to_string()) {
            // Do something with a camera
        }
    }
}

// 0.7
#[derive(Component, Default)]
pub struct FirstPassCamera;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle::<FirstPassCamera> {
        camera: Camera::default(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..PerspectiveCameraBundle::new()
    });
}

fn camera_system(cameras: Query<&Camera, With<FirstPassCamera>>) {
    for camera in cameras.iter() {
        // Do something with camera
    }
}
```

### [Remove the config api](https://github.com/bevyengine/bevy/pull/3633)

```rs
// 0.6
struct Config(u32);

fn local_config(local: Local<Config>) {
    assert_eq!(*local.0, 42);
}

fn main() {
        App::new()
        .add_system(local_is_42.config(|params| params.0 = Some(Config(42))))
        .run();
}

// 0.7
struct Config(u32);

fn local_config(local: u32) -> impl FnMut(ResMut<Config>) {
    move |mut val| {
        val.0 = local;

        assert_eq!(val.0, 42);
    }
}

fn main() {
        App::new()
        .add_system(local_config(Config(42)))
        .run();
}
```

### [Cameras now point at RenderTarget rather than Window](https://github.com/bevyengine/bevy/pull/3412)

This change was made to support rendering to textures. Users working with multiple windows may be affected.

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

### [Implement init_resource for Commands and World](https://github.com/bevyengine/bevy/pull/3079)

```rs
#[derive(Default)]
struct Scoreboard {
    current_score: u32,
    high_score: u32,
}

// 0.6
commands.insert_resource(Scoreboard::default());

// 0.7
commands.init_resource::<Scoreboard>();
```

### [Infallabile resource getters](https://github.com/bevyengine/bevy/pull/4047)

```rs
// 0.6
let score = world.get_resource::<Score>().unwrap();

// 0.7
let score = world.resource::<Score>();
```

### [Event handling types are no longer re-exported from bevy_app](https://github.com/bevyengine/bevy/pull/4066)

This only affects users who were importing these types directly from `bevy_app` and not through bevy's prelude.

```rs
// 0.6
use bevy::app::{EventId, EventReader, EventWriter, Events, ManualEventReader};

// 0.7
use bevy::ecs::event::{EventId, EventReader, EventWriter, Events, ManualEventReader};
```

use std::f32::consts::FRAC_PI_2;

use bevy::{color::palettes::tailwind, input::mouse::AccumulatedMouseMotion, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<DidFixedTimestepRunThisFrame>()
        .add_systems(Startup, (spawn_text, spawn_player, spawn_environment))
        // At the beginning of each frame, clear the flag that indicates whether the fixed timestep has run this frame.
        .add_systems(PreUpdate, clear_fixed_timestep_flag)
        // At the beginning of each fixed timestep, set the flag that indicates whether the fixed timestep has run this frame.
        .add_systems(FixedPreUpdate, set_fixed_time_step_flag)
        // Advance the physics simulation using a fixed timestep.
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(
            // The `RunFixedMainLoop` schedule allows us to schedule systems to run before and after the fixed timestep loop.
            RunFixedMainLoop,
            (
                (
                    // The camera needs to be rotated before the physics simulation is advanced in before the fixed timestep loop,
                    // so that the physics simulation can use the current rotation.
                    // Note that if we ran it in `Update`, it would be too late, as the physics simulation would already have been advanced.
                    // If we ran this in `FixedUpdate`, it would sometimes not register player input, as that schedule may run zero times per frame.
                    rotate_camera,
                    // Accumulate our input before the fixed timestep loop to tell the physics simulation what it should do during the fixed timestep.
                    accumulate_input,
                )
                    .chain()
                    .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                (
                    // Clear our accumulated input after it was processed during the fixed timestep.
                    // By clearing the input *after* the fixed timestep, we can still use `AccumulatedInput` inside `FixedUpdate` if we need it.
                    clear_input.run_if(did_fixed_timestep_run_this_frame),
                    // The player's visual representation needs to be updated after the physics simulation has been advanced.
                    // This could be run in `Update`, but if we run it here instead, the systems in `Update`
                    // will be working with the `Transform` that will actually be shown on screen.
                    interpolate_rendered_transform,
                    // The camera can then use the interpolated transform to position itself correctly.
                    translate_camera,
                )
                    .chain()
                    .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
            ),
        )
        .run();
}

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct AccumulatedInput {
    // The player's movement input (WASD).
    movement: Vec2,
    // Other input that could make sense would be e.g.
    // boost: bool
}

/// A vector representing the player's velocity in the physics simulation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct Velocity(Vec3);

/// The actual position of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual representation.
///
/// If you want to make sure that this component is always initialized
/// with the same value as the `Transform`'s translation, you can
/// use a [component lifecycle hook](https://docs.rs/bevy/0.14.0/bevy/ecs/component/struct.ComponentHooks.html)
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PhysicalTranslation(Vec3);

/// The value [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

/// Spawn the player and a 3D camera. We could also spawn the camera as a child of the player,
/// but in practice, they are usually spawned separately so that the player's rotation does not
/// influence the camera's rotation.
fn spawn_player(mut commands: Commands) {
    commands.spawn((Camera3d::default(), CameraSensitivity::default()));
    commands.spawn((
        Name::new("Player"),
        Transform::from_scale(Vec3::splat(0.3)),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
    ));
}

/// Spawn a field of floating spheres to fly around in
fn spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_material = materials.add(Color::from(tailwind::SKY_200));
    let sphere_mesh = meshes.add(Sphere::new(0.3));
    let spheres_in_x = 6;
    let spheres_in_y = 4;
    let spheres_in_z = 10;
    let distance = 3.0;
    for x in 0..spheres_in_x {
        for y in 0..spheres_in_y {
            for z in 0..spheres_in_z {
                let translation = Vec3::new(
                    x as f32 * distance - (spheres_in_x as f32 - 1.0) * distance / 2.0,
                    y as f32 * distance - (spheres_in_y as f32 - 1.0) * distance / 2.0,
                    z as f32 * distance - (spheres_in_z as f32 - 1.0) * distance / 2.0,
                );
                commands.spawn((
                    Name::new("Sphere"),
                    Transform::from_translation(translation),
                    Mesh3d(sphere_mesh.clone()),
                    MeshMaterial3d(sphere_material.clone()),
                ));
            }
        }
    }

    commands.spawn((
        DirectionalLight::default(),
        Transform::default().looking_to(Vec3::new(-1.0, -3.0, 0.5), Vec3::Y),
    ));
}

/// Spawn a bit of UI text to explain how to move the player.
fn spawn_text(mut commands: Commands) {
    let font = TextFont {
        font_size: 25.0,
        ..default()
    };
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (Text::new("Move the player with WASD"), font.clone()),
            (Text::new("Rotate the camera with the mouse"), font)
        ],
    ));
}

fn rotate_camera(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Camera>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta time, we will get a smaller rotation than intended by the user.
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            // We use a component for them so that we can make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect` the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

/// Handle keyboard input and accumulate it in the `AccumulatedInput` component.
///
/// There are many strategies for how to handle all the input that happened since the last fixed timestep.
/// This is a very simple one: we just use the last available input.
/// That strategy works fine for us since the user continuously presses the input keys in this example.
/// If we had some kind of instantaneous action like activating a boost ability, we would need to remember that that input
/// was pressed at some point since the last fixed timestep.
fn accumulate_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut AccumulatedInput, &mut Velocity)>,
    camera: Single<&Transform, With<Camera>>,
) {
    /// Since Bevy's 3D renderer assumes SI units, this has the unit of meters per second.
    /// Note that about 1.5 is the average walking speed of a human.
    const SPEED: f32 = 4.0;
    let (mut input, mut velocity) = player.into_inner();
    // Reset the input to zero before reading the new input. As mentioned above, we can only do this
    // because this is continuously pressed by the user. Do not reset e.g. whether the user wants to boost.
    input.movement = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        input.movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input.movement.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input.movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input.movement.x += 1.0;
    }

    // Remap the 2D input to Bevy's 3D coordinate system.
    // Pressing W makes `input.y` go up. Since Bevy assumes that -Z is forward, we make our new Z equal to -input.y
    let input_3d = Vec3 {
        x: input.movement.x,
        y: 0.0,
        z: -input.movement.y,
    };

    // Rotate the input so that forward is aligned with the camera's forward direction.
    let rotated_input = camera.rotation * input_3d;

    // We need to normalize and scale because otherwise
    // diagonal movement would be faster than horizontal or vertical movement.
    // We use `clamp_length_max` instead of `.normalize_or_zero()` because gamepad input
    // may be smaller than 1.0 when the player is pushing the stick just a little bit.
    velocity.0 = rotated_input.clamp_length_max(1.0) * SPEED;
}

/// A simple resource that tells us whether the fixed timestep ran this frame.
#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct DidFixedTimestepRunThisFrame(bool);

/// Reset the flag at the start of every frame.
fn clear_fixed_timestep_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = false;
}

/// Set the flag during each fixed timestep.
fn set_fixed_time_step_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = true;
}

fn did_fixed_timestep_run_this_frame(
    did_fixed_timestep_run_this_frame: Res<DidFixedTimestepRunThisFrame>,
) -> bool {
    did_fixed_timestep_run_this_frame.0
}

// Clear the input after it was processed in the fixed timestep.
fn clear_input(mut input: Single<&mut AccumulatedInput>) {
    **input = AccumulatedInput::default();
}

/// Advance the physics simulation by one fixed timestep. This may run zero or multiple times per frame.
///
/// Note that since this runs in `FixedUpdate`, `Res<Time>` would be `Res<Time<Fixed>>` automatically.
/// We are being explicit here for clarity.
fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &Velocity,
    )>,
) {
    for (mut current_physical_translation, mut previous_physical_translation, velocity) in
        query.iter_mut()
    {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();
    }
}

fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in
        query.iter_mut()
    {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;
        // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation;
    }
}

// Sync the camera's position with the player's interpolated position
fn translate_camera(
    mut camera: Single<&mut Transform, With<Camera>>,
    player: Single<&Transform, (With<AccumulatedInput>, Without<Camera>)>,
) {
    camera.translation = player.translation;
}

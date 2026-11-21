use bevy::{
    mesh::MeshVertexBufferLayoutRef,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dKey, Material2dPipeline, Material2dPlugin},
};

const SHADER_ASSET_PATH: &str = "shaders/pipeline_constants.wesl";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<PosterizeMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PosterizeMaterial>>,
) {
    commands.spawn(Camera2d);

    let quad = meshes.add(Rectangle::new(200.0, 150.0));

    for (i, levels) in [2u32, 4, 8].into_iter().enumerate() {
        let x = (i as f32 - 1.0) * 220.0;

        commands.spawn((
            Mesh2d(quad.clone()),
            MeshMaterial2d(materials.add(PosterizeMaterial { levels })),
            Transform::from_xyz(x, 20.0, 0.0),
        ));

        commands.spawn((
            Text2d::new(format!("{levels} levels")),
            Transform::from_xyz(x, -65.0, 0.0),
        ));
    }
}

/// A material that posterizes a color gradient using a pipeline-overridable constant.
///
/// Each distinct `levels` value produces a separate compiled pipeline variant.
#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[bind_group_data(PosterizeMaterialKey)]
struct PosterizeMaterial {
    /// Number of discrete color steps. Compiled into the shader as a pipeline constant.
    levels: u32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct PosterizeMaterialKey {
    levels: u32,
}

impl From<&PosterizeMaterial> for PosterizeMaterialKey {
    fn from(m: &PosterizeMaterial) -> Self {
        Self { levels: m.levels }
    }
}

impl Material2d for PosterizeMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        _pipeline: &Material2dPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor
            .fragment
            .as_mut()
            .unwrap()
            .constants
            .push(("LEVELS".into(), key.bind_group_data.levels as f64));
        Ok(())
    }
}

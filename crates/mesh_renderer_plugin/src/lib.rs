use bevy::ecs::bundle::Bundle;
use bevy::prelude::{AddAsset, GlobalTransform, Handle, Plugin, Transform};

pub use bevy_pbr::prelude::StandardMaterial;
pub use bevy_render::{
    color::Color,
    mesh::{self, Mesh},
    pipeline::PrimitiveTopology,
    texture,
};

use bevy_rafx_plugin::VisibilityComponent;

#[derive(Bundle, Default)]
pub struct PbrBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    // pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Default)]
pub struct MeshRendererPlugin {}

impl Plugin for MeshRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_asset::<Mesh>().add_asset::<StandardMaterial>();
    }
}

struct MeshBundle {
    visibility_component: VisibilityComponent,
}

impl Default for MeshBundle {
    fn default() -> Self {
        Self {
            visibility_component: VisibilityComponent::default(),
        }
    }
}

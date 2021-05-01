use bevy::prelude::Plugin;
use bevy_rafx_plugin::VisibilityComponent;

pub mod assets;
pub mod mesh;

#[derive(Default)]
pub struct MeshRendererPlugin {}

impl Plugin for MeshRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {}
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

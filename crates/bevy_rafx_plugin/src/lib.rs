use bevy::{
    log,
    prelude::{CoreStage, IntoSystem, Plugin, StageLabel, SystemStage},
};
use rafx::{
    nodes::FramePacketBuilder,
    visibility::{VisibilityObjectArc, VisibilityRegion},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderStage {
    Visibility,
    Extract,
    Prepare,
    Submit,
}

#[derive(Default)]
pub struct BevyRafxPlugin;

impl Plugin for BevyRafxPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(RenderPlugins::default())
            .insert_resource(VisibilityRegion::new())
            .add_stage_after(
                CoreStage::PostUpdate,
                RenderStage::Visibility,
                SystemStage::parallel(),
            )
            .add_stage_after(
                RenderStage::Visibility,
                RenderStage::Extract,
                SystemStage::parallel(),
            )
            .add_stage_after(
                RenderStage::Extract,
                RenderStage::Prepare,
                SystemStage::parallel(),
            )
            .add_stage_after(
                RenderStage::Prepare,
                RenderStage::Submit,
                SystemStage::parallel(),
            )
            // TODO run first in stage
            .add_system_to_stage(RenderStage::Visibility, update_visibility.system())
            .add_system_to_stage(RenderStage::Extract, create_frame_packet.system());
    }
}

fn update_visibility(// mut visibility_region: ResMut<VisibilityRegion>,
    // query: Query<(Entity, &GlobalTransform, &mut VisibilityComponent)>,
) {
}

fn create_frame_packet() {
    let _frame_packet_builder = FramePacketBuilder::new();
}
#[derive(Default)]
pub struct VisibilityComponent {
    handle: Option<VisibilityObjectArc>,
}

pub trait RenderPlugin: Send + Sync + 'static {}

pub type RenderPlugins = Vec<Box<dyn RenderPlugin>>;

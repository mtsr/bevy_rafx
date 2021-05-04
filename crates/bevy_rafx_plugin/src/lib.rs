use std::fmt::Debug;

use bevy::{
    ecs::reflect::ReflectComponent,
    prelude::{
        CoreStage, GlobalTransform, IntoSystem, Plugin, Query, Res, ResMut, StageLabel,
        StartupStage, SystemStage, Transform,
    },
    reflect::Reflect,
    window::Windows,
};
pub use bevy_render::{
    camera::{Camera, CameraProjection, OrthographicProjection, PerspectiveProjection},
    entity::PerspectiveCameraBundle,
};
use rafx::{
    nodes::{
        FramePacketBuilder, RenderFeatureMaskBuilder, RenderPhaseMask, RenderPhaseMaskBuilder,
        RenderRegistry, RenderRegistryBuilder, RenderViewDepthRange, RenderViewSet,
    },
    visibility::{VisibilityObjectArc, VisibilityRegion},
};

pub mod phases;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderStage {
    Visibility,
    PreExtract,
    Extract,
    Prepare,
    Submit,
}

#[derive(Default)]
pub struct BevyRafxPlugin;

impl Plugin for BevyRafxPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource::<Option<RenderRegistryBuilder>>(
            Some(RenderRegistryBuilder::default()),
        )
        .insert_resource::<Option<RenderRegistry>>(None)
        .insert_resource(Some(RenderPhaseMaskBuilder::default()))
        .insert_resource(Some(RenderFeatureMaskBuilder::default()))
        .insert_resource(RenderPlugins::default())
        .insert_resource(VisibilityRegion::new())
        .add_stage_after(
            CoreStage::PostUpdate,
            RenderStage::Visibility,
            SystemStage::parallel(),
        )
        .add_stage_after(
            RenderStage::Visibility,
            RenderStage::PreExtract,
            SystemStage::parallel(),
        )
        .add_stage_after(
            RenderStage::PreExtract,
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
        .add_startup_system_to_stage(StartupStage::PostStartup, build_render_registry.system())
        .add_system_to_stage(RenderStage::Visibility, update_visibility.system())
            .add_system_to_stage(RenderStage::Extract, create_frame_packet.system());
    }
}

fn build_render_registry(
    mut render_registry_builder: ResMut<Option<RenderRegistryBuilder>>,
    mut render_registry: ResMut<Option<RenderRegistry>>,
) {
    let render_registry_builder =
        render_registry_builder
            .take()
            .unwrap()
            .register_render_phase::<phases::opaque_render_phase::OpaqueRenderPhase>("Opaque");
    render_registry.replace(render_registry_builder.build());
}

fn update_visibility(// mut visibility_region: ResMut<VisibilityRegion>,
    // query: Query<(Entity, &GlobalTransform, &mut VisibilityComponent)>,
) {
}

fn create_frame_packet() {
    let _frame_packet_builder = FramePacketBuilder::new();
}
#[derive(Clone, Default, Reflect)]
#[reflect(Component)]
pub struct VisibilityComponent {
    #[reflect(ignore)]
    pub handle: Option<VisibilityObjectArc>,
}

impl Debug for VisibilityComponent {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.debug_struct("VisibilityComponent")
            .field(
                "handle",
                match self.handle {
                    Some(_) => &Some("VisibilityObjectArc"),
                    None => &Option::<&str>::None,
                },
            )
            .finish()
    }
}

pub trait RenderPlugin: Send + Sync + 'static {}

pub type RenderPlugins = Vec<Box<dyn RenderPlugin>>;

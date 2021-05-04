use std::fmt::Debug;

use bevy::{
    ecs::reflect::ReflectComponent,
    math::Vec3,
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
        FramePacket, FramePacketBuilder, RenderFeatureMask, RenderFeatureMaskBuilder,
        RenderPhaseMask, RenderPhaseMaskBuilder, RenderRegistry, RenderRegistryBuilder,
        RenderViewDepthRange, RenderViewSet,
    },
    rafx_visibility::{DepthRange, PerspectiveParameters, Projection},
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
        app
            // Improve ergonomics of Options, maybe replace_with?
            // Never actually None, but used to .take() and .replace() builder
            .insert_resource::<Option<RenderRegistryBuilder>>(
                Some(RenderRegistryBuilder::default()),
            )
            .insert_resource::<Option<RenderRegistry>>(None)
            .insert_resource(FramePacketBuilder::new())
            .insert_resource::<Option<FramePacket>>(None)
            .insert_resource(RenderViewSet::default())
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
            .add_system_to_stage(RenderStage::PreExtract, build_frame_packet.system())
            .add_system_to_stage(RenderStage::Extract, create_main_view.system());
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

fn create_main_view(
    windows: Res<Windows>,
    render_view_set_resource: ResMut<RenderViewSet>,
    visibility_region: Res<VisibilityRegion>,
    mut frame_packet_builder_resource: ResMut<FramePacketBuilder>,
    // TODO different projections
    query: Query<(&Camera, &PerspectiveProjection, &GlobalTransform)>,
) {
    // TODO multiple cameras
    let window = windows.get_primary().unwrap();
    let extents = (window.physical_width(), window.physical_height());

    let render_phase_mask = RenderPhaseMaskBuilder::default()
        .add_render_phase::<phases::opaque_render_phase::OpaqueRenderPhase>()
        .build();

    let render_feature_mask = RenderFeatureMaskBuilder::default().build();

    let (camera, projection, global_transform) = query.single().unwrap();

    let depth_range = RenderViewDepthRange::new(projection.near, projection.far);

    let view_frustum = visibility_region.register_view_frustum();

    let projection = Projection::Perspective(PerspectiveParameters::new(
        projection.fov,
        projection.aspect_ratio,
        projection.near,
        projection.far,
        DepthRange::Normal,
    ));

    let forward = global_transform.rotation * Vec3::Z;
    let up = global_transform.rotation * Vec3::Y;

    view_frustum.set_projection(&projection).set_transform(
        global_transform.translation,
        // TODO need to keep the lookat and up or calculate from rotation
        forward,
        up,
    );

    let main_view = render_view_set_resource.create_view(
        view_frustum,
        global_transform.translation,
        global_transform.compute_matrix(),
        projection.as_rh_mat4(),
        extents,
        depth_range,
        render_phase_mask,
        render_feature_mask,
        camera.name.clone().unwrap(),
    );

    frame_packet_builder_resource.query_visibility_and_add_results(&main_view, &visibility_region);
}

fn build_frame_packet(
    mut frame_packet_resource: ResMut<Option<FramePacket>>,
    mut frame_packet_builder_resource: ResMut<FramePacketBuilder>,
    mut render_view_set_resource: ResMut<RenderViewSet>,
    visibility_region: Res<VisibilityRegion>,
    // TODO different projections
    query: Query<(&Camera, &PerspectiveProjection, &GlobalTransform)>,
) {
    // Swap in the new frame_packet_builder for next frame
    let mut frame_packet_builder = FramePacketBuilder::new();
    std::mem::swap(
        &mut *frame_packet_builder_resource,
        &mut frame_packet_builder,
    );

    // build the frame packet and update the resource
    frame_packet_resource.replace(frame_packet_builder.build());

    // make the render_view_set for the next Frame and swap
    let mut render_view_set = RenderViewSet::default();
    std::mem::swap(&mut *render_view_set_resource, &mut render_view_set);
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

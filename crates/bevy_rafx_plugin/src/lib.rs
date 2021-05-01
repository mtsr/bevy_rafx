use bevy::{
    log,
    prelude::{CoreStage, IntoSystem, Plugin, StageLabel, SystemStage},
};
use rafx::{
    assets::distill_impl,
    nodes::FramePacketBuilder,
    visibility::{VisibilityObjectArc, VisibilityRegion},
};

use crate::daemon::AssetDaemonOpt;

pub use rafx::assets::distill_impl::AssetResource;

mod daemon;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderStage {
    Visibility,
    Extract,
    Prepare,
    Submit,
}

#[derive(Default)]
pub struct RafxPlugin;

impl Plugin for RafxPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(RenderPlugins::default())
            .insert_resource(VisibilityRegion::new())
            .insert_resource(distill_daemon())
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

fn distill_daemon() -> AssetResource {
    let asset_daemon_opt = AssetDaemonOpt::default();

    log::info!("Hosting local daemon at {:?}", asset_daemon_opt.address);

    let mut asset_daemon = distill_impl::default_daemon()
        .with_db_path(asset_daemon_opt.db_dir)
        .with_address(asset_daemon_opt.address)
        .with_asset_dirs(asset_daemon_opt.asset_dirs);

    // for plugin in render_plugins {
    //     asset_daemon = plugin.configure_asset_daemon(asset_daemon);
    // }

    // Spawn the daemon in a background thread.
    std::thread::spawn(move || {
        asset_daemon.run();
    });

    daemon::init_distill_daemon(asset_daemon_opt.address.to_string())
}

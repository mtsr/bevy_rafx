use bevy::prelude::*;
use bevy_rafx_plugin::{AssetResource, RafxPlugin};
use mesh_renderer::{assets::MeshAsset, MeshRendererPlugin};

fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins)
        .add_plugin(RafxPlugin::default())
        .add_plugin(MeshRendererPlugin::default())
        .add_startup_system(setup.system());

    #[cfg(feature = "print_schedule")]
    app.set_runner(print_schedule_runner);

    app.run();
}

fn setup(mut asset_resource: ResMut<AssetResource>) {
    // let handle: Handle< = asset_server.load("assets/models/Monkey.gltf");
    asset_resource.load_asset_path::<MeshAsset, _>("assets/models/Monkey.gltf");
}

#[cfg(feature = "print_schedule")]
fn print_schedule_runner(app: App) {
    use std::io::Write;

    let dot = bevy_mod_debugdump::schedule_graph_dot(&app.schedule);
    let mut file = std::fs::File::create("schedule.dot").unwrap();
    write!(file, "{}", dot).unwrap();
    println!("*** Updated schedule.dot");
}

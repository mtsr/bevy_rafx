use bevy::prelude::*;
use bevy_rafx_gltf::GltfPlugin;
use bevy_rafx_plugin::BevyRafxPlugin;
use mesh_renderer_plugin::MeshRendererPlugin;

fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins)
        .add_plugin(BevyRafxPlugin::default())
        .add_plugin(MeshRendererPlugin::default())
        .add_plugin(GltfPlugin)
        .add_startup_system(setup.system());

    #[cfg(feature = "print_schedule")]
    app.set_runner(print_schedule_runner);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("models/Monkey.gltf#Scene0");
    commands.spawn_scene(handle);
}

#[cfg(feature = "print_schedule")]
fn print_schedule_runner(app: App) {
    use std::io::Write;

    let dot = bevy_mod_debugdump::schedule_graph_dot(&app.schedule);
    let mut file = std::fs::File::create("schedule.dot").unwrap();
    write!(file, "{}", dot).unwrap();
    println!("*** Updated schedule.dot");
}

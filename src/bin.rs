use bevy::prelude::*;
use bevy_rafx::RafxPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RafxPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup() {}

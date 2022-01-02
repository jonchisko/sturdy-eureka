use bevy::prelude::*;
use hello_plugin::HelloPlugin;

mod hello_plugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

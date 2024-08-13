mod hello_plugin;

use bevy::prelude::*;
use hello_plugin::HelloPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

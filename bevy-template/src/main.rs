use bevy::prelude::*;
use duck_man::MainPlugin;

fn main() {
	App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(MainPlugin)
    .run();
}

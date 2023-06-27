use bevy::prelude::*;
use duck_man::MainPlugin;

fn main() {
	App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(MainPlugin)
		.add_plugin(bevy_editor_pls::prelude::EditorPlugin::default())
    .run();
}

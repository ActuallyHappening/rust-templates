use bevy::prelude::*;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_mod_picking::prelude::*;
use duck_man::MainPlugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(MainPlugin)
		.add_plugin(bevy_editor_pls::prelude::EditorPlugin::default())
		// .add_plugin(FlyCameraPlugin)
		.add_plugins(
			DefaultPickingPlugins
				.build()
				.disable::<DefaultHighlightingPlugin>()
				.disable::<DebugPickingPlugin>()
				.enable::<RapierBackend>(),
		)
		.run();
}

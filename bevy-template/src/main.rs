use bevy::prelude::*;
use tracing::{debug, info};

fn main() {
	{{crate_name}}::init_debug_tools();

	info!("Bevy app running ...");

	App::new()
		// startup systems
		.add_startup_system(hello_world)
		// plugins	
		.add_plugins(DefaultPlugins)
		// run
		.run();

	debug!("Bevy app finished.");
}

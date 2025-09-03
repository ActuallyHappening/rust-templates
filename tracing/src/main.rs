use tracing::*;

fn main() {
	example_name::init_debug_tools("example-name=debug").unwrap();
	debug!("Logging started");

	info!("Hello, world!");
}

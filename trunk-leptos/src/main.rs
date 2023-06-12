use leptos::prelude::*;
use trunk_leptos::app::*;

fn main() {
	trunk_leptos::init_debug_tools();

	println!("Hello, world!");

	leptos::mount_to_body(move |cx| {
		leptos::view! {cx, <App/> }
	});
}

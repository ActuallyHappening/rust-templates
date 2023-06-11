fn main() {
	trunk_leptos::init_debug_tools();

	println!("Hello, world!");

	leptos::mount_to_body(move |cx| {
		view! {cx, <App/> }
	});
}

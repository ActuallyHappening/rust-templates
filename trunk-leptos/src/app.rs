use leptos::prelude::*;
use leptos::{component, Scope, IntoView};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	leptos::view! {cx, <div>"Hello world!"</div>}
}
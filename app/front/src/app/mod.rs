use leptos::prelude::*;
mod page;

#[component]
pub fn App() -> impl IntoView {
	/*
	let (text, set_text) = signal(None);

	set_text.set(Some("Hello, World!".to_string()));

	if text.get().is_none() {
		return view! {
			<p>
				"Loading..."
			</p>
		};
	}
	*/

	let (text, set_text) = signal("If this is all it takes to tear us apart, then maybe we weren't all that close to begin with.".to_string().replace(" ", "_"));
	// let (text, set_text) = signal("".to_string());

	view! { <page::Typing text=text /> }
}

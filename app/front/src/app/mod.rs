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

	let (text, set_text) = signal("Hello, World!".to_string());

	view! { <page::Typing text=text /> }
}

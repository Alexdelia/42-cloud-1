use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
	let (index, set_index) = signal(0);
	let (text, set_text) = signal(None);

	set_text.set(Some("Hello, World!".to_string()));

	/*
	if text.get().is_none() {
		return view! {
			<p>
				"Loading..."
			</p>
		};
	}
	*/

	view! {
		<button on:click=move |_| *set_index.write() += 1>"Click me: " {index}</button>
		<p>"Double index: " {move || index.get() * 2}</p>
		<p>{text}</p>
		<progress
			max=move || text.get().expect("text is None").len()
			value=index
		/>
	}
}

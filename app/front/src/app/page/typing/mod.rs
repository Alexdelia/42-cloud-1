use leptos::prelude::*;
mod field;

stylance::import_crate_style!(style, "src/app/page/typing/style.module.scss");

#[component]
pub fn Typing() -> impl IntoView {
	let (text, set_text) = signal("If this is all it takes to tear us apart, then maybe we weren't all that close to begin with.".to_string().replace(" ", "_"));
	let (index, set_index) = signal(0usize);

	window_event_listener(leptos::ev::keydown, move |event| {
		let mut key = event.key();
		if key.len() != 1 {
			return;
		}

		if key == " " {
			key = "_".to_string();
		}

		let target = text
			.get()
			.chars()
			.nth(index.get())
			.expect("Index out of bounds");

		if key != target.to_string() {
			leptos::logging::log!("Key pressed: '{key}' != '{target}'");
			return;
		}

		set_index.set((index.get() + 1) % text.get().len());
	});

	view! {
		<div class=style::container>
			<Show
				when=move || { !text.get().is_empty() }
				fallback=|| view! { <p>"Finding a cool quote..."</p> }
			>
				<field::Field text=text index=index />

				<progress max=move || text.get().len() - 1 value=index />
			</Show>
		</div>
	}
}

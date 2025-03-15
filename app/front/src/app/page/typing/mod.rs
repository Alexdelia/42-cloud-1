use leptos::prelude::*;
mod field;
mod on_keydown;

stylance::import_crate_style!(style, "src/app/page/typing/style.module.scss");

#[component]
pub fn Typing() -> impl IntoView {
	let (text, set_text) = signal("If this is all it takes to tear us apart, then maybe we weren't all that close to begin with.".to_string().replace(" ", "_"));
	let (index, set_index) = signal(0usize);

	on_keydown::set_event_listener(text.clone(), index.clone(), set_index);

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

use leptos::prelude::*;
mod field;
mod on_keydown;
mod wrong_key_animation;

stylance::import_crate_style!(style, "src/app/page/typing/style.module.css");

#[component]
pub fn Typing() -> impl IntoView {
	let (text, set_text) = signal("If this is all it takes to tear us apart, then maybe we weren't all that close to begin with.".to_string().replace(" ", "_"));
	let (index, set_index) = signal(0usize);
	let (animations, set_animations) = signal(Vec::<(String, usize)>::new());
	let animation_id = create_rw_signal(0);

	on_keydown::set_event_listener(
		text.clone(),
		index.clone(),
		set_index,
		set_animations,
		animation_id,
	);

	view! {
		<div class=style::container>
			<Show
				when=move || { !text.get().is_empty() }
				fallback=|| view! { <p>"Finding a cool quote..."</p> }
			>
				<field::Field text=text index=index />

				// TODO: progress bar styling
				<progress max=move || text.get().len() - 1 value=index />
			</Show>
		</div>

		<wrong_key_animation::WrongKeyAnimations animations=animations />
	}
}

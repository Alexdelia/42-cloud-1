use leptos::prelude::*;

stylance::import_crate_style!(style, "src/app/page/typing/wrong_key_animation.module.css");

#[component]
pub fn WrongKeyAnimations(animations: ReadSignal<Vec<(String, usize)>>) -> impl IntoView {
	view! {
		<div class=style::animation_container>
			<For
				each=move || animations.get()
				key=|(_, id)| *id
				children=move |(key, id)| {
					view! {
						<div
							class=style::wrong_key_animation
							style=format!("z-index: {}", 8 + id)
						>
							{key}
						</div>
					}
				}
			/>
		</div>
	}
}

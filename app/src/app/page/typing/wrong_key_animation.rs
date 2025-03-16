use leptos::prelude::*;

use super::Animation;

#[component]
pub fn WrongKeyAnimations(animations: ReadSignal<Vec<Animation>>) -> impl IntoView {
	view! {
		<div class="wrong_key_animation_container">
			<For
				each=move || animations.get()
				key=|(_, id)| *id
				children=move |(key, id)| {
					view! {
						<div
							class="wrong_key_animation"
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

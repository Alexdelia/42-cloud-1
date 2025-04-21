use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
	view! {
		<div class="not_found">
			<div class="overlay">
				<h1>"404"</h1>
			</div>
		</div>
	}
}

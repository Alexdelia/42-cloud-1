use leptos::prelude::*;

use super::Quote;

#[component]
pub fn Origin(quote: Resource<Result<Quote, ServerFnError>>) -> impl IntoView {
	let quote = move || {
		quote
			.get()
			.expect("Quote is empty")
			.expect("Quote is error")
	};

	view! {
		<div class="quote_origin">
			<span class="dash">"—"</span>
			<span>{move || quote().character}</span>
		</div>
	}
}

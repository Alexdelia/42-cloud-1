use leptos::prelude::*;

use super::Quote;

const VISIBLE_CHAR_RADIUS: usize = 16;

#[component]
pub fn Field(quote: ReadSignal<Option<Quote>>, index: ReadSignal<usize>) -> impl IntoView {
	let t = move || quote.get().expect("Quote is empty").text;
	let max = t().len();
	let i = move || index.get();

	view! {
		<div
			class="typing_field"
			style="display: flex; justify-content: center"
		>
			<span class="prev">
				{move || {
					t()[(i().saturating_sub(VISIBLE_CHAR_RADIUS))..i()]
						.to_string()
				}}
			</span>
			<span class="current">
				{move || { t()[i()..(i() + 1).min(max)].to_string() }}
			</span>
			<span class="next">
				{move || {
					t()[(i() + 1)
							.min(max)..(i() + VISIBLE_CHAR_RADIUS).min(max)]
						.to_string()
				}}
			</span>
		</div>
	}
}

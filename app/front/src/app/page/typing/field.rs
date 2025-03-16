use leptos::prelude::*;

use super::Quote;

stylance::import_crate_style!(style, "src/app/page/typing/field.module.css");

const VISIBLE_CHAR_RADIUS: usize = 16;

#[component]
pub fn Field(quote: ReadSignal<Quote>, index: ReadSignal<usize>) -> impl IntoView {
	let t = move || quote.get().text;
	let max = move || t().len();
	let i = move || index.get();

	view! {
		<div
			class=style::typing_field
			style="display: flex; justify-content: center"
		>
			<span class=style::prev>
				{move || {
					t()[(i().saturating_sub(VISIBLE_CHAR_RADIUS))..i()]
						.to_string()
				}}
			</span>
			<span class=style::current>
				{move || { t()[i()..(i() + 1).min(max())].to_string() }}
			</span>
			<span class=style::next>
				{move || {
					t()[(i() + 1)
							.min(max())..(i() + VISIBLE_CHAR_RADIUS).min(max())]
						.to_string()
				}}
			</span>
		</div>
	}
}

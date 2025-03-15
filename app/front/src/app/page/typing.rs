use core::num::Saturating;

use leptos::prelude::*;

stylance::import_crate_style!(style, "src/app/page/typing.module.scss");

#[component]
pub fn Typing(text: ReadSignal<String>) -> impl IntoView {
	let (index, set_index) = signal(0usize);

	view! {
		<div class=style::container>
			<Show
				when=move || { !text.get().is_empty() }
				fallback=|| view! { <p>"Loading..."</p> }
			>
				<button on:click=move |_| {
					set_index.set((index.get() + 1) % text.get().len())
				}>"Click me: " {index}</button>
				<Field text=text index=index />

				<progress max=move || text.get().len() - 1 value=index />
			</Show>
		</div>
	}
}

const VISIBLE_CHAR_RADIUS: usize = 8;

#[component]
fn Field(text: ReadSignal<String>, index: ReadSignal<usize>) -> impl IntoView {
	let t = move || text.get();
	let max = move || text.get().len();
	let i = move || index.get();

	view! {
		<div
			class=style::typing_field
			style="display: flex; justify-content: center"
		>
			<span style=format!(
				"width: {VISIBLE_CHAR_RADIUS}rem; text-align: right; color: green",
			)>
				{move || {
					t()[(i().saturating_sub(VISIBLE_CHAR_RADIUS))..i()]
						.to_string()
				}}
			</span>
			<span style="color: red">
				{move || { t()[i()..(i() + 1).min(max())].to_string() }}
			</span>
			<span>
				{move || {
					t()[(i() + 1)
							.min(max())..(i() + VISIBLE_CHAR_RADIUS).min(max())]
						.to_string()
				}}
			</span>
		</div>
	}
}

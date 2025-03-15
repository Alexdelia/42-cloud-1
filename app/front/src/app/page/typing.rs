use leptos::prelude::*;

#[component]
pub fn Typing(text: ReadSignal<String>) -> impl IntoView {
	let (index, set_index) = signal(0usize);

	view! {
		<button on:click=move |_| {
			set_index.set((index.get() + 1) % text.get().len())
		}>"Click me: " {index}</button>
		<Field text=text index=index />
		<progress max=move || text.get().len() - 1 value=index />
	}
}

#[component]
fn Field(text: ReadSignal<String>, index: ReadSignal<usize>) -> impl IntoView {
	let t = move || text.get();
	let max = move || t().len() - 1;
	let i = move || index.get();

	view! {
		<div style="display: flex; justify-content: center">
			<span>{move || t()[..i()].to_string()}</span>
			<span style="font-weight: bold">
				{move || { t()[i()..(i() + 1).min(max())].to_string() }}
			</span>
			<span>{move || t()[(i() + 1).min(max())..].to_string()}</span>
		</div>
	}
}

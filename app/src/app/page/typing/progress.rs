use leptos::prelude::*;

#[component]
pub fn Progress(value: ReadSignal<usize>, max: usize) -> impl IntoView {
	view! { <progress value=value max=max /> }
}

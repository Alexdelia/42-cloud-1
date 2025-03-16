use leptos::prelude::*;

stylance::import_crate_style!(style, "src/app/page/typing/progress.module.css");

#[component]
pub fn Progress(value: ReadSignal<usize>, max: usize) -> impl IntoView {
	view! { <progress value=value max=max /> }
}

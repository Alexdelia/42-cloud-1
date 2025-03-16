use leptos::prelude::*;

stylance::import_crate_style!(style, "src/app/page/typing/progress.module.css");

#[component]
pub fn Progress(text: ReadSignal<String>, index: ReadSignal<usize>) -> impl IntoView {
	view! {
		<progress max=move || text.get().len() - 1 value=index />
	}
}

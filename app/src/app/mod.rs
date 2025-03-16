use leptos::prelude::*;
mod page;

#[component]
pub fn App() -> impl IntoView {
	view! { <page::Typing /> }
}

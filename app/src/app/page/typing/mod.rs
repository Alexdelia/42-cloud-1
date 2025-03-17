mod field;
mod on_keydown;
mod progress;
mod query_quote;
mod stats;
mod wrong_key_animation;

use crate::schema::{quote::Quote, stats::Stats};
use leptos::prelude::*;

type Animation = (char, usize);

#[component]
pub fn Typing() -> impl IntoView {
	let (new_quote_requested, trigger_new_quote) = signal(0u8);
	let (index, set_index) = signal(0usize);
	let (stats, set_stats) = signal(Stats::default());
	let (animations, set_animations) = signal(Vec::<Animation>::new());
	let animation_id = RwSignal::new(0usize);

	let quote = Resource::new(
		move || new_quote_requested.get(),
		|_| query_quote::query_quote(),
	);

	Effect::watch(
		move || new_quote_requested.get(),
		move |_, _, _| {
			set_index.set(0);
			set_stats.update(|stats| *stats = Stats::default());
		},
		true,
	);

	on_keydown::set_event_listener(
		quote,
		trigger_new_quote,
		index,
		set_index,
		stats,
		set_stats,
		set_animations,
		animation_id,
	);

	view! {
		<div class="page_typing_container">
			<Transition fallback=move || {
				view! { <p>"[Transition] Finding a cool quote..."</p> }
			}>
				<Show
					when=move || {
						quote.get().map(|q| q.is_ok()).unwrap_or(false)
					}
					fallback=|| {
						view! { <p>"[Show] Finding a cool quote..."</p> }
					}
				>
					<stats::Ongoing stats=stats />

					<field::Field quote=quote index=index />

					<progress::Progress
						value=index
						max=quote.get().unwrap().unwrap().text.len()
					/>
				</Show>
			</Transition>
		</div>

		<wrong_key_animation::WrongKeyAnimations animations=animations />
	}
}

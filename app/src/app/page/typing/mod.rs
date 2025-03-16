mod field;
mod on_keydown;
mod progress;
// mod query_quote;
mod wrong_key_animation;

use crate::schema::{
	quote::{Anime, Quote},
	stats::Stats,
};
use leptos::prelude::*;

stylance::import_crate_style!(style, "src/app/page/typing/style.module.css");

type Animation = (char, usize);

#[component]
pub fn Typing() -> impl IntoView {
	let (quote, set_quote) = signal(None);
	let (new_quote_requested, trigger_new_quote) = signal(0u8);
	let (index, set_index) = signal(0usize);
	let (stats, set_stats) = signal(Stats::default());
	let (animations, set_animations) = signal(Vec::<Animation>::new());
	let animation_id = RwSignal::new(0usize);

	set_quote.set(Some(Quote {
        text: "If this is all it takes to tear us apart, then maybe we weren't all that close to begin with.".to_string().replace(" ", "_"),
        character: "Yukino Yukinoshita".to_string(),
        anime: Anime {
            name: "Oregairu".to_string(),
            alt_name: "My Teen Romantic Comedy SNAFU".to_string(),
        },
    }));

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
		trigger_new_quote.clone(),
		index.clone(),
		set_index,
		stats.clone(),
		set_stats,
		set_animations,
		animation_id,
	);

	view! {
		<div class=style::container>
			<Show
				when=move || { !quote.get().is_none() }
				fallback=|| view! { <p>"Finding a cool quote..."</p> }
			>
				<stats::Ongoing stats=stats />

				<field::Field quote=quote index=index />

				<progress::Progress
					value=index
					max=quote.get().unwrap().text.len()
				/>
			</Show>
		</div>

		<wrong_key_animation::WrongKeyAnimations animations=animations />
	}
}

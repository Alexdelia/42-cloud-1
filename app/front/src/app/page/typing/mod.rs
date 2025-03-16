use chrono::{DateTime, Utc};
use leptos::prelude::*;
mod field;
mod on_keydown;
mod progress;
mod query_quote;
mod stats;
mod wrong_key_animation;

stylance::import_crate_style!(style, "src/app/page/typing/style.module.css");

#[derive(Default, Clone)]
struct Quote {
	text: String,

	character: String,
	anime: Anime,
}

#[derive(Default, Clone)]
struct Anime {
	name: String,
	alt_name: String,
}

type Animation = (char, usize);

#[component]
pub fn Typing() -> impl IntoView {
	let (quote, set_quote) = signal(Quote::default());
	let new_quote = RwSignal::new(0u8);
	let (index, set_index) = signal(0usize);
	let (stats, set_stats) = signal(Stats::default());
	let (animations, set_animations) = signal(Vec::<Animation>::new());
	let animation_id = RwSignal::new(0usize);

	Effect::watch(
		move || new_quote.get(),
		move |_, _, _| {
			set_quote.set(query_quote::query_quote());
			set_index.set(0);
			set_stats.update(|stats| *stats = Stats::default());
		},
		true,
	);

	on_keydown::set_event_listener(
		quote.clone(),
		new_quote,
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
				when=move || { !quote.get().text.is_empty() }
				fallback=|| view! { <p>"Finding a cool quote..."</p> }
			>
				<stats::Ongoing stats=stats />

				<field::Field quote=quote index=index />

				<progress::Progress
					value=index
					max=quote.get().text.len()
				/>
			</Show>
		</div>

		<wrong_key_animation::WrongKeyAnimations animations=animations />
	}
}

#[derive(Clone)]
struct Stats {
	start_time: DateTime<Utc>,
	end_time: DateTime<Utc>,

	correct_key: usize,
	wrong_key: usize,
}

impl Default for Stats {
	fn default() -> Self {
		Self {
			start_time: Utc::now(),
			end_time: Utc::now(),

			correct_key: 0,
			wrong_key: 0,
		}
	}
}

impl Stats {
	pub fn wpm(&self) -> f64 {
		let count = self.key_count();
		if count <= 1 {
			return 0.0;
		}

		let duration = self.end_time - self.start_time;
		let minutes = duration.num_milliseconds() as f64 / 1000.0 / 60.0;
		if minutes == 0.0 {
			return 0.0;
		}

		let words = count as f64 / 5.0;
		words / minutes
	}

	pub fn accuracy(&self) -> f64 {
		if self.wrong_key == 0 {
			return 100.0;
		}
		if self.correct_key == 0 {
			return 0.0;
		}

		self.correct_key as f64 / self.key_count() as f64 * 100.0
	}

	pub fn key_count(&self) -> usize {
		self.correct_key + self.wrong_key
	}
}

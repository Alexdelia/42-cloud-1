use chrono::{DateTime, Utc};
use leptos::prelude::*;
mod field;
mod on_keydown;
mod wrong_key_animation;

stylance::import_crate_style!(style, "src/app/page/typing/style.module.css");

type Animation = (char, usize);

#[component]
pub fn Typing() -> impl IntoView {
	let (text, set_text) = signal("If this is all it takes to tear us apart, then maybe we weren't all that close to begin with.".to_string().replace(" ", "_"));
	let (index, set_index) = signal(0usize);
	let (stats, set_stats) = signal(Stats::default());
	let (animations, set_animations) = signal(Vec::<Animation>::new());
	let animation_id = RwSignal::new(0usize);

	on_keydown::set_event_listener(
		text.clone(),
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
				when=move || { !text.get().is_empty() }
				fallback=|| view! { <p>"Finding a cool quote..."</p> }
			>
				<p>
					<span>{move || {
				let s = stats.get();
				format!("WPM: {:.2}", if s.key_count() > 1 {s.wpm()} else {0.0})
			}}</span>
					<span>{move || format!("Accuracy: {:.2}%", stats.get().accuracy())}</span>
				</p>
				<field::Field text=text index=index />

				// TODO: progress bar styling
				<progress max=move || text.get().len() - 1 value=index />
			</Show>
		</div>

		<wrong_key_animation::WrongKeyAnimations animations=animations />
	}
}

#[derive(Clone)]
struct Stats {
	start_time: DateTime<Utc>,
	end_time: Option<DateTime<Utc>>,

	correct_key: usize,
	wrong_key: usize,
}

impl Default for Stats {
	fn default() -> Self {
		Self {
			start_time: Utc::now(),
			end_time: None,

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

		let duration = self.end_time.unwrap_or(Utc::now()) - self.start_time;
		let minutes = duration.num_milliseconds() as f64 / 1000.0 / 60.0;
		if minutes == 0.0 {
			return 0.0;
		}

		let words = count as f64 / 5.0;
		words / minutes
	}

	pub fn accuracy(&self) -> f64 {
		if self.correct_key == 0 {
			return 0.0;
		}

		self.correct_key as f64 / self.key_count() as f64 * 100.0
	}

	pub fn key_count(&self) -> usize {
		self.correct_key + self.wrong_key
	}
}

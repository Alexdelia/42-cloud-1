mod field;
mod on_keydown;
mod progress;
// mod query_quote;
mod stats;
mod wrong_key_animation;

use chrono::{DateTime, Utc};
use leptos::{prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};
use server_fn::codec::GetUrl;

stylance::import_crate_style!(style, "src/app/page/typing/style.module.css");

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Quote {
	text: String,

	character: String,
	anime: Anime,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Anime {
	name: String,
	alt_name: String,
}

type Animation = (char, usize);

pub async fn query_quote() -> Quote {
	let URL: &str = "https://animechan.io/api/v1/quotes/random";

	let res = reqwest::get(URL).await;

	leptos::logging::log!("res: {:?}", res);

	Quote {
		text: "yes".to_string(),
		character: "Yukino Yukinoshita".to_string(),
		anime: Anime {
			name: "Oregairu".to_string(),
			alt_name: "My Teen Romantic Comedy SNAFU".to_string(),
		},
	}
}

#[component]
pub fn Typing() -> impl IntoView {
	let (quote, set_quote) = signal(None);
	let (new_quote_requested, trigger_new_quote) = signal(0u8);
	let (index, set_index) = signal(0usize);
	let (stats, set_stats) = signal(Stats::default());
	let (animations, set_animations) = signal(Vec::<Animation>::new());
	let animation_id = RwSignal::new(0usize);

	// let quote = Resource::new(
	// move || new_quote_requested.get(),
	// |_| query_quote::query_quote(),
	// );
	// let quote = fetch_data();

	Effect::watch(
		move || new_quote_requested.get(),
		move |_, _, _| {
			let quote_resource = LocalResource::new(move || query_quote());
			if let Some(quote) = quote_resource.get().as_deref() {
				set_quote.set(Some(quote.clone()));
			}
			// spawn_local(async move { set_quote.set(Some(query_quote().await.unwrap())) });
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

// unsafe impl Send for Quote {}
// unsafe impl Sync for Quote {}

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

use super::{Animation, Quote, State, Stats};
use chrono::Utc;
use leptos::prelude::*;

pub fn set_event_listener(
	quote: Resource<Result<Quote, ServerFnError>>,
	(state, set_state): (ReadSignal<State>, WriteSignal<State>),
	(index, set_index): (ReadSignal<usize>, WriteSignal<usize>),
	(stats, set_stats): (ReadSignal<Stats>, WriteSignal<Stats>),
	set_animations: WriteSignal<Vec<Animation>>,
	animation_id: RwSignal<usize>,
) {
	let handle = window_event_listener(leptos::ev::keydown, move |event| {
		let Some(Ok(quote)) = quote.get() else {
			leptos::logging::log!("no quote to type");
			return;
		};
		let text = quote.text;
		let index = index.get();

		if state.get() == State::Typing && text.len() <= index {
			set_state.set(State::Completed);
			return;
		}
		let over = state.get() == State::Completed;

		let key = event.key();
		if key.len() != 1 {
			if (over && key == "Enter") || key == "Escape" {
				set_state.set(State::Reset);
			}
			return;
		}

		if text.is_empty() || over {
			return;
		}
		set_stats.update(|stats| stats.end_time = Utc::now());

		let mut key = key.chars().next().expect("Key is empty");

		if key == ' ' {
			key = '_';
		}

		let target = text.chars().nth(index).expect("Index out of bounds");

		let stats = stats.get();
		let new_key = stats.key_count() < index;

		if key != target {
			handle_wrong_key(key, new_key, set_stats, set_animations, animation_id);
			return;
		}

		set_index.set(index + 1);

		if new_key {
			set_stats.update(|stats| stats.correct_key += 1);
		}

		if stats.correct_key == 0 && stats.wrong_key <= 1 {
			set_stats.update(|stats| stats.start_time = Utc::now());
		}
	});

	on_cleanup(move || {
		handle.remove();
	});
}

fn handle_wrong_key(
	key: char,
	new_key: bool,
	set_stats: WriteSignal<Stats>,
	set_animations: WriteSignal<Vec<Animation>>,
	animation_id: RwSignal<usize>,
) {
	let id = animation_id.get();
	animation_id.update(|id| *id = (*id + 1) % u32::MAX as usize);
	set_animations.update(|animations| animations.push((key, id)));
	set_timeout(
		move || {
			set_animations.update(|animations| animations.retain(|(_, anim_id)| *anim_id != id));
		},
		std::time::Duration::from_secs(2),
	);

	if new_key {
		set_stats.update(|stats| stats.wrong_key += 1);
	}
}

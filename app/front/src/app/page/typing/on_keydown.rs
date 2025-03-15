use leptos::prelude::*;

pub fn set_event_listener(
	text: ReadSignal<String>,
	index: ReadSignal<usize>,
	set_index: WriteSignal<usize>,
	set_animations: WriteSignal<Vec<(String, usize)>>,
	animation_id: RwSignal<usize>,
) {
	let handle = window_event_listener(leptos::ev::keydown, move |event| {
		let mut key = event.key();
		if key.len() != 1 {
			return;
		}

		if key == " " {
			key = "_".to_string();
		}

		let target = text
			.get()
			.chars()
			.nth(index.get())
			.expect("Index out of bounds");

		if key != target.to_string() {
			leptos::logging::log!("Key pressed: '{key}' != '{target}'");

			let id = animation_id.get();
			animation_id.update(|id| *id = (*id + 1) % u32::MAX as usize);
			set_animations.update(|animations| animations.push((key.clone(), id)));
			set_timeout(
				move || {
					set_animations
						.update(|animations| animations.retain(|(_, anim_id)| *anim_id != id));
				},
				std::time::Duration::from_secs(2),
			);
			return;
		}

		set_index.set((index.get() + 1) % text.get().len());
	});

	on_cleanup(move || {
		handle.remove();
	});
}

use leptos::prelude::*;

pub fn set_event_listener(
	text: ReadSignal<String>,
	index: ReadSignal<usize>,
	set_index: WriteSignal<usize>,
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
			return;
		}

		set_index.set((index.get() + 1) % text.get().len());
	});

	on_cleanup(move || {
		handle.remove();
	});
}

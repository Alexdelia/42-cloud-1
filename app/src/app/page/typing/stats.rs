use leptos::prelude::*;

use crate::app::dynamic_style;

use super::{State, Stats};

#[component]
pub fn Ongoing(stats: ReadSignal<Stats>, state: ReadSignal<State>) -> impl IntoView {
	view! {
		<div class="stats_ongoing_container">
			<div class="item" style="right: calc(50vw + 3rem)">
				<span
					class=move || {
						let mut c = String::from("value");
						if state.get() == State::Completed {
							c.push_str(" completed");
						}
						c
					}
					style=move || {
						dynamic_style::wpm::gradient(stats.get().wpm())
					}
				>
					{move || format!("{:.1}", stats.get().wpm())}
				</span>
				<span class="unit">wpm</span>
			</div>

			<div class="item" style="right: calc(50vw - 18rem)">
				<span
					class="value"
					style=move || {
						dynamic_style::accuracy::gradient(stats.get().accuracy())
					}
				>
					{move || format!("{:.2}%", stats.get().accuracy())}
				</span>
				<span class="unit">accuracy</span>
			</div>
		</div>
	}
}

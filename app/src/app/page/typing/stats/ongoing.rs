use leptos::prelude::*;

use crate::app::page::typing::Stats;

stylance::import_crate_style!(style, "src/app/page/typing/stats/ongoing.module.css");

#[component]
pub fn Ongoing(stats: ReadSignal<Stats>) -> impl IntoView {
	view! {
		<div class=style::container>
			<div class=style::item style="right: calc(50vw + 3rem)">
				<span
					class=style::value
					style=move || { wpm_gradient(stats.get().wpm()) }
				>
					{move || format!("{:.1}", stats.get().wpm())}
				</span>
				<span class=style::unit>wpm</span>
			</div>

			<div class=style::item style="right: calc(50vw - 18rem)">
				<span
					class=style::value
					style=move || { accuracy_gradient(stats.get().accuracy()) }
				>
					{move || format!("{:.2}%", stats.get().accuracy())}
				</span>
				<span class=style::unit>accuracy</span>
			</div>
		</div>
	}
}

fn wpm_gradient(wpm: f64) -> String {
	if wpm == 0.0 {
		return "color: hsl(360, 50%, 100%)".to_string();
	}

	let min = 30.0;
	let max = 120.0;
	let hue = relative(wpm, min, max, 240.0, 360.0);
	let lum = relative(wpm, min, max, 50.0, 100.0);
	format!("color: hsl({hue}, 50%, {lum}%)")
}

fn accuracy_gradient(accuracy: f64) -> String {
	let min = 80.0;
	let max = 100.0;
	let hue = relative(accuracy, min, max, 0.0, 120.0);
	let sat = relative(accuracy, min, max, 40.0, 70.0);
	format!("color: hsl({hue}, {sat}%, 50%)")
}

fn relative(k: f64, min: f64, max: f64, low: f64, high: f64) -> f64 {
	(low + (high - low) * ((k - min) / (max - min)))
		.max(low)
		.min(high)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn relative_test() {
		assert_eq!(relative(0.0, 0.0, 1.0, 0.0, 1.0), 0.0);
		assert_eq!(relative(0.5, 0.0, 1.0, 0.0, 1.0), 0.5);
		assert_eq!(relative(1.0, 0.0, 1.0, 0.0, 1.0), 1.0);
		assert_eq!(relative(0.0, 0.8, 1.0, 42.0, 84.0), 42.0);
		assert_eq!(relative(0.5, 0.8, 1.0, 42.0, 84.0), 42.0);
		assert_eq!(relative(0.9, 0.8, 1.0, 42.0, 84.0), (42.0 + 84.0) / 2.0);
		assert_eq!(relative(1.0, 0.8, 1.0, 42.0, 84.0), 84.0);

		assert_eq!(relative(80.0, 80.0, 100.0, 0.0, 120.0), 0.0);
		assert_eq!(relative(90.0, 80.0, 100.0, 0.0, 120.0), 60.0);
		assert_eq!(relative(100.0, 80.0, 100.0, 0.0, 120.0), 120.0);

		assert_eq!(relative(80.0, 80.0, 100.0, 40.0, 70.0), 40.0);
		assert_eq!(relative(90.0, 80.0, 100.0, 40.0, 70.0), 55.0);
		assert_eq!(relative(100.0, 80.0, 100.0, 40.0, 70.0), 70.0);
	}
}

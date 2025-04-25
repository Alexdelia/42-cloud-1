use crate::{app::dynamic_style, schema::stats::query::ComputedStatsRow};
use chrono::Local;
use leptos::prelude::*;

#[component]
pub fn Row(row: ComputedStatsRow) -> impl IntoView {
	view! {
		<tr>
			<td style=move || {
				dynamic_style::wpm::gradient(row.wpm)
			}>{format!("{:.1}", row.wpm)}</td>
			<td style=move || {
				dynamic_style::accuracy::gradient(row.accuracy)
			}>{format!("{:.2}%", row.accuracy)}</td>
			<td>{format!("{:.3}s", row.duration_seconds)}</td>
			<td>
				{row
					.timestamp
					.with_timezone(&Local)
					.format("%F %X")
					.to_string()}
			</td>
		</tr>
	}
}

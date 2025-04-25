use crate::schema::stats::query::ComputedStatsRow;
use chrono::Local;
use leptos::prelude::*;

#[component]
pub fn Row(row: ComputedStatsRow) -> impl IntoView {
	view! {
		<tr>
			<td>{format!("{:.1}", row.wpm)}</td>
			<td>{format!("{:.2}%", row.accuracy)}</td>
			<td>{row.duration_seconds}</td>
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

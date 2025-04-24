use crate::schema::stats::query::ComputedStatsRow;
use leptos::prelude::*;

#[component]
pub fn Row(row: ComputedStatsRow) -> impl IntoView {
	view! {
		<tr>
			<td>{row.timestamp.to_string()}</td>
			<td>{format!("{:.1}", row.wpm)}</td>
			<td>{format!("{:.2}%", row.accuracy)}</td>
			<td>{row.duration_seconds}</td>
		</tr>
	}
}

use leptos::prelude::*;

use crate::schema::stats::Stats;
use uuid::Uuid;

#[component]
pub fn History(user_uuid: Uuid) -> impl IntoView {
	let stats_list = Resource::new(
		move || user_uuid,
		move |user_uuid| crate::schema::stats::query::list(user_uuid.unwrap_or_default()),
	);

	view! {
		<table class="table">
			<thead>
				<tr>
					<th>"Date"</th>
					<th>"WPM"</th>
					<th>"Accuracy"</th>
				</tr>
			</thead>
			<tbody>
				<For
					each=rows
					key=|row| row.end_time
					children=move |row| {
						view! {
							<tr>
								<td>{row.end_time.to_string()}</td>
								<td>{row.wpm()}</td>
								<td>{row.accuracy()}</td>
							</tr>
						}
					}
				/>
			</tbody>
		</table>
	}
}

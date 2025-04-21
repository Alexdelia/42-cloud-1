use leptos::prelude::*;

use crate::schema::stats::Stats;

#[component]
pub fn List(rows: Resource<Result<Vec<Stats>, ServerFnError>>) -> impl IntoView {
	let rows = move || rows.get().expect("Stats is empty").expect("Stats is error");

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

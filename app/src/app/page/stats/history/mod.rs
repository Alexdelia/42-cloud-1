mod list;
mod personal_best;
mod row;

use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn History(user_uuid: Uuid) -> impl IntoView {
	view! {
		<table>
			<thead>
				<tr>
					<th>"wpm"</th>
					<th>"accuracy"</th>
					<th>"time"</th>
					<th>"date"</th>
				</tr>
			</thead>
			<tbody>
				<personal_best::PersonalBest user_uuid=user_uuid />
				<list::List user_uuid=user_uuid />
			</tbody>
		</table>
	}
}

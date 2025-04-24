// mod list;
mod personal_best;

use crate::schema::stats::Stats;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn History(user_uuid: Uuid) -> impl IntoView {
	view! {
		<div class="stats_history">
			<personal_best::PersonalBest user_uuid=user_uuid />
		// <list::List user_uuid=user_uuid />
		</div>
	}
}

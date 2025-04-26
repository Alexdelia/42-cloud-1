mod bubble;
use bubble::StatsBubble;

use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn GlobalCompare(user_uuid: Uuid) -> impl IntoView {
	view! {
		<div class="stats_global_compare">
			<div class="stats_row">

				<StatsBubble
					title="wpm avg".to_owned()
					user="42.0"
					global="69.0"
				/>

				<StatsBubble
					title="wpm med".to_owned()
					user="42.0"
					global="69.0"
				/>
			</div>

		</div>
	}
}

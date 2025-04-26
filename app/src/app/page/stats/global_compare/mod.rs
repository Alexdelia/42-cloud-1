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

		/*
				<div class="stats_row">
					<StatsBubble
						title="acc avg".to_owned()
						user="42.00%"
						global="69.00%"
					/>
					<StatsBubble
						title="acc med".to_owned()
						user="42.00%"
						global="69.00%"
					/>
				</div>

				<div class="stats_row">
					<StatsBubble
						title="sum keys".to_owned()
						user="100 000 000"
						global="1 000 000 000"
					/>
					<StatsBubble
						title="sum time".to_owned()
						user="1000:00:00"
						global="9000:00:00"
					/>
					<StatsBubble
						title="sum session".to_owned()
						user="1000"
						global="9000"
					/>
				</div>
	*/
			</div>
		}
}

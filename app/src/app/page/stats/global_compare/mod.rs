mod bubble;
use bubble::StatsBubble;

use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn GlobalCompare(user_uuid: Uuid) -> impl IntoView {
	let res = Resource::new(
		move || user_uuid,
		move |user_uuid| crate::schema::stats::query::global_compare(user_uuid),
	);

	view! {
		<Transition>
			{move || {
				(match res.get() {
					Some(Ok(Some(stats))) => Some(stats),
					_ => None,
				})
					.map(|stats| {
						view! {
							<div class="stats_global_compare">
								<div class="stats_row">

									<StatsBubble
										title="wpm avg".to_owned()
										user=stats.user.wpm.average.to_string()
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
					})
			}}
		</Transition>
	}
}

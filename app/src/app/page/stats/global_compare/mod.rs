mod bubble;
use bubble::StatsBubble;

use leptos::prelude::*;
use uuid::Uuid;

use crate::app::dynamic_style;

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
										user=view! {
											<span style=dynamic_style::wpm::gradient(
												stats.user.wpm.average,
											)>{format!("{:.1}", stats.user.wpm.average)}</span>
										}
										global=view! {
											<span style=dynamic_style::wpm::gradient(
												stats.global.wpm.average,
											)>{format!("{:.1}", stats.global.wpm.average)}</span>
										}
									/>

									<StatsBubble
										title="wpm med".to_owned()
										user=view! {
											<span style=dynamic_style::wpm::gradient(
												stats.user.wpm.median,
											)>{format!("{:.1}", stats.user.wpm.median)}</span>
										}
										global=view! {
											<span style=dynamic_style::wpm::gradient(
												stats.global.wpm.median,
											)>{format!("{:.1}", stats.global.wpm.median)}</span>
										}
									/>
								</div>

							</div>
						}
					})
			}}
		</Transition>
	}
}

mod bubble;
use bubble::StatsBubble;

use leptos::prelude::*;
use uuid::Uuid;

use crate::app::dynamic_style;

#[component]
pub fn GlobalCompare(user_uuid: Uuid) -> impl IntoView {
	let res = Resource::new(
		move || user_uuid,
		crate::schema::stats::query::global_compare,
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

								<div class="stats_row">
									<StatsBubble
										title="acc avg".to_owned()
										user=view! {
											<span style=dynamic_style::accuracy::gradient(
												stats.user.accuracy.average,
											)>{format!("{:.2}%", stats.user.accuracy.average)}</span>
										}
										global=view! {
											<span style=dynamic_style::accuracy::gradient(
												stats.global.accuracy.average,
											)>{format!("{:.2}%", stats.global.accuracy.average)}</span>
										}
									/>

									<StatsBubble
										title="acc med".to_owned()
										user=view! {
											<span style=dynamic_style::accuracy::gradient(
												stats.user.accuracy.median,
											)>{format!("{:.2}%", stats.user.accuracy.median)}</span>
										}
										global=view! {
											<span style=dynamic_style::accuracy::gradient(
												stats.global.accuracy.median,
											)>{format!("{:.2}%", stats.global.accuracy.median)}</span>
										}
									/>
								</div>

								<div class="stats_row">
									<StatsBubble
										title="key sum".to_owned()
										user=view! {
											// TODO: locale format
											<span>{format!("{}", stats.user.key_sum)}</span>
										}
										global=view! {
											<span>{format!("{}", stats.global.key_sum)}</span>
										}
									/>

									<StatsBubble
										title="time sum".to_owned()
										user=view! {
											// TODO: locale format
											<span>{format!("{}", stats.user.time_sum)}</span>
										}
										global=view! {
											<span>{format!("{}", stats.global.time_sum)}</span>
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

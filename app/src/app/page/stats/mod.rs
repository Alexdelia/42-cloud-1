mod history;
mod list;

use crate::{app::page, schema::stats::Stats};
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use uuid::Uuid;

#[derive(Params, PartialEq)]
struct UserParams {
	user_uuid: Option<Uuid>,
}

#[component]
pub fn Stats() -> impl IntoView {
	let params = use_params::<UserParams>();
	let user_uuid = move || {
		params
			.read()
			.as_ref()
			.ok()
			.and_then(|params| params.user_uuid)
	};

	view! {
		<div class="page_stats">
			<Transition fallback=move || {
				view! { <p>"[Transition] getting stats..."</p> }
			}>
				<Show
					when=move || {
						user_uuid().is_some()
							&& stats_list.get().map(|s| s.is_ok()).unwrap_or(false)
					}
					fallback=page::NotFound
				>
					<div class="page_stats_loaded">
						<history::History user_uuid=move || {
							user_uuid.get().unwrap()
						} />

						<list::List rows=stats_list />
					</div>
				</Show>
			</Transition>
		</div>
	}
}

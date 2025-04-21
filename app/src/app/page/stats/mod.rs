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

	let stats_list = Resource::new(
		move || user_uuid(),
		move |user_uuid| crate::schema::stats::query::list(user_uuid.unwrap_or_default()),
	);

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
					<div>{user_uuid().unwrap().to_string()}</div>
					<list::List rows=stats_list />
				</Show>
			</Transition>
		</div>
	}
}

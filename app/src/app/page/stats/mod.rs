mod global_compare;
mod history;

use crate::app::page;
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
				view! { <span class="loader" /> }
			}>
				<Show
					when=move || { user_uuid().is_some() }
					fallback=page::NotFound
				>
					<div class="page_stats_loaded">
						{move || {
							user_uuid()
								.map(|user_uuid| {
									view! {
										<global_compare::GlobalCompare user_uuid=user_uuid />
										<history::History user_uuid=user_uuid />
									}
								})
						}}
					</div>
				</Show>
			</Transition>
		</div>
	}
}

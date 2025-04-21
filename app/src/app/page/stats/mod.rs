use crate::schema::stats::Stats;
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
			.unwrap_or_default()
	};

	let stats_list = Resource::new(
		|| {},
		move |_| crate::schema::stats::query::list(user_uuid()),
	);

	view! {
		<div class="page_stats">
			<Transition fallback=move || {
				view! { <p>"[Transition] getting stats..."</p> }
			}>
				<Show
					when=move || {
						stats_list.get().map(|s| s.is_ok()).unwrap_or(false)
					}
					fallback=|| {
						view! { <p>"[Show] getting stats..."</p> }
					}
				>
					<div>{user_uuid().to_string()}</div>
					<pre>{stats_list.get().unwrap().unwrap().len()}</pre>
				</Show>
			</Transition>
		</div>
	}
}

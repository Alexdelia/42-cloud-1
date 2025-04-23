use leptos::prelude::*;
use leptos_router::components::A;
use uuid::Uuid;

#[component]
pub fn Stats(user_uuid: Signal<Option<Uuid>>) -> impl IntoView {
	let (uuid, set_uuid) = signal(None);

	// wrapping in Effect because user_uuid is in local storage
	// and can't be accessed during server rendering
	// https://book.leptos.dev/ssr/24_hydration_bugs.html
	Effect::new(move |_| {
		if let Some(uuid) = user_uuid.get() {
			set_uuid.set(Some(uuid));
		}
	});

	view! {
		<Show when=move || { uuid.get().is_some() } fallback=|| ()>
			<div class="stats_redirect_btn">
				<A href=move || format!("/stats/{}", uuid.get().unwrap())>
					<button class="redirect_btn">"hello"</button>
				</A>
			</div>
		</Show>
	}
}

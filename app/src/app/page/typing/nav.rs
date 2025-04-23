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
					// https://fonts.google.com/icons?selected=Material+Symbols+Rounded:bar_chart:FILL@0;wght@400;GRAD@0;opsz@24&icon.query=chart&icon.size=24&icon.color=%23e3e3e3&icon.style=Rounded
					<svg
						xmlns="http://www.w3.org/2000/svg"
						height="3rem"
						width="3rem"
						viewBox="0 -960 960 960"
						fill="#fff"
					>
						<path d="M680-160q-17 0-28.5-11.5T640-200v-200q0-17 11.5-28.5T680-440h80q17 0 28.5 11.5T800-400v200q0 17-11.5 28.5T760-160h-80Zm-240 0q-17 0-28.5-11.5T400-200v-560q0-17 11.5-28.5T440-800h80q17 0 28.5 11.5T560-760v560q0 17-11.5 28.5T520-160h-80Zm-240 0q-17 0-28.5-11.5T160-200v-360q0-17 11.5-28.5T200-600h80q17 0 28.5 11.5T320-560v360q0 17-11.5 28.5T280-160h-80Z" />
					</svg>
				</A>
			</div>
		</Show>
	}
}

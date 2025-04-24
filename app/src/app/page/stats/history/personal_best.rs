use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn PersonalBest(user_uuid: Uuid) -> impl IntoView {
	let pb = Resource::new(
		move || user_uuid,
		move |user_uuid| crate::schema::stats::query::personal_best(user_uuid),
	);

	view! {
		<Transition fallback=move || {
			view! { <p>"Loading initial data..."</p> }
		}>
			<Show
				when=move || {
					pb.get()
						.map(|pb| pb.map(|pb| pb.is_some()).unwrap_or(false))
						.unwrap_or(false)
				}
				fallback=move || ()
			>
				<super::row::Row row=pb.get().unwrap().unwrap().unwrap() />
			</Show>
		</Transition>
	}
}

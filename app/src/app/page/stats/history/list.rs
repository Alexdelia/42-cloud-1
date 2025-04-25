use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn List(user_uuid: Uuid) -> impl IntoView {
	let rows = Resource::new(
		move || user_uuid,
		move |user_uuid| crate::schema::stats::query::list(user_uuid),
	);

	view! {
		<Transition>
			{move || {
				(match rows.get() {
					Some(Ok(rows)) => Some(rows),
					_ => None,
				})
					.map(|rows| {
						rows.into_iter()
							.map(|row| view! { <super::row::Row row=row /> })
							.collect_view()
					})
			}}
		</Transition>
	}
}

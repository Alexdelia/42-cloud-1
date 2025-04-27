use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn PersonalBest(user_uuid: Uuid) -> impl IntoView {
	let pb = Resource::new(
		move || user_uuid,
		crate::schema::stats::query::personal_best,
	);

	view! {
		<Transition>
			{move || {
				(match pb.get() {
					Some(Ok(Some(row))) => Some(row),
					_ => None,
				})
					.map(|row| {
						view! { <super::row::Row row=row /> }
					})
			}}
		</Transition>
	}
}

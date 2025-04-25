use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn PersonalBest(user_uuid: Uuid) -> impl IntoView {
	/*
	let pb = Resource::new(
		move || user_uuid,
		move |user_uuid| crate::schema::stats::query::personal_best(user_uuid),
	);

	view! {
		<super::row::Row row=Default::default() />

		<Transition fallback=move || {
			view! { <p>"Loading initial data..."</p> }
		}>
			<Show
				when=move || { match pb.get() {
					Some(Ok(Some(_))) => true,
					_ => false,
				}}
				fallback=move || {
					view! { <p>"No personal best yet..."</p> }
				}
			>

			{move || {
				(match pb.get() {
					Some(Ok(Some(row))) => Some(row),
					_ => None,
				})
					.map(|row| {
						view! { <super::row::Row row=row /> }
					})
			}}
			</Show>
		</Transition>
	}
	*/

	view! {
		<super::row::Row row=Default::default() />
	}
}

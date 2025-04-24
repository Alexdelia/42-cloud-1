use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn PersonalBest(user_uuid: Uuid) -> impl IntoView {
	let pb = LocalResource::new(move || crate::schema::stats::query::personal_best(user_uuid));

	view! {
		<Transition fallback=move || {
			view! { <p>"Loading initial data..."</p> }
		}>
			<p>
				{move || {
					pb
						.read()
						.as_deref()
						.map(|pb| {
							match pb {
								Ok(pb) => {
									if let Some(pb) = pb {
										format!("Your personal best is: {}", pb.wpm)
									} else {
										"Personal best not found".to_string()
									}
								}
								Err(e) => format!("Error fetching personal best: {}", e),
							}
						})
				}}
			</p>
		</Transition>
	}
}

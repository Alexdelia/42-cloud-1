mod page;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title};
use leptos_router::{
	StaticSegment,
	components::{Route, Router, Routes},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="utf-8" />
				<meta
					name="viewport"
					content="width=device-width, initial-scale=1"
				/>
				<AutoReload options=options.clone() />
				<HydrationScripts options />
				<MetaTags />
			</head>
			<body>
				<App />
			</body>
		</html>
	}
}

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	leptos_meta::provide_meta_context();

	view! {
		<Stylesheet id="leptos" href="/pkg/cloud-1.css" />

		<Title text="Tust" />

		<Router>
			<main>
				// TODO: add a nice 404 page
				<Routes fallback=|| "Page not found.".into_view()>
					<Route path=StaticSegment("") view=page::Typing />
				</Routes>
			</main>
		</Router>
	}
}

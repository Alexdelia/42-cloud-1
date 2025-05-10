pub mod dynamic_style;
mod page;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title};
use leptos_router::{
	StaticSegment,
	components::{Route, Router, Routes},
	path,
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
			<body style="background-color: hsl(0, 0%, 20%);">
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
				<Routes fallback=page::NotFound>
					<Route path=StaticSegment("") view=page::Typing />
					<Route path=path!("stats/:user_uuid") view=page::Stats />
				</Routes>
			</main>
		</Router>
	}
}

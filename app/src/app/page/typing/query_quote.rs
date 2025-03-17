use leptos::prelude::*;
use serde::Deserialize;

use super::{Anime, Quote};

const URL: &str = "https://animechan.io/api/v1/quotes/random";

#[derive(Deserialize, Debug)]
struct QuoteResponse {
	data: Quote,
}

#[server]
pub async fn query_quote() -> Result<Quote, ServerFnError> {
	let res = reqwest::get(URL).await;

	let default = Ok(Quote {
        text: "If this is all it takes to tear us apart, then maybe we weren't all that close to begin with.".to_string().replace(" ", "_"),
		character: "Yukino Yukinoshita".to_string(),
		anime: Anime {
			name: "Oregairu".to_string(),
			alt_name: "My Teen Romantic Comedy SNAFU".to_string(),
		},
	});

	let Ok(res) = res else {
		leptos::logging::log!("error fetching quote:\n{res:#?}");
		return default;
	};

	let res = res.json::<QuoteResponse>().await;
	dbg!(&res);

	let Ok(res) = res else {
		leptos::logging::log!("error parsing quote:\n{res:#?}");
		return default;
	};

	Ok(res.data)
}

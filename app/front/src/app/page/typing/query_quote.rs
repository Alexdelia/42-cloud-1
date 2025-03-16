use leptos::prelude::*;

use super::{Anime, Quote};

const URL: &str = "https://animechan.io/api/v1/quotes/random";

#[server]
pub async fn query_quote() -> Result<Quote, ServerFnError> {
	let res = reqwest::get(URL).await;

	leptos::logging::log!("res: {:?}", res);

	Quote {
		text: "yes".to_string(),
		character: "Yukino Yukinoshita".to_string(),
		anime: Anime {
			name: "Oregairu".to_string(),
			alt_name: "My Teen Romantic Comedy SNAFU".to_string(),
		},
	}
}

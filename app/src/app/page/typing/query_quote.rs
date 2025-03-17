use leptos::prelude::*;

#[cfg(feature = "ssr")]
use serde::Deserialize;

#[cfg(feature = "ssr")]
use crate::schema::quote::Anime;

use crate::schema::quote::Quote;

#[cfg(feature = "ssr")]
const URL: &str = "https://animechan.io/api/v1/quotes/random";

#[cfg(feature = "ssr")]
#[derive(Default, Deserialize, Debug, Clone)]
struct QuoteResponse {
	data: Option<Quote>,
	message: Option<String>,
}

#[cfg(feature = "ssr")]
impl Into<Quote> for QuoteResponse {
	fn into(self) -> Quote {
		let Some(quote) = self.data else {
			return Quote {
                text: "If this is all it takes to tear us apart, then maybe we weren't all that close to begin with.".to_string().replace(" ", "_"),
                character: "Yukino Yukinoshita".to_string(),
                anime: Anime {
                    name: "Oregairu".to_string(),
                    alt_name: "My Teen Romantic Comedy SNAFU".to_string(),
                },
            };
		};

		Quote {
			text: quote.text.replace(" ", "_"),
			..quote
		}
	}
}

#[server]
pub async fn query_quote() -> Result<Quote, ServerFnError> {
	let res = reqwest::get(URL).await;

	let Ok(res) = res else {
		leptos::logging::log!("error fetching quote:\n{res:#?}");
		return Ok(QuoteResponse::default().into());
	};

	let res = res.json::<QuoteResponse>().await;
	dbg!(&res);

	let Ok(res) = res else {
		leptos::logging::log!("error parsing quote:\n{res:#?}",);
		return Ok(QuoteResponse::default().into());
	};

	if res.data.is_none() {
		leptos::logging::log!(
			"error parsing inner quote: '{message}'\n{res:#?}",
			message = res.clone().message.unwrap_or_else(|| "None".to_string()),
		);
		return Ok(QuoteResponse::default().into());
	};

	Ok(res.into())
}

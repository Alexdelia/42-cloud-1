use super::{Anime, Quote};

pub fn query_quote() -> Quote {
	let text = "If this is all it takes to tear us apart, then maybe we weren't all that close to begin with."
		.to_string()
		.replace(" ", "_");

	Quote {
		text,
		character: "Yukino Yukinoshita".to_string(),
		anime: Anime {
			name: "Oregairu".to_string(),
			alt_name: "My Teen Romantic Comedy SNAFU".to_string(),
		},
	}
}

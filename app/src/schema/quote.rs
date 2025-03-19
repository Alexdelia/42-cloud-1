use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Quote {
	pub text: String,

	pub character: String,
	pub anime: Anime,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Anime {
	pub name: String,
	#[serde(rename = "altName")]
	pub alt_name: String,
}

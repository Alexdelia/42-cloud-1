mod computed;
// #[cfg(feature = "ssr")]
// mod query;

use chrono::{DateTime, Utc};
#[cfg(feature = "ssr")]
use sqlx::prelude::FromRow;

#[derive(Clone)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Stats {
	pub start_time: DateTime<Utc>,
	pub end_time: DateTime<Utc>,

	pub correct_key: usize,
	pub wrong_key: usize,
}

impl Default for Stats {
	fn default() -> Self {
		Self {
			start_time: Utc::now(),
			end_time: Utc::now(),

			correct_key: 0,
			wrong_key: 0,
		}
	}
}

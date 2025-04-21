mod computed;
pub mod query;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
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

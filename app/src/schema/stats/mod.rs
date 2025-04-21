mod computed;
#[cfg(feature = "ssr")]
mod query;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::prelude::FromRow;

#[derive(Clone, Serialize, Deserialize, Debug)]
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

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {

use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct StatsRow {
	pub uuid: Uuid,
	pub user_uuid: Uuid,

	pub start_time: DateTime<Utc>,
	pub end_time: DateTime<Utc>,

	pub correct_key: i32,
	pub wrong_key: i32,

	pub created_at: DateTime<Utc>,
}

impl From<StatsRow> for Stats {
	fn from(row: StatsRow) -> Self {
		Self {
			start_time: row.start_time,
			end_time: row.end_time,

			correct_key: row.correct_key as usize,
			wrong_key: row.wrong_key as usize,
		}
	}
}
}
}

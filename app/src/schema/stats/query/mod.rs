mod store;
pub use store::store;
mod personal_best;
pub use personal_best::personal_best;
mod list;
pub use list::list;
mod global_compare;
pub use global_compare::{StatsGlobalCompare, global_compare};

#[cfg(feature = "ssr")]
mod raw;

#[cfg(feature = "ssr")]
use const_format::formatcp;

use super::Stats;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use sqlx::prelude::FromRow;

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

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ComputedStatsRow {
	pub timestamp: DateTime<Utc>,

	pub wpm: f64,
	pub accuracy: f64,

	pub duration_seconds: f32,
}

impl ComputedStatsRow {
	#[cfg(feature = "ssr")]
	const RAW_SELECT: &str = formatcp!(
		"\
end_time as timestamp,\
{wpm} as wpm,\
{accuracy} as accuracy,\
{duration} as duration_seconds",
		wpm = raw::WPM,
		accuracy = raw::ACCURACY,
		duration = raw::DURATION_SECONDS,
	);
}

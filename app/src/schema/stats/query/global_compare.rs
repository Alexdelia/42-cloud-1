use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use super::raw::*;
#[cfg(feature = "ssr")]
use const_format::formatcp;
#[cfg(feature = "ssr")]
use sqlx::prelude::FromRow;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StatsGlobalCompare {
	pub user: StatsCompareResult,
	pub global: StatsCompareResult,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StatsCompareResult {
	pub wpm: StatsAvgMed,
	pub accuracy: StatsAvgMed,
	pub key_sum: i64,
	pub time_sum: f32,
	pub row_count: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StatsAvgMed {
	pub average: f64,
	pub median: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
struct StatsGlobalCompareRow {
	user_wpm_average: f64,
	user_wpm_median: f64,
	user_accuracy_average: f64,
	user_accuracy_median: f64,
	user_key_sum: i64,
	user_time_sum: f32,
	user_row_count: i64,
	global_wpm_average: f64,
	global_wpm_median: f64,
	global_accuracy_average: f64,
	global_accuracy_median: f64,
	global_key_sum: i64,
	global_time_sum: f32,
	global_row_count: i64,
}

impl From<StatsGlobalCompareRow> for StatsGlobalCompare {
	fn from(row: StatsGlobalCompareRow) -> Self {
		Self {
			user: StatsCompareResult {
				wpm: StatsAvgMed {
					average: row.user_wpm_average,
					median: row.user_wpm_median,
				},
				accuracy: StatsAvgMed {
					average: row.user_accuracy_average,
					median: row.user_accuracy_median,
				},
				key_sum: row.user_key_sum,
				time_sum: row.user_time_sum,
				row_count: row.user_row_count,
			},
			global: StatsCompareResult {
				wpm: StatsAvgMed {
					average: row.global_wpm_average,
					median: row.global_wpm_median,
				},
				accuracy: StatsAvgMed {
					average: row.global_accuracy_average,
					median: row.global_accuracy_median,
				},
				key_sum: row.global_key_sum,
				time_sum: row.global_time_sum,
				row_count: row.global_row_count,
			},
		}
	}
}

#[cfg(feature = "ssr")]
const SUBQUERY_SELECT: &str = formatcp!(
	"{KEY_COUNT} AS key_count,
{DURATION_SECONDS} AS duration_seconds,
{WPM} AS wpm,
{ACCURACY} AS accuracy"
);

#[server]
pub async fn global_compare(user_uuid: Uuid) -> Result<Option<StatsGlobalCompare>, ServerFnError> {
	let pool = use_context::<sqlx::PgPool>().ok_or_else::<ServerFnError, _>(|| {
		ServerFnError::ServerError(String::from("no postgres connection pool"))
	})?;

	let res = sqlx::query_as::<_, StatsGlobalCompareRow>(formatcp!(
		"\
WITH
    user_stats AS (
        SELECT {SUBQUERY_SELECT}
        FROM stats
        WHERE user_uuid = $1
    ),
    global_stats AS (
        SELECT {SUBQUERY_SELECT}
        FROM stats
    )
SELECT
    (SELECT AVG(wpm) FROM user_stats)
        AS user_wpm_average,
    (SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY wpm) FROM user_stats)
        AS user_wpm_median,
    (SELECT AVG(accuracy) FROM user_stats)
        AS user_accuracy_average,
    (SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY accuracy) FROM user_stats)
        AS user_accuracy_median,
    (SELECT SUM(key_count) FROM user_stats)
        AS user_key_sum,
    (SELECT SUM(duration_seconds) FROM user_stats)
        AS user_time_sum,
    (SELECT COUNT(*) FROM user_stats)
        AS user_row_count,
    (SELECT AVG(wpm) FROM global_stats)
        AS global_wpm_average,
    (SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY wpm) FROM global_stats)
        AS global_wpm_median,
    (SELECT AVG(accuracy) FROM global_stats)
        AS global_accuracy_average,
    (SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY accuracy) FROM global_stats)
        AS global_accuracy_median,
    (SELECT SUM(key_count) FROM global_stats)
        AS global_key_sum,
    (SELECT SUM(duration_seconds) FROM global_stats)
        AS global_time_sum,
    (SELECT COUNT(*) FROM global_stats)
        AS global_row_count"
	))
	.bind(user_uuid)
	.fetch_optional(&pool)
	.await
	.map_err::<ServerFnError, _>(|e| {
		dbg!(e.to_string());
		ServerFnError::Deserialization(e.to_string())
	})?;

	Ok(res.map(|r| r.into()))
}

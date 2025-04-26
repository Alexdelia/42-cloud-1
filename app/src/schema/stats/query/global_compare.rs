use super::ComputedStatsRow;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use const_format::formatcp;
#[cfg(feature = "ssr")]
use sqlx::prelude::FromRow;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct StatsGlobalCompare {
	user: StatsCompareResult,
	global: StatsCompareResult,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct StatsCompareResult {
	wpm: StatsAvgMed,
	accuracy: StatsAvgMed,
	key_sum: usize,
	time_sum: usize,
	row_count: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct StatsAvgMed {
	average: f64,
	median: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
#[server]
pub async fn global_compare(user_uuid: Uuid) -> Result<Vec<ComputedStatsRow>, ServerFnError> {
	let pool = use_context::<sqlx::PgPool>().ok_or_else::<ServerFnError, _>(|| {
		ServerFnError::ServerError(String::from("no postgres connection pool"))
	})?;

	let res = sqlx::query_as::<_, ComputedStatsRow>(formatcp!(
		"\
SELECT {select}
FROM stats
WHERE user_uuid = $1
ORDER BY end_time DESC",
		select = ComputedStatsRow::RAW_SELECT,
	))
	.bind(user_uuid)
	.fetch_all(&pool)
	.await
	.map_err::<ServerFnError, _>(|e| ServerFnError::Deserialization(e.to_string()))?;

	Ok(res)
}

use super::ComputedStatsRow;
use leptos::prelude::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use const_format::formatcp;

#[server]
pub async fn list(user_uuid: Uuid) -> Result<Vec<ComputedStatsRow>, ServerFnError> {
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

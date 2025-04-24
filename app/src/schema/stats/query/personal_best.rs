use super::ComputedStatsRow;
use leptos::prelude::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use super::raw;
#[cfg(feature = "ssr")]
use const_format::formatcp;

#[server]
pub async fn personal_best(user_uuid: Uuid) -> Result<Option<ComputedStatsRow>, ServerFnError> {
	let pool = use_context::<sqlx::PgPool>().ok_or_else::<ServerFnError, _>(|| {
		ServerFnError::ServerError(String::from("no postgres connection pool"))
	})?;

	let res = sqlx::query_as::<_, ComputedStatsRow>(formatcp!(
		"\
SELECT {select}
FROM stats
WHERE user_uuid = $1
ORDER BY {wpm} DESC",
		select = ComputedStatsRow::RAW_SELECT,
		wpm = raw::WPM,
	))
	.bind(user_uuid)
	.fetch_optional(&pool)
	.await
	.map_err::<ServerFnError, _>(|e| ServerFnError::Deserialization(e.to_string()))?;

	Ok(res)
}

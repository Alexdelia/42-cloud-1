use super::{Stats, StatsRow};
use leptos::prelude::*;

#[server]
pub async fn list() -> Result<Vec<Stats>, ServerFnError> {
	let pool = use_context::<sqlx::PgPool>().ok_or_else::<ServerFnError, _>(|| {
		ServerFnError::ServerError(String::from("no postgres connection pool"))
	})?;

	let stats = sqlx::query_as!(StatsRow, "SELECT * FROM stats")
		.fetch_all(&pool)
		.await
		.map_err::<ServerFnError, _>(|e| ServerFnError::Deserialization(e.to_string()))?;
	let stats = stats.into_iter().map(|row| row.into()).collect();

	Ok(stats)
}

/*
#[server]
pub async fn store(cx: Scope, stats: Stats) -> Result<(), ServerFnError> {
	let pool = cx.get::<sqlx::PgPool>().await?;
	sqlx::query(
		"INSERT INTO stats (start_time, end_time, correct_key, wrong_key) VALUES ($1, $2, $3, $4)",
	)
	.bind((
		stats.start_time,
		stats.end_time,
		stats.correct_key,
		stats.wrong_key,
	))
	.execute(&pool)
	.await?;

	Ok(())
}
*/

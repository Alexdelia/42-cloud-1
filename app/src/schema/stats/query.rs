use super::Stats;
use leptos::attr::Scope;
use leptos::prelude::*;

#[server]
pub async fn list(cx: Scope) -> Result<Vec<Stats>, ServerFnError> {
	let pool = use_context::<sqlx::PgPool>()
		.ok_or_else(|| ServerFnError::ServerError(String::from("no postgres connection pool")))?;

	let stats = sqlx::query!("SELECT * FROM stats")
		.fetch_all(&pool)
		.await
		.map_err(|e| ServerFnError::ServerError(e.to_string()))?;

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

use super::Stats;
use leptos::prelude::*;
use uuid::Uuid;

#[server]
pub async fn store(user_uuid: Uuid, stats: Stats) -> Result<(), ServerFnError> {
	let pool = use_context::<sqlx::PgPool>().ok_or_else::<ServerFnError, _>(|| {
		ServerFnError::ServerError(String::from("no postgres connection pool"))
	})?;

	sqlx::query!(
		"\
INSERT INTO
	stats
	(
		user_uuid,
		start_time,
		end_time,
		correct_key,
		wrong_key
	)
VALUES ($1, $2, $3, $4, $5)",
		user_uuid,
		stats.start_time,
		stats.end_time,
		stats.correct_key as i32,
		stats.wrong_key as i32,
	)
	.execute(&pool)
	.await?;

	Ok(())
}

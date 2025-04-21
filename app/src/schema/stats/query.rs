use super::Stats;
use chrono::{DateTime, Utc};
use leptos::prelude::*;
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

#[server]
pub async fn store(user_uuid: Uuid, stats: Stats) -> Result<(), ServerFnError> {
	let pool = use_context::<sqlx::PgPool>().ok_or_else::<ServerFnError, _>(|| {
		ServerFnError::ServerError(String::from("no postgres connection pool"))
	})?;

	sqlx::query!(
        "INSERT INTO stats (user_uuid, start_time, end_time, correct_key, wrong_key) VALUES ($1, $2, $3, $4, $5)",
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

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {

#[macro_use]
extern crate dotenv_codegen;

use axum::{
	body::Body as AxumBody,
	extract::{FromRef, Path, State},
	http::Request,
	response::{IntoResponse, Response},
	routing::get,
	Router,
};
use cloud_1::app::*;
use leptos::prelude::*;
use leptos_axum::handle_server_fns_with_context;
use leptos::logging::log;

#[derive(FromRef, Clone)]
pub struct AppState {
	pub leptos_options: LeptosOptions,
	pub pool: sqlx::postgres::PgPool,
	// pub client: reqwest::Client,
}

impl AppState {
	pub async fn new() -> Self {
		let pool = sqlx::postgres::PgPoolOptions::new()
			.max_connections(8)
			.connect(const_format::formatcp!(
				"postgres://{username}:{password}@localhost:{port}/{dbname}",
				username = dotenv!("PROJECT_NAME"),
				password = dotenv!("DB_PASSWORD"),
				port = dotenv!("DB_PORT"),
				dbname = dotenv!("PROJECT_NAME"),
			))
			.await
			.expect("failed to connect to postgres and create pool");

		Self {
			leptos_options: get_configuration(None).expect("failed to get configuration").leptos_options,
			pool,
			// client: reqwest::Client::new(),
		}
	}
}

async fn server_fn_handler(
	State(state): State<AppState>,
	path: Path<String>,
	request: Request<AxumBody>,
) -> impl IntoResponse {
	log!("{:?}", path);

	handle_server_fns_with_context(
		move || {
			provide_context(state.pool.clone());
		},
		request,
	)
	.await
}

async fn leptos_routes_handler(
	State(state): State<AppState>,
	req: Request<AxumBody>,
) -> Response {
	let handler = leptos_axum::render_app_to_stream_with_context(
		move || {
			provide_context(state.pool.clone());
		},
		move || shell(state.leptos_options.clone())
	);
	handler(req).await.into_response()
}

#[tokio::main]
async fn main() {
	use leptos_axum::LeptosRoutes;

	let state = AppState::new().await;

	sqlx::migrate!()
		.run(&state.pool)
		.await
		.expect("failed to run migrations");

	let addr = state.leptos_options.site_addr;
	let routes = leptos_axum::generate_route_list(App);

	let app = Router::new()
		.route(
			"/api/{*fn_name}",
			get(server_fn_handler).post(server_fn_handler),
		)
		.leptos_routes_with_handler(routes, get(leptos_routes_handler))
		.fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
		.with_state(state);

	let listener = tokio::net::TcpListener::bind(&addr).await.expect("failed to bind to address");
	log!("🟢 http://{}", &addr);
	axum::serve(listener, app.into_make_service())
		.await
		.expect("failed to start server");
}

} else {
pub fn main() {
	// no client-side main function
	// unless we want this to work with e.g., Trunk for pure client-side testing
	// see lib.rs for hydration function instead
}
}
}

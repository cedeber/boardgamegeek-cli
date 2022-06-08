use async_graphql::{
	http::{playground_source, GraphQLPlaygroundConfig},
	EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
	extract::Extension,
	http::{HeaderValue, Method, StatusCode},
	response::{self, IntoResponse},
	routing::{get, post},
	Router, Server,
};
use serde::Serialize;
use sqlx::{query_as, SqlitePool};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub async fn run() {
	// initialize tracing
	tracing_subscriber::fmt::init();

	let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);

	let backend = tokio::spawn(async {
		let allowed_methods = vec![Method::GET, Method::POST];

		// TODO: .nest() for /api + CORS
		let app = Router::new()
			.route("/hello", post(hello))
			.route("/graphql", get(graphql_playground).post(graphql_handler))
			.layer(Extension(schema))
			.layer(
				// see https://docs.rs/tower-http/latest/tower_http/cors/index.html
				// for more details
				CorsLayer::new()
					.allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
					.allow_methods(allowed_methods)
					.allow_headers(Any),
			);

		serve(app, 4000).await
	});

	tokio::join!(backend).0.unwrap();
}

/// Run the app with hyper.
// `axum::Server` is a re-export of `hyper::Server`
async fn serve(app: Router, port: u16) {
	let addr = SocketAddr::from(([127, 0, 0, 1], port));
	info!("listening on {}", addr);
	Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn hello() -> impl IntoResponse {
	(
		StatusCode::OK,
		[("X-Foo", "foo")],
		String::from("Hello, World!"),
	)
}

#[derive(Debug, Clone, Serialize, SimpleObject)]
pub struct BoardGameResult {
	pub id: i64,
	pub name: String,
	pub year: Option<i64>,
	pub min_players: Option<i64>,
	pub max_players: Option<i64>,
	pub playtime: Option<i64>,
}

#[derive(Default)]
struct Query;

#[Object]
impl Query {
	/// Returns the sum of a and b
	async fn add(&self, a: i32, b: i32) -> i32 {
		a + b
	}

	async fn games(&self, limit: Option<i32>) -> Vec<BoardGameResult> {
		let pool = SqlitePool::connect("sqlite:games.sqlite").await.unwrap();
		let limit = limit.unwrap_or(-1);

		let result = query_as!(
			BoardGameResult,
			"SELECT gameid as id, title as name, published as year, playing_time as playtime, min_players, max_players FROM boardgames ORDER BY title LIMIT ?",
			limit
		)
		.fetch_all(&pool)
		.await
		.unwrap();

		result
	}
}

type QuerySchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<QuerySchema>, req: GraphQLRequest) -> GraphQLResponse {
	schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
	response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

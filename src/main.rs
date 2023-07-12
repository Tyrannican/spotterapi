mod api_types;
mod configuration;
mod routes;

use configuration::get_configuration;
use routes::get;
use routes::post;

use axum::{
    http::header::CONTENT_TYPE,
    routing::{get, post},
    Extension, Router, Server,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() {
    let app_settings = get_configuration().expect("failed to load configuration");
    let db_url = app_settings.database.connection_url();
    let db_pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("unable to connect to postgres");

    let state = AppState {};

    let router = Router::new()
        .route("/", get(get::root))
        .route("/status", get(get::status))
        .route("/api/sightings", get(get::sightings))
        .route("/api/sightings", post(post::sighting))
        .layer(Extension(db_pool))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
        .with_state(state);

    let server_addr = format!("0.0.0.0:{}", app_settings.application_port);
    let server = Server::bind(&server_addr.parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

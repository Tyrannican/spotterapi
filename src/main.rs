mod configuration;

use configuration::get_configuration;

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sighting {
    user_id: String,
    lat: f32,
    lng: f32,
    object: String,
    description: String,
}

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
        .route("/", get(root_get))
        .route("/status", get(get_status))
        .route("/api/sightings", get(get_sightings))
        .route("/api/sightings", post(post_sighting))
        .layer(Extension(db_pool))
        .with_state(state);

    let server_addr = format!("0.0.0.0:{}", app_settings.application_port);
    let server = Server::bind(&server_addr.parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

async fn root_get() -> impl IntoResponse {
    Json("Spotter API")
}

async fn get_sightings(State(state): State<AppState>) -> impl IntoResponse {
    let response = "I am a sighting";
    Json(response)
}

async fn post_sighting(
    Extension(connection): Extension<PgPool>,
    Json(payload): Json<Sighting>,
) -> impl IntoResponse {
    let Sighting {
        user_id,
        lat,
        lng,
        object,
        description,
    } = payload;

    let query = "SELECT * FROM sightings;";
    let response = sqlx::query(&query)
        .execute(&connection)
        .await
        .expect("error");

    println!("{:?}", response);

    Json("Sighting added successfully")
}

async fn get_status() -> impl IntoResponse {
    Json("OK")
}

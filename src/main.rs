mod api_types;
mod configuration;

use api_types::Sighting;
use configuration::get_configuration;

use axum::{
    http::header::CONTENT_TYPE,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router, Server,
};
use chrono::Utc;
use sqlx::postgres::{PgPool, PgPoolOptions};
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
        .route("/", get(root_get))
        .route("/status", get(get_status))
        .route("/api/sightings", get(get_sightings))
        .route("/api/sightings", post(post_sighting))
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

async fn root_get() -> impl IntoResponse {
    Json("Spotter API")
}

async fn get_sightings(Extension(connection): Extension<PgPool>) -> impl IntoResponse {
    let response = sqlx::query!(r#"SELECT * FROM sightings"#)
        .fetch_all(&connection)
        .await
        .expect("unable to execute query");

    let mut sightings: Vec<Sighting> = response
        .into_iter()
        .map(|r| Sighting {
            id: Some(r.id),
            user_id: r.user_id,
            lat: r.lat,
            lng: r.lng,
            object: r.object,
            description: r.description,
            created_at: Some(r.created_at),
        })
        .collect();

    sightings.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Json(sightings)
}

async fn post_sighting(
    Extension(connection): Extension<PgPool>,
    Json(mut payload): Json<Sighting>,
) -> impl IntoResponse {
    payload.id = Some(ulid::Ulid::new().to_string());
    payload.created_at = Some(Utc::now().timestamp());

    let Sighting {
        id,
        user_id,
        lat,
        lng,
        object,
        description,
        created_at,
    } = payload;

    sqlx::query!(
        r#"
        INSERT INTO "sightings"(id,user_id,lat,lng,object,description,created_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7)
    "#,
        id,
        user_id,
        lat,
        lng,
        object,
        description,
        created_at
    )
    .execute(&connection)
    .await
    .expect("error");

    println!("Added things: {}", user_id);

    Json("Sighting added successfully")
}

async fn get_status() -> impl IntoResponse {
    Json("OK")
}

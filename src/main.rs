use chrono::Utc;
use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Server,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sighting {
    id: Option<String>,
    user_id: String,
    lat: f32,
    lng: f32,
    object: String,
    description: String,
    timestamp: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SightingCollection {
    latest: Vec<Sighting>,
}

impl SightingCollection {
    pub fn new() -> Self {
        Self { latest: vec![] }
    }
}

#[derive(Clone)]
struct AppState {
    sightings: Arc<Mutex<SightingCollection>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        sightings: Arc::new(Mutex::new(SightingCollection::new())),
    };

    let router = Router::new()
        .route("/", get(root_get))
        .route("/status", get(get_status))
        .route("/api/sightings", get(get_sightings))
        .route("/api/sightings", post(post_sighting))
        .with_state(state);

    let server = Server::bind(&"0.0.0.0:9123".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

async fn root_get() -> impl IntoResponse {
    Json("Spotter API")
}

async fn get_sightings(State(state): State<AppState>) -> impl IntoResponse {
    let sightings = state.sightings.lock().unwrap();
    let mut response = sightings.latest.clone();
    response.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    Json(response)
}

async fn post_sighting(
    State(state): State<AppState>,
    Json(mut payload): Json<Sighting>,
) -> impl IntoResponse {
    let mut sightings = state.sightings.lock().unwrap();

    payload.id = Some(ulid::Ulid::new().to_string());
    payload.timestamp = Some(Utc::now().timestamp());
    sightings.latest.push(payload);

    Json("Sighting added successfully")
}

async fn get_status() -> impl IntoResponse {
    Json("OK")
}

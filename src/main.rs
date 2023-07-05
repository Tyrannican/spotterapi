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
pub struct Sightings {
    sightings: Vec<Sighting>,
}

impl Sightings {
    pub fn new() -> Self {
        Self { sightings: vec![] }
    }

    pub fn add(&mut self, sighting: Sighting) {
        self.sightings.push(sighting);
    }
}

#[derive(Clone)]
struct AppState {
    sightings: Arc<Mutex<Sightings>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        sightings: Arc::new(Mutex::new(Sightings::new())),
    };

    let router = Router::new()
        .route("/", get(root_get))
        .route("/status", get(get_status))
        .route("/api/sightings", get(get_sightings))
        .route("/api/sightings", post(post_sighting))
        .with_state(state);

    let server = Server::bind(&"0.0.0.0:9000".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

async fn root_get() -> &'static str {
    "Spotter API"
}

async fn get_sightings(State(state): State<AppState>) -> impl IntoResponse {
    let sightings = state.sightings.lock().unwrap();
    let response = sightings.clone();

    Json(response)
}

async fn post_sighting(
    State(state): State<AppState>,
    Json(mut payload): Json<Sighting>,
) -> impl IntoResponse {
    let mut state = state.sightings.lock().unwrap();

    payload.id = Some(ulid::Ulid::new().to_string());
    payload.timestamp = Some(Utc::now().timestamp());
    state.sightings.push(payload);

    Json("OK")
}

async fn get_status() -> &'static str {
    "OK"
}

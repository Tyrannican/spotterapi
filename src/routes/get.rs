use axum::{response::IntoResponse, Extension, Json};
use sqlx::PgPool;

use crate::api_types::{Thing, ThingType};

pub async fn root() -> impl IntoResponse {
    Json("Spotter API")
}

pub async fn status() -> impl IntoResponse {
    Json("OK")
}

pub async fn sightings(Extension(connection): Extension<PgPool>) -> impl IntoResponse {
    let response = sqlx::query!(r#"SELECT * FROM things"#)
        .fetch_all(&connection)
        .await
        .expect("unable to execute query");

    let sightings = response
        .into_iter()
        .map(|s| {
            let ts = if let Some(timestamp) = s.timestamp {
                timestamp.and_utc().to_rfc3339()
            } else {
                String::from("?")
            };

            Thing {
                name: s.name,
                thing: ThingType::from_id(s.thing_type),
                lat: s.lat,
                lng: s.lng,
                count: s.count,
                description: s.description,
                image: s.image,
                timestamp: ts,
            }
        })
        .collect::<Vec<Thing>>();

    Json(sightings)
}

pub async fn debug_clear_sightings(Extension(connection): Extension<PgPool>) -> impl IntoResponse {
    let _ = sqlx::query("DELETE FROM things")
        .execute(&connection)
        .await
        .expect("unable to execute debug deletion query");

    Json("Successfully cleared `things` table")
}

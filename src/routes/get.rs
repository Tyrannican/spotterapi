use axum::{response::IntoResponse, Extension, Json};
use sqlx::PgPool;

use crate::api_types::Sighting;

pub async fn root() -> impl IntoResponse {
    Json("Spotter API")
}

pub async fn status() -> impl IntoResponse {
    Json("OK")
}

pub async fn sightings(Extension(connection): Extension<PgPool>) -> impl IntoResponse {
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

pub async fn debug_clear_sightings(Extension(connection): Extension<PgPool>) -> impl IntoResponse {
    let _ = sqlx::query("DELETE FROM sightings")
        .execute(&connection)
        .await
        .expect("unable to execute debug deletion query");

    Json("Successfully cleared `sightings` table")
}

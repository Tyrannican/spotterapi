use axum::{response::IntoResponse, Extension, Json};
use chrono::Utc;
use sqlx::PgPool;

use crate::api_types::Sighting;

pub async fn sighting(
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

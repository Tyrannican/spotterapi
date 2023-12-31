use axum::{response::IntoResponse, Extension, Json};
use chrono::Utc;
use sqlx::PgPool;

use crate::api_types::Thing;

pub async fn sighting(
    Extension(connection): Extension<PgPool>,
    Json(payload): Json<Thing>,
) -> impl IntoResponse {
    let thing_id = Some(uuid::Uuid::new_v4());
    let thing_ts = Some(Utc::now().naive_utc());

    println!("Payload: {:?}", &payload);
    let Thing {
        thing,
        name,
        lat,
        lng,
        count,
        description,
        image,
        ..
    } = payload;

    sqlx::query!(
        r#"
        INSERT INTO "things"(id,thing_type,name,lat,lng,count,description,timestamp,image)
        VALUES ($1,$2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        thing_id,
        thing as i32,
        name,
        lat,
        lng,
        count,
        description,
        thing_ts,
        image
    )
    .execute(&connection)
    .await
    .expect("error adding entry to DB");

    Json("Sighting added successfully")
}

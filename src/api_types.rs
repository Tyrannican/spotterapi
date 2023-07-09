use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sighting {
    pub id: Option<String>,
    pub user_id: String,
    pub lat: f64,
    pub lng: f64,
    pub object: String,
    pub description: Option<String>,
    pub created_at: Option<i64>,
}

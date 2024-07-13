use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Serializable data to store response from GET requests
// TODO: Handle serialization for date time
#[derive(Serialize, Deserialize)]
pub struct DatabaseResponse {
    pub id: Uuid,
    pub table_no: i32,
    pub item: String,
    pub quantity: i32,
    pub preparation_time: i32,
    #[serde(skip_serializing, skip_deserializing)]
    pub placed_at: chrono::DateTime<Utc>,
}

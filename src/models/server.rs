use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub connection_url: String,
    pub arkservers_api_key: String,
    pub server_icon: String,
    pub server_bg: String,
    pub geolocation: String,
    pub is_online: bool,
    pub is_visible: bool,
    pub players: u32,
    pub max_players: u32,
    pub avg_players: u32,
}
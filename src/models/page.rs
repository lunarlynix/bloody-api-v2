use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub str_id: String,
    pub title: String,
    pub description: String,
    pub bg_image: String,
    pub content: String,
    pub page_icon: String,
    pub is_published: bool,
}
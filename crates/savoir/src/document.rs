use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "external_id")]
    pub id: String,
    pub name: String,
    pub content: String,
    pub url: Option<String>,
}

impl Document {
    pub fn uuid(&self) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_OID, self.id.as_bytes())
    }
}

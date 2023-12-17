use std::fmt::Debug;

use anyhow::Result;
use serde::Deserialize;

use crate::document::Document;

pub mod weaviate;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Config {
    Weaviate(weaviate::Config),
}

#[async_trait::async_trait]
pub trait DocumentStore: Debug + Send + Sync {
    async fn store(&self, document: &Document) -> Result<()>;
    async fn query(&self, query: &str) -> Result<Vec<Document>>;
}

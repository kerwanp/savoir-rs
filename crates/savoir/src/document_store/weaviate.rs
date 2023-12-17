use anyhow::Result;
use serde::Deserialize;
use thiserror::Error;
use weaviate_community::{
    collections::{objects::Object, query::GetQuery},
    WeaviateClient,
};

use crate::document::Document;

use super::DocumentStore;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot create weaviate client: {0}")]
    CreateClient(String),
    #[error("cannot create weaviate document: {0}")]
    CreateDocument(String),
    #[error("cannot create weaviate document: {0}")]
    UpdateDocument(String),
    #[error("cannot create weviate document: {0}")]
    QueryDocument(String),
}

#[derive(Deserialize, Debug)]
pub struct Config {
    host: String,
}

impl TryFrom<Config> for WeaviateClient {
    type Error = Error;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let client = WeaviateClient::builder(&value.host)
            .build()
            .map_err(|e| Error::CreateClient(e.to_string()))?;

        Ok(client)
    }
}

const CLASS_NAME: &str = "Document";

#[async_trait::async_trait]
impl DocumentStore for WeaviateClient {
    async fn store(&self, document: &Document) -> Result<()> {
        let exists = self
            .objects
            .exists(CLASS_NAME, &document.uuid(), None, None)
            .await
            .unwrap_or(false);

        let value = serde_json::to_value(document)?;

        if exists {
            let obj = Object::builder(CLASS_NAME, value).build();

            self.objects
                .create(&obj, None)
                .await
                .map_err(|e| Error::CreateDocument(e.to_string()))?;
        } else {
            self.objects
                .update(&value, CLASS_NAME, &document.uuid(), None)
                .await
                .map_err(|e| Error::UpdateDocument(e.to_string()))?;
        }

        Ok(())
    }

    async fn query(&self, query: &str) -> Result<Vec<Document>> {
        let query = GetQuery::builder(CLASS_NAME, vec!["external_id", "name", "url", "content"])
            .with_limit(5)
            .with_near_text(&format!("{{ concepts: [\"{query}\"] }}"))
            .build();

        let res = self
            .query
            .get(query)
            .await
            .map_err(|e| Error::QueryDocument(e.to_string()))?;

        let data = res
            .get("data")
            .unwrap()
            .get("Get")
            .unwrap()
            .get("Document")
            .unwrap()
            .to_owned();

        let documents: Vec<Document> = serde_json::from_value(data)?;

        Ok(documents)
    }
}

use std::fmt::Debug;

use anyhow::Error;
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

use crate::{document::Document, interals::AsyncTryFrom};

use self::google::GoogleDatasource;

pub mod google;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Config {
    Google(google::Config),
}

#[async_trait::async_trait]
pub trait Datasource: Send + Sync + Debug {
    async fn stream_documents(&self, tx: Sender<Document>);
}

#[async_trait::async_trait]
impl AsyncTryFrom<Config> for Box<dyn Datasource> {
    type Error = Error;
    async fn async_try_from(value: Config) -> Result<Self, Error> {
        let datasource = match value {
            Config::Google(config) => Box::new(GoogleDatasource::async_try_from(config).await?),
        };

        Ok(datasource)
    }
}

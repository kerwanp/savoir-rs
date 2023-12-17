use anyhow::Error;
use futures_util::StreamExt;
use google_drive3::{
    hyper::{self, body},
    hyper_rustls, oauth2, DriveHub,
};
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

use crate::{document::Document, interals::AsyncTryFrom};

use super::Datasource;

#[derive(Deserialize, Debug)]
pub struct Config {
    service_account: String,
    subject: Option<String>,
}

pub struct GoogleDatasource {
    client: DriveHub<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    // config: Config,
}

impl std::fmt::Debug for GoogleDatasource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoogleDatasource").finish()
    }
}

#[async_trait::async_trait]
impl AsyncTryFrom<Config> for GoogleDatasource {
    type Error = Error;

    async fn async_try_from(value: Config) -> Result<Self, Self::Error> {
        let service_account = oauth2::read_service_account_key(value.service_account)
            .await
            .unwrap();

        let mut auth = oauth2::ServiceAccountAuthenticator::builder(service_account);
        if let Some(subject) = value.subject {
            auth = auth.subject(subject);
        }

        let auth = auth.build().await.unwrap();

        let client = DriveHub::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );
        Ok(Self { client })
    }
}

impl GoogleDatasource {
    async fn export(&self, id: &str) -> String {
        let data = self
            .client
            .files()
            .export(id, "text/plain")
            .doit()
            .await
            .unwrap();

        let bytes = body::to_bytes(data.into_body()).await.unwrap();
        String::from_utf8(bytes.into_iter().collect()).unwrap()
    }
}

#[async_trait::async_trait]
impl Datasource for GoogleDatasource {
    async fn stream_documents(&self, tx: Sender<Document>) {
        let (_, res) = self
            .client
            .files()
            .list()
            .corpora("allDrives")
            .supports_all_drives(true)
            .include_items_from_all_drives(true)
            .q("mimeType = 'application/vnd.google-apps.document'")
            .doit()
            .await
            .unwrap();

        tokio_stream::iter(res.files.unwrap())
            .for_each_concurrent(8, |f| async {
                let id = f.id.unwrap();
                let content = self.export(&id).await;
                let _ = tx
                    .send(Document {
                        id,
                        name: f.name.unwrap(),
                        content,
                        url: None,
                    })
                    .await;
            })
            .await
    }
}

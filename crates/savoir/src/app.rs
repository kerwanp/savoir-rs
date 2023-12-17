use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use log::{error, info};
use serde::Deserialize;
use tokio::sync::{mpsc, Mutex};
use weaviate_community::WeaviateClient;

use crate::{
    agent,
    conversation::Conversation,
    conversation_store::{in_memory::InMemoryConversationStore, ConversationStore},
    datasource::{self, Datasource},
    document::Document,
    document_store::{self, DocumentStore},
    integration::{self, Integration},
    interals::AsyncTryFrom,
    llm::{self, Llm},
    message::{self, Message},
};

#[derive(Deserialize, Debug)]
pub struct Config {
    datasources: HashMap<String, datasource::Config>,
    llms: HashMap<String, llm::Config>,
    store: document_store::Config,
    agents: HashMap<String, agent::Config>,
    integrations: HashMap<String, integration::Config>,
}

#[derive(Debug)]
pub struct App {
    document_store: Box<dyn DocumentStore>,
    datasources: HashMap<String, Arc<Box<dyn Datasource>>>,
    llms: HashMap<String, Arc<Box<dyn Llm>>>,
    agents: HashMap<String, agent::Config>,
    integrations: HashMap<String, integration::Config>,
    conversation_store: Arc<Mutex<Box<dyn ConversationStore>>>,
}

#[async_trait::async_trait]
impl AsyncTryFrom<Config> for App {
    type Error = anyhow::Error;

    async fn async_try_from(value: Config) -> Result<Self, Self::Error> {
        let mut datasources: HashMap<String, Arc<Box<dyn Datasource>>> = HashMap::new();
        let mut llms: HashMap<String, Arc<Box<dyn Llm>>> = HashMap::new();

        let document_store: Box<dyn DocumentStore> = match value.store {
            document_store::Config::Weaviate(config) => Box::new(WeaviateClient::try_from(config)?),
        };

        for (name, config) in value.datasources {
            let datasource: Box<dyn Datasource> = Box::async_try_from(config).await?;
            datasources.insert(name, Arc::new(datasource));
        }

        for (name, config) in value.llms {
            let llm: Box<dyn Llm> = Box::from(config);
            llms.insert(name, Arc::new(llm));
        }

        Ok(Self {
            document_store,
            datasources,
            llms,
            agents: value.agents,
            integrations: value.integrations,
            conversation_store: Arc::new(Mutex::new(Box::<InMemoryConversationStore>::default())),
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("The {0} {1} does not exist in the configuration")]
    ResourceNotFound(String, String),
}

impl App {
    pub fn datasource(&self, name: &str) -> Result<&Arc<Box<dyn Datasource>>> {
        let datasource = self.datasources.get(name).ok_or(Error::ResourceNotFound(
            "datasource".to_string(),
            name.to_string(),
        ))?;

        Ok(datasource)
    }

    pub fn agent(&self, name: &str) -> Result<&agent::Config> {
        let agent = self.agents.get(name).ok_or(Error::ResourceNotFound(
            "agent".to_string(),
            name.to_string(),
        ))?;

        Ok(agent)
    }

    pub fn integration(&self, name: &str) -> Result<&integration::Config> {
        let integration = self.integrations.get(name).ok_or(Error::ResourceNotFound(
            "integration".to_string(),
            name.to_string(),
        ))?;

        Ok(integration)
    }

    pub fn llm(&self, name: &str) -> Result<&Arc<Box<dyn Llm>>> {
        let llm = self
            .llms
            .get(name)
            .ok_or(Error::ResourceNotFound("llm".to_string(), name.to_string()))?;

        Ok(llm)
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Document>> {
        self.document_store.query(query).await
    }

    // TODO: Split & clean that
    pub async fn ask(&self, agent: &str, conversation_id: &str, query: &str) -> Result<String> {
        let agent = self.agent(agent)?;
        let llm = self.llm(&agent.llm)?;

        let documents = self.document_store.query(query).await?;

        info!("Found {} documents", documents.len());

        let documents = serde_json::to_string(&documents)?;

        let mut store = self.conversation_store.lock().await;
        let conversation = store.get_mut(conversation_id);

        let conversation = match conversation {
            Some(conversation) => conversation,
            None => {
                let conv = Conversation(vec![Message::new(
                    message::Role::System,
                    &format!("{}\n{}", &agent.prompt, documents),
                )]);
                store.create(conversation_id, conv)
            }
        };

        conversation
            .0
            .push(Message::new(message::Role::User, query));

        let res = llm.chat(conversation.clone()).await?;

        conversation
            .0
            .push(Message::new(message::Role::Assistant, query));

        Ok(res)
    }

    pub async fn synchronize(&self, name: &str) -> Result<()> {
        info!("Synchronizing datasource {name}");
        let datasource = self.datasource(name)?.clone();
        let (tx, mut rx) = mpsc::channel(32);

        tokio::spawn(async move {
            datasource.stream_documents(tx).await;
        });

        while let Some(document) = rx.recv().await {
            info!("Synchronizing document {name}:{}", &document.id);
            let res = self.document_store.store(&document).await;
            if let Err(e) = res {
                error!(
                    "Error while synchronizing document '{}': {}",
                    document.id,
                    e.to_string()
                );
            }
        }

        Ok(())
    }

    pub async fn run_integration(self, name: &str) -> Result<()> {
        let integration = self.integration(name)?;
        let integration: Box<dyn Integration> = Box::try_from(integration.clone()).unwrap();
        integration.serve(self).await
    }
}

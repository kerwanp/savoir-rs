use anyhow::Result;
use serde::Deserialize;
use std::fmt::Debug;

use crate::conversation::Conversation;

use self::openai::OpenAi;

pub mod openai;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Config {
    #[serde(rename = "openai")]
    OpenAi(openai::Config),
}

#[async_trait::async_trait]
pub trait Llm: Send + Sync + Debug {
    async fn chat(&self, conversation: Conversation) -> Result<String>;
}

impl From<Config> for Box<dyn Llm> {
    fn from(value: Config) -> Self {
        match value {
            Config::OpenAi(config) => Box::new(OpenAi::from(config)),
        }
    }
}

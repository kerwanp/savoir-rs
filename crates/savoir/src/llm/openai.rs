use anyhow::{anyhow, Result};
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{self, ChatCompletionMessage, ChatCompletionRequest, MessageRole},
    common::GPT3_5_TURBO_1106,
};
use serde::Deserialize;

use crate::message::{Message, Role};

use super::Llm;

#[derive(Deserialize, Debug)]
pub struct Config {
    api_key: String,
    model: String,
}

pub struct OpenAi {
    client: Client,
    model: String,
}

impl std::fmt::Debug for OpenAi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAi").finish()
    }
}

impl From<Config> for OpenAi {
    fn from(value: Config) -> Self {
        Self {
            client: Client::new(value.api_key),
            model: value.model,
        }
    }
}

#[async_trait::async_trait]
impl Llm for OpenAi {
    async fn chat(&self, messages: Vec<Message>) -> Result<String> {
        let req = ChatCompletionRequest::new(
            self.model.clone(),
            messages
                .into_iter()
                .map(ChatCompletionMessage::from)
                .collect(),
        );

        let result = self.client.chat_completion(req)?;
        let content = result.choices[0].message.content.clone();

        content.ok_or(anyhow!("No completion found"))
    }
}

impl From<Message> for ChatCompletionMessage {
    fn from(value: Message) -> Self {
        Self {
            role: match value.role {
                Role::User => MessageRole::user,
                Role::System => MessageRole::system,
            },
            content: value.content,
            function_call: None,
            name: None,
        }
    }
}

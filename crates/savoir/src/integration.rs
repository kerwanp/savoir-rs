pub mod slack;

use anyhow::Result;
use serde::Deserialize;
use std::fmt::Debug;

use crate::app::App;

use self::slack::SlackIntegration;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Config {
    Slack(slack::Config),
}

#[async_trait::async_trait]
pub trait Integration: Debug {
    async fn serve(&self, app: App) -> Result<()>;
}

impl TryFrom<Config> for Box<dyn Integration> {
    type Error = anyhow::Error;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let integration: Box<dyn Integration> = match value {
            Config::Slack(config) => Box::new(SlackIntegration::try_from(config)?),
        };

        Ok(integration)
    }
}

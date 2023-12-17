use std::sync::Arc;

use anyhow::Result;
use log::{error, warn};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::app::App;

use super::Integration;
use axum::{extract::State, Extension};
use slack_morphism::prelude::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    signing_secret: String,
    #[serde(default)]
    port: Option<u16>,
    agent: String,
}

pub struct SlackIntegration {
    pub config: Config,
}

impl std::fmt::Debug for SlackIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slack").finish()
    }
}

impl TryFrom<Config> for SlackIntegration {
    type Error = anyhow::Error;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        Ok(Self { config: value })
    }
}

fn error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> HttpStatusCode {
    println!("{:#?}", err);

    // Defines what we return Slack server
    HttpStatusCode::BAD_REQUEST
}

async fn command_event_handler(
    Extension(environment): Extension<Arc<SlackHyperListenerEnvironment>>,
    Extension(event): Extension<SlackCommandEvent>,
    State(state): State<Arc<MyState>>,
) -> axum::Json<Value> {
    println!("Received command event: {:?}", event);

    let response_url = event.response_url;

    match event.command {
        SlackCommandId(cmd) if &cmd == "/ask" => {
            let text = event.text.unwrap();
            let SlackChannelId(channel_id) = event.channel_id;

            tokio::spawn(async move {
                let response = state
                    .app
                    .ask(&state.config.agent, &channel_id, &text)
                    .await
                    .unwrap();
                let req = SlackApiPostWebhookMessageRequest::new(
                    SlackMessageContent::new().with_text(response),
                );
                let res = environment
                    .client
                    .respond_to_event(&response_url.clone(), &req)
                    .await;

                if let Err(err) = res {
                    error!("Something went wrong when responding to event {err}");
                }
            });
        }
        SlackCommandId(cmd) => warn!("Command {cmd} not handled"),
    };

    axum::Json(json! {{ "text": "Loading..." }})
}

struct MyState {
    app: App,
    config: Config,
}

#[async_trait::async_trait]
impl Integration for SlackIntegration {
    async fn serve(&self, app: App) -> Result<()> {
        let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.config.port.unwrap_or(8080)));

        let state = MyState {
            app,
            config: self.config.clone(),
        };

        let signing_secret: SlackSigningSecret = self.config.signing_secret.clone().into();

        let listener_environment: Arc<SlackHyperListenerEnvironment> = Arc::new(
            SlackClientEventsListenerEnvironment::new(client.clone())
                .with_error_handler(error_handler),
        );

        let listener: SlackEventsAxumListener<SlackHyperHttpsConnector> =
            SlackEventsAxumListener::new(listener_environment.clone());

        let app = axum::routing::Router::new()
            .route(
                "/command",
                axum::routing::post(command_event_handler).layer(
                    listener
                        .events_layer(&signing_secret)
                        .with_event_extractor(SlackEventsExtractors::command_event()),
                ),
            )
            .with_state(Arc::new(state));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}

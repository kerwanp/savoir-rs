use std::fs;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use log::LevelFilter;
use savoir::{app::App, integration, interals::AsyncTryFrom};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Synchronize { datasource: String },
    Ask { agent: String, query: String },
    Search { query: String },
    Serve { integration: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = fs::read_to_string("savoir.yaml").unwrap();
    let config: savoir::app::Config = serde_yaml::from_str(&config).unwrap();

    let app = App::async_try_from(config).await?;

    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .try_init()?;

    let cli = Cli::parse();

    match cli.command {
        Command::Synchronize { datasource } => app.synchronize(&datasource).await,
        Command::Search { query } => match app.query(&query).await {
            Ok(documents) => {
                println!("---------------------");
                for document in documents {
                    println!("Name: {}", document.name);
                    println!("---------------------");
                }

                Ok(())
            }
            Err(e) => Err(anyhow!("Something wrong happened {e}")),
        },
        Command::Ask { agent, query } => match app.ask(&agent, &query).await {
            Ok(res) => {
                println!("Answer: {res}");
                Ok(())
            }
            Err(e) => Err(e),
        },
        Command::Serve { integration } => app.run_integration(&integration).await,
    }
}

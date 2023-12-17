use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub llm: String,
    pub prompt: String,
}

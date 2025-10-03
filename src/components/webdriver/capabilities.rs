use super::import::*;
use super::prelude::*;

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Default, Clone)]
#[transporter(async fn trans() {
    transporter(&message).await;
})]
pub struct Capabilities {
    pub browser_version: String,
    pub accept_insecure_certs: bool,
    pub page_load_strategy: String,
    pub unhandled_prompt_behavior: String,
    #[set(value = Args::new())]
    pub args: Args,
}

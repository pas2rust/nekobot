use super::prelude::*;
use kenzu::Builder;
use nekoprint::NekoPrint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Clone, Default)]
#[transporter(async fn trans() {
    transporter(&message).await;
})]
pub struct NekoBot {
    #[set(value=WebDriver::new())]
    pub web_driver: WebDriver,
}

impl NekoBot {
    pub fn firefox() -> Self {
        Self::new().web_driver(WebDriver::new().browser(Browser::Firefox))
    }

    pub fn chrome() -> Self {
        Self::new()
    }
}
